# CodeRED-Astra 🚀

A hackathon-ready project with React frontend and Rust backend engine.

## Quick Start

```bash
# 1. Setup environment
cp .env.example .env
# Edit .env with your credentials

# 2. Start everything with Docker
docker-compose up --build

# 3. Access your app
# Frontend: http://localhost
# API: http://localhost:8000
# Database Admin: http://127.0.0.1:8080
```

## Development

**Frontend (React + Vite)**:
```bash
cd web-app
npm install
npm run dev  # http://localhost:5173
```

**Backend (Rust)**:
```bash
cd rust-engine  
cargo run    # http://localhost:8000
```

## Architecture

- **Frontend**: React 18 + Vite + Tailwind CSS
- **Backend**: Rust + Warp + SQLx
- **Database**: MySQL 8.0 + phpMyAdmin
- **API**: RESTful endpoints with CORS enabled
- **Docker**: Full containerization for easy deployment

## Project Structure

```
├── web-app/           # React frontend
│   ├── src/
│   │   ├── App.jsx    # Main component
│   │   └── main.jsx   # Entry point
│   └── Dockerfile
├── rust-engine/       # Rust backend
│   ├── src/
│   │   └── main.rs    # API server
│   └── Dockerfile
├── docker-compose.yml # Full stack orchestration
└── .env.example      # Environment template
```

## Team Workflow

- **Frontend devs**: Work in `web-app/src/`, use `/api/*` for backend calls
- **Backend devs**: Work in `rust-engine/src/`, add endpoints to main.rs
- **Database**: Access phpMyAdmin at http://127.0.0.1:8080

## Features

✅ Hot reload for both frontend and backend  
✅ Automatic API proxying from React to Rust  
✅ Database connection with graceful fallback  
✅ CORS configured for cross-origin requests  
✅ Production-ready Docker containers  
✅ Health monitoring and status dashboard  

Ready for your hackathon! See `DEVELOPMENT.md` for detailed setup instructions.
