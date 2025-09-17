# xest-ai-dex

xest-ai-dex est un DEX (version 1) sur Solana avec intégration AI prévue pour vérification automatique et analyses.

Structure du dépôt

- contracts/ : Programme Anchor (Rust) — logique minimale pour pools, swap, lock
- frontend/  : dApp React + Tailwind (Phantom, Solflare)
- backend/   : Node.js + Express pour la logique off-chain (paiement des frais, enregistrement des tokens)

Nom du projet : xest-ai-dex
Réseau de développement recommandé : devnet

Configuration requise avant déploiement

- ADMIN_WALLET : adresse publique Solana qui recevra les frais de création et la part des fees de swap.
- NETWORK : URL RPC Solana (par défaut https://api.devnet.solana.com)
- ANCHOR_PROVIDER_URL et ANCHOR_WALLET pour la compilation / déploiement Anchor
- PROGRAM_ID : remplacer le placeholder dans contracts/Anchor.toml après déploiement

Mode de création de token (recommandé)

- Mode A (recommandé) : l'utilisateur signe la transaction de création du mint et paie les frais (backend vérifie et enregistre). Pas de clé privée stockée côté serveur.
- Mode B : backend/deployer crée les mints (nécessite stockage sécurisé de la clé privée du deployer)

Frais

- Frais de création des tokens : 5 USD (en SOL ou USDC). Conversion USD->SOL via oracle (Pyth) ou Coingecko en fallback.
- Frais de swap : 0.3% par trade. Répartition configurable (par défaut : 50% au pool, 50% à ADMIN_WALLET).

Sécurité

- Pipeline de vérification : cargo-audit, tests unitaires, linter. Option d'analyse AI (service externe) pour repérer patterns dangereux.
- Lock de liquidité : option pour verrouiller LP pour une durée donnée.

Déploiement rapide

1. Configurez ADMIN_WALLET et NETWORK dans le backend (fichier .env ou variables d'environnement).
2. Déployer le programme Anchor : mettre PROGRAM_ID dans contracts/Anchor.toml.
3. Lancer backend : cd backend && npm install && npm start
4. Lancer frontend : cd frontend && npm install && npm run dev
