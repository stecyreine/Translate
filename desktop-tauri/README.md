# Live Interpretation - Application Tauri Desktop

Application de diffusion audio en direct sur réseau local, compilée en exécutable Windows natif avec Tauri.

## 🎯 Objectif

Créer une application **portable**, **exécutable standalone** pour Windows qui:
- ✅ Fonctionne sans dépendances externes
- ✅ S'installe comme une application Windows classique
- ✅ Inclut le serveur Node.js intégré
- ✅ Gère les certificats HTTPS automatiquement

## 📁 Structure

```
desktop-tauri/
├── index.html                     # Point d'entrée frontend
├── package.json                   # Configuration npm
├── vite.config.js                 # Configuration Vite
├── tsconfig.json                  # Configuration TypeScript
├── prepare-server.js              # Script de compilation serveur
├── build-server.bat               # Batch pour compilation Windows
├── build-server.sh                # Shell pour compilation Linux
├── BUILD_INSTRUCTIONS.md          # Guide détaillé de compilation
├── SETUP_TAURI.md                # Configuration Tauri
├── .env                          # Variables d'environnement
│
├── src-tauri/                    # Code Rust Tauri
│   ├── Cargo.toml               # Dépendances Rust
│   ├── tauri.conf.json          # Configuration Tauri
│   ├── build.rs                 # Script de build
│   ├── .cargo/
│   │   └── config.toml          # Config optimisation Rust
│   ├── src/
│   │   └── main.rs              # Point d'entrée Rust
│   ├── icons/                   # Icônes de l'app
│   └── resources/
│       ├── live-app/            # Fichiers web existants
│       └── server/              # Serveur Node.js compilé
│
└── dist/                        # Sortie du build frontend (gitignored)
```

## 🚀 Démarrage rapide

### Installation initiale

```bash
cd desktop-tauri

# 1. Installer les prérequis Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 2. Installer Node.js 18+
# https://nodejs.org/

# 3. Installer les dépendances npm
npm install

# 4. Compiler le serveur Node.js
npm run build:server
```

### Développement

```bash
npm run dev
```

### Build pour Windows

```bash
npm run build:windows
```

## 📦 Fichiers générés

### Développement
- `dist/` - CAssembrage frontend Vite
- `src-tauri/target/debug/` - Version debug Rust

### Production (après `npm run build:windows`)
- **Exécutable**: `src-tauri/target/release/live-interpretation.exe`
- **Installateurs**:
  - NSIS: `src-tauri/target/release/bundle/nsis/LiveInterpretation_1.0.0_x64_en-US.exe`
  - MSI: `src-tauri/target/release/bundle/msi/LiveInterpretation_1.0.0_x64.msi`

## 🔧 Configuration

- **Frontend**: Vite + HTML/CSS/JS vanilla
- **Backend**: Tauri + Rust
- **Serveur**: Node.js (Express + Socket.IO + WebRTC)
- **HTTPS**: Certificats autosignés inclus

## 📝 Notes

- L'exécutable final n'a **aucune dépendance** (standalone)
- Le serveur Node.js démarre **automatiquement**
- Les certificats HTTPS permettent le micro sur réseau local
- Optimisé pour **Windows 64-bit**

## 📖 Documentation

- [BUILD_INSTRUCTIONS.md](BUILD_INSTRUCTIONS.md) - Guide complet de compilation
- [SETUP_TAURI.md](SETUP_TAURI.md) - Prérequis et installation
- [../live-interpretation-node/README.md](../live-interpretation-node/README.md) - Détails du serveur

## 🎨 Icônes

Placer les icônes dans `src-tauri/icons/`:
- `icon.ico` - Icône Windows 32x32, 128x128
- `32x32.png`, `128x128.png`, `128x128@2x.png` - Icônes PNG

## 🔗 URLs

- **Application**: http://localhost:5173 (développement)
- **Serveur HTTPS**: https://localhost:3000
- **API Health**: https://localhost:3000/health

## 📋 Checklist de déploiement

- [ ] Rust installé (`rustc --version`)
- [ ] Visual C++ Build Tools (Windows)
- [ ] Node.js 18+ (`node --version`)
- [ ] Dépendances: `npm install`
- [ ] Serveur compilé: `npm run build:server`
- [ ] Test développement: `npm run dev`
- [ ] Build production: `npm run build:windows`
- [ ] Exécutable généré dans `src-tauri/target/release/`
