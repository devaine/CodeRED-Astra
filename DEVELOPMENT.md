# CodeRED-Astra Development Guide

## Project Structure

This is a hackathon-ready project with a clean separation between frontend and backend:

- **React Frontend** (`web-app/`): Modern React app with Vite and Tailwind CSS
- **Rust Engine** (`rust-engine/`): High-performance backend API server
- **Database**: MySQL 8.0 with phpMyAdmin for management

## Quick Start

### Prerequisites
- Docker & Docker Compose
- Node.js 20+ (for local development)
- Rust 1.82+ (for local development)

### Development Setup

1. **Clone and setup environment**:
```bash
cp .env.example .env
# Edit .env with your database passwords and API keys
```

2. **Start the entire stack**:
```bash
docker-compose up --build
```

3. **Access the application**:
- Frontend: http://localhost (port 80)
- Rust API: http://localhost:8000
- phpMyAdmin: http://127.0.0.1:8080

### Local Development (Recommended for Hackathon)

**Frontend Development**:
```bash
cd web-app
npm install
npm run dev  # Starts on http://localhost:5173
```

**Backend Development**:
```bash
cd rust-engine
cargo run    # Starts on http://localhost:8000
```

## Team Workflow

### Frontend Team (React)
- Work in `web-app/src/`
- Main entry: `src/App.jsx`
- Add new components in `src/components/`
- API calls go through `/api/*` (auto-proxied to Rust engine)
- Use Tailwind CSS for styling
- Hot reload enabled with Vite

### Backend Team (Rust)
- Work in `rust-engine/src/`
- Main server: `src/main.rs`
- Add new modules in `src/`
- API endpoints start with `/api/`
- Database connection via SQLx
- CORS enabled for frontend communication

## API Communication

The frontend communicates with the Rust engine via:
```javascript
// This automatically proxies to http://rust-engine:8000 in Docker
// or http://localhost:8000 in local development
fetch('/api/health')
  .then(response => response.json())
  .then(data => console.log(data));
```

## Database Schema

Edit `rust-engine/src/main.rs` to add database migrations and models as needed.

## Environment Variables

Required in `.env`:
```
MYSQL_DATABASE=astra
MYSQL_USER=astraadmin
MYSQL_PASSWORD=your_secure_password
MYSQL_ROOT_PASSWORD=your_root_password
GEMINI_API_KEY=your_gemini_key
```

## Deployment

The project is containerized and ready for deployment:
- Frontend: Static files served via Vite preview
- Backend: Optimized Rust binary
- Database: Persistent MySQL data volume

## Hackathon Tips

1. **Frontend team**: Start with the existing App.jsx and build your UI components
2. **Backend team**: Add new API endpoints in the Rust main.rs file
3. **Database**: Use phpMyAdmin at http://127.0.0.1:8080 to manage data
4. **Testing**: The app shows connection status between frontend and backend
5. **Hot reload**: Both frontend and backend support hot reload during development

## Common Issues

- **CORS errors**: Already configured, but check Rust engine CORS settings if needed
- **Database connection**: Engine gracefully handles DB offline state for initial development
- **Port conflicts**: Web runs on 80, API on 8000, phpMyAdmin on 8080
- **Build failures**: Check Node.js and Rust versions match requirements

Happy hacking! 🚀