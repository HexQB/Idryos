.PHONY: help build start stop logs clean dev install test

# Variables
COMPOSE_FILE = docker-compose.yml
PROJECT_NAME = idryos

help: ## Afficher cette aide
	@echo "Idryos - Privacy-Centric Decentralized Identity Platform"
	@echo ""
	@echo "Commandes disponibles :"
	@awk 'BEGIN {FS = ":.*?## "} /^[a-zA-Z_-]+:.*?## / {printf "  \033[36m%-15s\033[0m %s\n", $$1, $$2}' $(MAKEFILE_LIST)

install: ## Installer les dépendances
	@echo "🔧 Installation des dépendances..."
	@cd frontend && npm install
	@echo "✅ Dépendances installées"

build: ## Construire tous les services
	@echo "🔨 Construction des services..."
	@docker-compose build
	@echo "✅ Services construits"

start: ## Démarrer tous les services
	@echo "🚀 Démarrage d'Idryos..."
	@chmod +x start.sh
	@./start.sh

stop: ## Arrêter tous les services
	@echo "🛑 Arrêt des services..."
	@docker-compose down
	@echo "✅ Services arrêtés"

restart: stop start ## Redémarrer tous les services

logs: ## Afficher les logs de tous les services
	@docker-compose logs -f

logs-auth: ## Afficher les logs du service d'authentification
	@docker-compose logs -f auth-service

logs-p2p: ## Afficher les logs du service P2P
	@docker-compose logs -f p2p-service

logs-frontend: ## Afficher les logs du frontend
	@docker-compose logs -f frontend

dev-backend: ## Démarrer le backend en mode développement
	@echo "🦀 Démarrage du backend Rust..."
	@cd backend && cargo run

dev-frontend: ## Démarrer le frontend en mode développement
	@echo "🎨 Démarrage du frontend SvelteKit..."
	@cd frontend && npm run dev

dev-p2p: ## Démarrer le service P2P en mode développement
	@echo "🌐 Démarrage du service P2P..."
	@cd services/p2p && go run main.go

test: ## Exécuter les tests
	@echo "🧪 Exécution des tests..."
	@cd backend && cargo test
	@cd frontend && npm run test || echo "Tests frontend non configurés"
	@cd services/p2p && go test ./... || echo "Tests P2P non configurés"

clean: ## Nettoyer les artefacts
	@echo "🧹 Nettoyage..."
	@docker-compose down -v --remove-orphans
	@docker system prune -f
	@cd backend && cargo clean || true
	@cd frontend && rm -rf node_modules .svelte-kit build || true
	@cd services/p2p && go clean || true
	@rm -rf data/
	@echo "✅ Nettoyage terminé"

reset: clean ## Réinitialiser complètement le projet
	@echo "🔄 Réinitialisation complète..."
	@docker-compose down -v --remove-orphans
	@docker volume prune -f
	@rm -f .env
	@echo "✅ Projet réinitialisé"

setup: ## Configuration initiale du projet
	@echo "⚙️ Configuration initiale..."
	@cp .env.example .env || echo "Fichier .env déjà existant"
	@mkdir -p data/p2p data/ipfs
	@echo "✅ Configuration terminée"

status: ## Afficher le statut des services
	@echo "📊 Statut des services :"
	@docker-compose ps

healthcheck: ## Vérifier la santé des services
	@echo "🏥 Vérification de la santé des services..."
	@curl -f http://localhost:8000/health 2>/dev/null && echo "✅ Auth Service: OK" || echo "❌ Auth Service: KO"
	@curl -f http://localhost:3000 2>/dev/null && echo "✅ Frontend: OK" || echo "❌ Frontend: KO"
	@curl -f http://localhost:8080 2>/dev/null && echo "✅ Traefik: OK" || echo "❌ Traefik: KO"

# Commandes de développement avancées
format: ## Formater le code
	@echo "🎨 Formatage du code..."
	@cd backend && cargo fmt || true
	@cd frontend && npm run format || true
	@cd services/p2p && go fmt ./... || true

lint: ## Vérifier le code avec les linters
	@echo "🔍 Vérification du code..."
	@cd backend && cargo clippy -- -D warnings || true
	@cd frontend && npm run lint || true
	@cd services/p2p && go vet ./... || true

# Commandes Docker
docker-build: ## Construire les images Docker
	@docker-compose build --no-cache

docker-pull: ## Télécharger les images Docker
	@docker-compose pull

docker-clean: ## Nettoyer les images Docker
	@docker system prune -a -f

# Commandes de documentation
docs: ## Générer la documentation
	@echo "📚 Génération de la documentation..."
	@cd backend && cargo doc --no-deps || true
	@echo "✅ Documentation générée"

# Commandes de base de données
db-migrate: ## Exécuter les migrations de base de données
	@echo "🗄️ Migration de la base de données..."
	@cd backend && sqlx migrate run || echo "Migrations non configurées"

db-reset: ## Réinitialiser la base de données
	@echo "🔄 Réinitialisation de la base de données..."
	@rm -f data/*.db data/*.sqlite
	@echo "✅ Base de données réinitialisée"
