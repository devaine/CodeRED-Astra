#!/usr/bin/env pwsh

Write-Host "🚀 CodeRED-Astra Setup Verification" -ForegroundColor Green
Write-Host "=================================" -ForegroundColor Green

# Check if Docker is available
Write-Host "`n📦 Checking Docker..." -ForegroundColor Yellow
try {
    $dockerVersion = docker --version
    Write-Host "✅ Docker found: $dockerVersion" -ForegroundColor Green
} catch {
    Write-Host "❌ Docker not found. Please install Docker Desktop." -ForegroundColor Red
    exit 1
}

# Check if .env exists
Write-Host "`n🔧 Checking environment setup..." -ForegroundColor Yellow
if (Test-Path ".env") {
    Write-Host "✅ .env file exists" -ForegroundColor Green
} else {
    Write-Host "⚠️  .env file not found. Creating from template..." -ForegroundColor Yellow
    Copy-Item ".env.example" ".env"
    Write-Host "✅ Created .env file. Please edit it with your credentials!" -ForegroundColor Green
}

# Check Node.js (for local development)
Write-Host "`n📱 Checking Node.js..." -ForegroundColor Yellow
try {
    $nodeVersion = node --version
    Write-Host "✅ Node.js found: $nodeVersion" -ForegroundColor Green
} catch {
    Write-Host "⚠️  Node.js not found (needed for local frontend development)" -ForegroundColor Yellow
}

# Check Rust (for local development)
Write-Host "`n🦀 Checking Rust..." -ForegroundColor Yellow
try {
    $rustVersion = rustc --version
    Write-Host "✅ Rust found: $rustVersion" -ForegroundColor Green
} catch {
    Write-Host "⚠️  Rust not found (needed for local backend development)" -ForegroundColor Yellow
}

Write-Host "`n🎯 Setup Summary:" -ForegroundColor Cyan
Write-Host "=================" -ForegroundColor Cyan
Write-Host "• Frontend: React + Vite + Tailwind CSS" -ForegroundColor White
Write-Host "• Backend:  Rust + Warp + SQLx + MySQL" -ForegroundColor White
Write-Host "• Docker:   Full stack containerization" -ForegroundColor White

Write-Host "`n🚀 Quick Start Commands:" -ForegroundColor Magenta
Write-Host "========================" -ForegroundColor Magenta
Write-Host "1. Start full stack:    docker-compose up --build" -ForegroundColor White
Write-Host "2. Frontend dev:        cd web-app && npm install && npm run dev" -ForegroundColor White
Write-Host "3. Backend dev:         cd rust-engine && cargo run" -ForegroundColor White

Write-Host "`n📍 Access URLs:" -ForegroundColor Cyan
Write-Host "===============" -ForegroundColor Cyan
Write-Host "• Web App:      http://localhost (Docker) or http://localhost:5173 (local)" -ForegroundColor White
Write-Host "• Rust API:     http://localhost:8000" -ForegroundColor White
Write-Host "• phpMyAdmin:   http://127.0.0.1:8080" -ForegroundColor White

Write-Host "`n✨ Your hackathon project is ready! Happy coding! ✨" -ForegroundColor Green