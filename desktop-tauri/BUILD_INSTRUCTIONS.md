# Guide de compilation Tauri pour Windows

## Étapes complètes pour créer un .exe exécutable

### 1. Installer les prérequis

#### Rust
```bash
# Télécharger depuis https://rustup.rs/ 
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

#### Visual C++ Build Tools (Windows)
- Télécharger depuis: https://visualstudio.microsoft.com/visual-cpp-build-tools/
- Ou installer Visual Studio Community avec "Desktop development with C++"

#### Node.js
- Version 18+ depuis https://nodejs.org/

### 2. Préparer les fichiers

```bash
cd /home/abigail/development/Translate/desktop-tauri

# Installer les dépendances npm
npm install

# Compiler le serveur Node.js en .exe
npm run build:server
```

**Note**: `pkg` peut prendre du temps la première fois (~1-2 minutes).

### 3. Développement

```bash
# Lancer en mode développement
npm run dev
```

Cela:
- Lance le frontend Vite sur `http://localhost:5173`
- Lance l'application Tauri
- Intègre le serveur Node.js

### 4. Build de production

```bash
# Compiler pour Windows 64-bit
npm run build:windows
```

**Résultat**: 
- Exécutable: `src-tauri/target/release/live-interpretation.exe`
- Installateurs:
  - `src-tauri/target/release/bundle/nsis/LiveInterpretation_1.0.0_x64_en-US.exe` (NSIS)
  - `src-tauri/target/release/bundle/msi/LiveInterpretation_1.0.0_x64.msi` (MSI)

### 5. Distribution

L'exécutable généré est **portable** et **à distribution autonome**:
- ✅ Pas de dépendance Rust
- ✅ Pas de dépendance Node.js
- ✅ Serveur intégré
- ✅ Certificats HTTPS inclus

## Architecture

```
Application Tauri (Rust)
    ├── Frontend (Vite + HTML/CSS/JS)
    │   └── Interface Web
    └── Sidecar: Serveur Node.js
        ├── Express
        ├── Socket.IO
        └── WebRTC Signaling
```

## Troubleshooting

### "Rust not found"
```bash
rustup update
rustc --version  # Doit afficher la version
```

### Erreur de compilation MSVC (Windows)
- Installer Visual Studio Build Tools
- Ou installer Visual Studio Community
- Redémarrer après l'installation

### Port 3000 déjà utilisé
```bash
# Linux/macOS
lsof -i :3000 | grep LISTEN | awk '{print $2}' | xargs kill -9

# Windows PowerShell
netstat -ano | findstr :3000
taskkill /PID <PID> /F
```

### "pkg not found"
```bash
npm install -g pkg@5
```

## Fichiers générés au build

```
desktop-tauri/
├── dist/                     # Frontend bundlé
├── node_modules/
├── src-tauri/
│   ├── target/
│   │   └── release/          # Exécutables Rust
│   │       ├── live-interpretation.exe
│   │       └── bundle/       # Installateurs
│   └── resources/
│       └── server/           # Serveur Node.js compilé
│           └── server.exe
```

## Certificats HTTPS

Les certificats sont générés et stockés dans:
```
live-interpretation-node/certs/
├── server-cert.pem
└── server-key.pem
```

Ils sont inclus dans le build final et copiés automatiquement.
