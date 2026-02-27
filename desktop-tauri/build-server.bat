@echo off
REM Script pour packager le serveur Node.js en exécutable Windows

echo Compilation du serveur Node.js en .exe...

REM Installer pkg globalement si nécessaire
npm install -g pkg

REM Créer le dossier de sortie
if not exist "src-tauri\resources\server" mkdir "src-tauri\resources\server"

REM Compiler avec pkg
pkg ..\live-interpretation-node\server.js --output "%cd%\src-tauri\resources\server\server.exe" --target win-x64 --compress Brotli

if %errorlevel% equ 0 (
    echo ✓ Serveur compilé avec succès: src-tauri\resources\server\server.exe
) else (
    echo X Erreur lors de la compilation du serveur
    exit /b 1
)
