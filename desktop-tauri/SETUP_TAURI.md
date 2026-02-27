# Tauri - Application Desktop Windows

## Prérequis

### 1. Installer Rust
```bash
# Télécharger depuis https://rustup.rs/
# Ou exécuter
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### 2. Installer Node.js
```bash
# Depuis https://nodejs.org/
# Version LTS recommandée (20.x+)
```

### 3. Installer les dépendances système (Windows)
```bash
# Pour MSVC (recommandé pour Tauri sur Windows)
# Visual Studio Build Tools ou Visual Studio Community
# https://visualstudio.microsoft.com/visual-cpp-build-tools/
```

## Installation et démarrage

```bash
# Se placer dans le dossier desktop-tauri
cd desktop-tauri

# Installer les dépendances npm
npm install

# Démarrage en développement
npm run dev

# Builder l'application Windows
npm run build:windows
```

## Résultat du build

L'exécutable sera généré dans :
```
src-tauri/target/release/
```

Les installateurs Windows (NSIS et MSI) seront dans :
```
src-tauri/target/release/bundle/
```

## Structure

- `package.json` - Configuration npm et scripts
- `vite.config.js` - Configuration du bundler frontend
- `index.html` - Point d'entrée de l'application
- `src-tauri/` - Code Rust et configuration Tauri
  - `Cargo.toml` - Dépendances Rust
  - `tauri.conf.json` - Configuration de l'application
  - `src/main.rs` - Point d'entrée Rust
  - `resources/live-app/` - Fichiers web
  - `resources/server/` - Serveur Node.js exécutable

## Serveur Node.js intégré

Le serveur Node.js est intégré dans l'application et démarre automatiquement. 
Il est accessible sur `https://localhost:3000`.

## Notes Windows

- Tauri sur Windows requiert le compilateur MSVC
- L'application est codesignée automatiquement si disponible
- Les certificats HTTPS sont inclus pour le développement local
