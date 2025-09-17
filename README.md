# AI DEX (WEB3-DEX-SOLANA-OPENSOURCE)

Branche: feature/full-create-token-flow

Ce dépôt propose un scaffold pour AI DEX — un prototype de DEX sur Solana qui inclut :
- contracts/ : Anchor program skeleton (config, token record, squelettes swap/liquidity)
- frontend/ : React + Tailwind pages pour Swap, Create Token, Explorer
- backend/ : Node.js/Express API pour price lookup, verify-fee, create-token (stockage en mémoire pour le prototype)
- scripts/ : init_config.ts pour initialiser le compte config on-chain avec l'admin

Admin wallet (par défaut) : 38sMTofCQyAjPGC1eHqjykr9C1ZWv7RdqbxJ87z44b18

Notes importantes :
- Pour V1, la création effective du mint est recommandée côté client (wallet), afin d'éviter d'exposer une clé privée côté serveur.
- Le contrat Anchor contient la logique de configuration et l'enregistrement des tokens créés. Les fonctions AMM (swap, add_liquidity) sont des squelettes et doivent être complétées (pricing math, CPIs vers SPL Token, pool accounts).
- La vérification du paiement 5 USD se fait en backend via la route /api/verify-fee qui analyse la transaction et confirme que l'admin a bien reçu l'équivalent en SOL.
- Pour la production : remplacer le tokenStore en mémoire par une DB (Postgres), utiliser Pyth ou autre oracle on-chain pour prix SOL/USD si besoin, et ajouter des vérifications de sécurité.

Étapes pour tester localement :
1) contracts :
   - installer Anchor et Rust
   - anchor build
   - déployer le programme (anchor deploy) et mettre PROGRAM_ID en env
   - run `ts-node scripts/init_config.ts` pour initialiser la config avec ADMIN_WALLET

2) backend :
   - cd backend && npm install
   - cp .env.example .env et ajuster
   - node index.js

3) frontend :
   - cd frontend && npm install
   - set REACT_APP_BACKEND_URL et REACT_APP_ADMIN_WALLET si nécessaire
   - npm start

Si tu veux, je peux maintenant :
- pousser ces fichiers dans la branche feature/full-create-token-flow et ouvrir une PR,
- ou mettre à jour d'autres occurrences (backend package.json, README plus détaillé, meta tags supplémentaires).