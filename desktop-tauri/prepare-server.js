#!/usr/bin/env node
/**
 * Script pour préparer le serveur Node.js compilé pour Tauri
 */
const fs = require('fs');
const path = require('path');
const { execSync } = require('child_process');

const resourcesDir = path.join(__dirname, 'src-tauri', 'resources', 'server');
const serverSource = path.join(__dirname, '..', 'live-interpretation-node');

// Créer le répertoire s'il n'existe pas
if (!fs.existsSync(resourcesDir)) {
  fs.mkdirSync(resourcesDir, { recursive: true });
}

console.log('📦 Préparation du serveur Node.js...');
console.log(`Source: ${serverSource}`);
console.log(`Destination: ${resourcesDir}`);

try {
  // Installer pkg globalement
  console.log('📥 Installation de pkg...');
  execSync('npm install -g pkg@5', { stdio: 'inherit' });

  // Compiler le serveur
  console.log('🔨 Compilation du serveur...');
  const target = process.platform === 'win32' ? 'win-x64' : 'linux-x64';
  const ext = process.platform === 'win32' ? '.exe' : '';
  const output = path.join(resourcesDir, `server${ext}`);

  const cmd = `pkg "${path.join(serverSource, 'server.js')}" --output "${output}" --target ${target} --compress Brotli`;
  
  execSync(cmd, { stdio: 'inherit' });

  console.log('✅ Serveur compilé avec succès!');
  console.log(`📍 Exécutable: ${output}`);
} catch (error) {
  console.error('❌ Erreur lors de la préparation du serveur:', error.message);
  process.exit(1);
}
