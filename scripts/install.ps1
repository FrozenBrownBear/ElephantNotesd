<#
.SYNOPSIS
    Install‑notes_app.ps1 – Installe Rust et compile le projet sous Windows.

.NOTES
    Nécessite PowerShell 5+ et l'accès Internet.
#>

$ErrorActionPreference = 'Stop'

Write-Host '▶ Vérification de Rust…'
if (-not (Get-Command rustup -ErrorAction SilentlyContinue)) {
    Write-Host '▶ Installation de Rust…'
    Invoke-WebRequest https://win.rustup.rs -OutFile rustup-init.exe
    Start-Process -Wait -FilePath .\rustup-init.exe -ArgumentList '-y'
    $env:Path += ';' + "$env:USERPROFILE\.cargo\bin"
}

Write-Host '▶ Ajout cible x86_64-pc-windows-gnu'
rustup target add x86_64-pc-windows-gnu

Write-Host '▶ Compilation (release)…'
cargo build --release
Write-Host '✅ Binaire dispo dans target\release\'