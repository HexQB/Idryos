# 🚀 Idryos - Privacy-Centric Decentralized Identity Platform

> **Idryos** est une plateforme de gestion d'identité ultra-privée, décentralisée, et open source.
> Elle permet aux utilisateurs de créer et gérer leurs identités **auto-hébergées** sans dépendre d'un tiers de confiance comme Google, Facebook ou Microsoft.

---

## 🌟 Objectif

Idryos a pour mission de **redonner le contrôle des identités numériques aux utilisateurs** en proposant :

* Une solution **auto-hébergeable**.
* Une gestion d'identité **sans centralisation**.
* Des **tokens de connexion temporaires** et auto-régénérés.
* Une **portabilité des identités** entre services compatibles.
* Un système de connexion **sans pistage** ni collecte de données.

---

## 💡 Fonctionnalités principales

* ✅ Fournisseur **OAuth2 / OpenID Connect** ultra-léger.
* ✅ Création et gestion d’identités **auto-hébergées**.
* ✅ Support des **DID (Decentralized Identifiers)**.
* ✅ Tokens temporaires et rotation automatique.
* ✅ Communication décentralisée via **libp2p**.
* ✅ Stockage local (fichiers sécurisés ou SQLite) ou stockage P2P décentralisé (IPFS / blockchain privée).
* ✅ API simple et extensible.
* ✅ Frontend moderne et responsive (SvelteKit ou React).

---

## 🛠️ Stack technique

| Composant         | Technologie                         |
| ----------------- | ----------------------------------- |
| Backend principal | Rust                                |
| Microservices     | Go                                  |
| Protocole P2P     | libp2p                              |
| Authentification  | OAuth2, JWT, DID                    |
| Gateway API       | Traefik ou NGINX                    |
| Stockage          | SQLite, IPFS, Substrate (optionnel) |
| Frontend          | SvelteKit / React (TypeScript)      |

---

## 📦 Installation rapide (Docker)

```bash
git clone https://github.com/HexQB/idryos.git
cd idryos
docker-compose up -d
```

---

## ⚙️ Configuration

### Exemple `.env` minimal

```ini
# Auth Service
AUTH_SERVICE_PORT=8000
JWT_SECRET=ChangeMeSuperSecretKey
TOKEN_EXPIRATION_MINUTES=15

# Frontend
FRONTEND_URL=http://localhost:3000

# Storage
STORAGE_MODE=local # options: local | ipfs | blockchain
```

---

## 📚 Documentation

* 📖 [Guide d'installation](docs/installation.md)
* 🔑 [Configuration des identités](docs/identity.md)
* 🔐 [Fonctionnement OAuth2](docs/oauth2.md)
* 🌐 [Architecture réseau décentralisée](docs/network.md)

---

## 🔒 Sécurité

Idryos applique des principes stricts :

* Pas de stockage d’informations personnelles non chiffrées.
* Aucune collecte centralisée.
* Toutes les communications sont chiffrées de bout en bout.
* Clés privées générées et stockées **localement par l’utilisateur**.

---

## 🚤 Roadmap

* [x] API OAuth2 Rust auto-hébergeable
* [x] Authentification avec tokens temporaires
* [ ] Intégration libp2p pour découverte P2P
* [ ] Stockage IPFS optionnel
* [ ] Synchronisation multi-device
* [ ] Frontend de gestion d’identité complet

---

## 🤝 Contribution

Contributions bienvenues !
Voir [CONTRIBUTING.md](CONTRIBUTING.md) pour plus de détails.

---

## 📄 Licence

Ce projet est sous licence [MIT](LICENSE).

---

## ✨ Crédits

Projet développé par **HexQB**.
Une initiative open-source pour redonner aux utilisateurs le contrôle de leurs identités numériques.
