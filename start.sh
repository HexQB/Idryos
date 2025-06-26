#!/bin/bash

# Script de démarrage rapide pour Idryos
set -e

echo "🚀 Démarrage d'Idryos..."

# Vérifier si Docker est installé
if ! command -v docker &> /dev/null; then
    echo "❌ Docker n'est pas installé. Veuillez l'installer avant de continuer."
    exit 1
fi

if ! command -v docker-compose &> /dev/null; then
    echo "❌ Docker Compose n'est pas installé. Veuillez l'installer avant de continuer."
    exit 1
fi

# Vérifier si sudo est nécessaire pour Docker
if ! docker ps &> /dev/null; then
    echo "🔑 Utilisation de sudo pour Docker..."
    DOCKER_CMD="sudo docker-compose"
else
    DOCKER_CMD="docker-compose"
fi

# Créer le fichier .env s'il n'existe pas
if [ ! -f .env ]; then
    echo "📝 Création du fichier .env..."
    cp .env.example .env
    
    # Générer un JWT secret aléatoirement
    JWT_SECRET=$(openssl rand -base64 32 2>/dev/null || echo "ChangeMeSuperSecretKey$(date +%s)")
    sed -i "s/JWT_SECRET=ChangeMeSuperSecretKey/JWT_SECRET=$JWT_SECRET/" .env
    
    echo "✅ Fichier .env créé avec un JWT secret généré aléatoirement"
fi

# Créer les dossiers de données
echo "📁 Création des dossiers de données..."
mkdir -p data/p2p data/ipfs

# Construire et démarrer les services
echo "🔨 Construction et démarrage des services..."
$DOCKER_CMD up -d --build

echo "⏳ Attente du démarrage des services..."
sleep 10

# Vérifier l'état des services
echo "🔍 Vérification de l'état des services..."

# Vérifier le service d'authentification
if curl -s http://localhost:8000/health > /dev/null; then
    echo "✅ Service d'authentification : OK"
else
    echo "⚠️  Service d'authentification : En cours de démarrage..."
fi

# Vérifier le frontend
if curl -s http://localhost:3000 > /dev/null; then
    echo "✅ Frontend : OK"
else
    echo "⚠️  Frontend : En cours de démarrage..."
fi

echo ""
echo "🎉 Idryos est maintenant disponible !"
echo ""
echo "📱 Interface utilisateur : http://localhost:3000"
echo "🔧 API d'authentification : http://localhost:8000"
echo "🌐 Traefik Dashboard : http://localhost:8080"
echo ""
echo "Pour arrêter les services : $DOCKER_CMD down"
echo "Pour voir les logs : $DOCKER_CMD logs -f"
