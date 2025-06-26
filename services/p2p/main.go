package main

import (
	"context"
	"encoding/json"
	"fmt"
	"log"
	"os"
	"os/signal"
	"syscall"
	"time"

	"github.com/libp2p/go-libp2p"
	"github.com/libp2p/go-libp2p/core/host"
	"github.com/libp2p/go-libp2p/core/peer"
	"github.com/libp2p/go-libp2p/p2p/discovery/mdns"
	"github.com/libp2p/go-libp2p/p2p/net/connmgr"
)

type P2PService struct {
	host   host.Host
	ctx    context.Context
	cancel context.CancelFunc
}

type Message struct {
	Type      string                 `json:"type"`
	Timestamp time.Time              `json:"timestamp"`
	Data      map[string]interface{} `json:"data"`
	From      string                 `json:"from"`
}

func NewP2PService() (*P2PService, error) {
	ctx, cancel := context.WithCancel(context.Background())

	// Connection manager
	connmgr, err := connmgr.NewConnManager(10, 100, connmgr.WithGracePeriod(time.Minute))
	if err != nil {
		cancel()
		return nil, fmt.Errorf("failed to create connection manager: %w", err)
	}

	// Create a new libp2p Host
	host, err := libp2p.New(
		libp2p.ListenAddrStrings("/ip4/0.0.0.0/tcp/4001"),
		libp2p.ConnectionManager(connmgr),
	)
	if err != nil {
		cancel()
		return nil, fmt.Errorf("failed to create libp2p host: %w", err)
	}

	service := &P2PService{
		host:   host,
		ctx:    ctx,
		cancel: cancel,
	}

	// Setup mDNS discovery
	if err := service.setupDiscovery(); err != nil {
		service.Close()
		return nil, fmt.Errorf("failed to setup discovery: %w", err)
	}

	return service, nil
}

func (p *P2PService) setupDiscovery() error {
	// Setup mDNS discovery service
	service := mdns.NewMdnsService(p.host, "idryos", &discoveryNotifee{host: p.host})
	return service.Start()
}

func (p *P2PService) GetPeerInfo() peer.AddrInfo {
	return peer.AddrInfo{
		ID:    p.host.ID(),
		Addrs: p.host.Addrs(),
	}
}

func (p *P2PService) BroadcastMessage(msgType string, data map[string]interface{}) error {
	msg := Message{
		Type:      msgType,
		Timestamp: time.Now(),
		Data:      data,
		From:      p.host.ID().String(),
	}

	msgBytes, err := json.Marshal(msg)
	if err != nil {
		return fmt.Errorf("failed to marshal message: %w", err)
	}

	// Broadcast to all connected peers
	for _, peerID := range p.host.Network().Peers() {
		if stream, err := p.host.NewStream(p.ctx, peerID, "/idryos/message/1.0.0"); err == nil {
			stream.Write(msgBytes)
			stream.Close()
		}
	}

	return nil
}

func (p *P2PService) Close() error {
	p.cancel()
	return p.host.Close()
}

// discoveryNotifee gets notified when we find a new peer via mDNS discovery
type discoveryNotifee struct {
	host host.Host
}

func (n *discoveryNotifee) HandlePeerFound(pi peer.AddrInfo) {
	log.Printf("discovered new peer %s", pi.ID.String())
	err := n.host.Connect(context.Background(), pi)
	if err != nil {
		log.Printf("error connecting to peer %s: %s", pi.ID.String(), err)
	} else {
		log.Printf("connected to peer %s", pi.ID.String())
	}
}

func main() {
	log.Println("Starting Idryos P2P Service...")

	service, err := NewP2PService()
	if err != nil {
		log.Fatalf("Failed to create P2P service: %v", err)
	}
	defer service.Close()

	peerInfo := service.GetPeerInfo()
	log.Printf("P2P node started with ID: %s", peerInfo.ID.String())
	log.Printf("Listening addresses:")
	for _, addr := range peerInfo.Addrs {
		log.Printf("  %s/p2p/%s", addr, peerInfo.ID.String())
	}

	// Wait for interrupt signal to gracefully shutdown
	c := make(chan os.Signal, 1)
	signal.Notify(c, os.Interrupt, syscall.SIGTERM)

	// Block until we receive our signal
	<-c
	log.Println("Shutting down P2P service...")
}
