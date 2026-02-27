#!/bin/bash
# Script pour packager le serveur Node.js en exécutable

echo "Compilation du serveur Node.js..."

# Installer pkg globalement si nécessaire
npm install -g pkg

# Créer le dossier de sortie
mkdir -p "src-tauri/resources/server"

# Compiler avec pkg pour Linux 64-bit
pkg ../live-interpretation-node/server.js --output "src-tauri/resources/server/server" --target linux-x64 --compress Brotli

if [ $? -eq 0 ]; then
    echo "✓ Serveur compilé avec succès: src-tauri/resources/server/server"
    chmod +x "src-tauri/resources/server/server"
else
    echo "X Erreur lors de la compilation du serveur"
    exit 1
fi
