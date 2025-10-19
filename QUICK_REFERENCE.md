# CodeRED-Astra Quick Reference

## System Overview

**Two-worker architecture for ISS document RAG:**

1. **FileWorker**: Analyzes uploaded files (Flash → Pro → Embed → Qdrant)
2. **QueryWorker**: Answers queries (Embed → Search → Relationships → Answer)

Both workers are **resumable** and automatically recover from crashes.

## Core Data Flow

```
Upload PDF → Storage → MySQL (pending) → FileWorker → Qdrant → MySQL (ready)
                                                          ↓
User Query → MySQL (queued) → QueryWorker → Search Qdrant → Gemini → Result
```

## Module Map

| Module | Purpose | Key Functions |
|--------|---------|---------------|
| `main.rs` | Entry point | Spawns workers, serves API |
| `db.rs` | Database init | Creates files/queries tables |
| `api.rs` | HTTP endpoints | Upload, list, delete, query CRUD |
| `file_worker.rs` | File analysis | Flash→Pro→embed→upsert |
| `worker.rs` | Query processing | Search→relationships→answer |
| `gemini_client.rs` | AI integration | Text generation, embeddings |
| `vector_db.rs` | Qdrant client | Upsert, search, delete |
| `storage.rs` | File management | Save/delete files |
| `models.rs` | Data structures | FileRecord, QueryRecord |

## API Endpoints

### Files
- `POST /api/files` - Upload file
- `POST /api/files/import-demo?force=1` - Bulk import demo PDFs
- `GET /api/files/list` - List all files with status
- `GET /api/files/delete?id=<uuid>` - Delete file

### Queries
- `POST /api/query/create` - Create query
- `GET /api/query/status?id=<uuid>` - Check status
- `GET /api/query/result?id=<uuid>` - Get result
- `GET /api/query/cancel?id=<uuid>` - Cancel query

## Database Schema

### files
- `id` - UUID primary key
- `filename` - Original filename
- `path` - Storage path
- `description` - Gemini Flash description
- `pending_analysis` - FALSE when ready for search
- `analysis_status` - Queued/InProgress/Completed/Failed

### queries
- `id` - UUID primary key
- `status` - Queued/InProgress/Completed/Cancelled/Failed
- `payload` - JSON query params `{"q": "...", "top_k": 5}`
- `result` - JSON result `{"summary": "...", "related_files": [...], "relationships": "...", "final_answer": "..."}`

## Environment Variables

### Required
- `GEMINI_API_KEY` - Gemini API key
- `DATABASE_URL` - MySQL connection string
- `QDRANT_URL` - Qdrant URL (default: http://qdrant:6333)

### Optional
- `ASTRA_STORAGE` - Storage directory (default: /app/storage)
- `DEMO_DATA_DIR` - Demo data directory (default: /app/demo-data)
- `GEMINI_MODEL` - Override Gemini model (default: gemini-1.5-pro)

## Worker States

### FileWorker
1. **Queued** - File uploaded, awaiting processing
2. **InProgress** - Currently being analyzed
3. **Completed** - Ready for search (pending_analysis=FALSE)
4. **Failed** - Error during processing

### QueryWorker
1. **Queued** - Query created, awaiting processing
2. **InProgress** - Currently searching/analyzing
3. **Completed** - Result available
4. **Cancelled** - User cancelled
5. **Failed** - Error during processing

## Gemini Prompts

### FileWorker Stage 1 (Flash)
```
Describe the file '{filename}' and extract all key components, keywords, 
and details for later vectorization. Be comprehensive and factual.
```

### FileWorker Stage 2 (Pro)
```
Given the file '{filename}' and its description: {desc}
Generate a set of vector graph data (keywords, use cases, relationships) 
that can be used for broad and precise search. Only include what is 
directly supported by the file.
```

### QueryWorker Stage 4 (Relationships)
```
You are an assistant analyzing relationships STRICTLY within the provided files.
Query: {query}
Files: {file_list}
Tasks:
1) Summarize key details from the files relevant to the query.
2) Describe relationships and linkages strictly supported by these files.
3) List important follow-up questions that could be answered only using the provided files.
Rules: Do NOT guess or invent. If information is insufficient in the files, explicitly state that.
```

### QueryWorker Stage 5 (Final Answer)
```
You are to compose a final answer to the user query using only the information from the files.
Query: {query}
Files considered: {file_list}
Relationship analysis: {relationships}
Requirements:
- Use only information present in the files and analysis above.
- If the answer is uncertain or cannot be determined from the files, clearly state that limitation.
- Avoid speculation or assumptions.
Provide a concise, structured answer.
```

## Docker Architecture

### Services
- **rust-engine** - Warp API + workers (port 8000)
- **web-app** - Express + React (port 3001)
- **mysql** - MySQL 9.1 (port 3306)
- **qdrant** - Qdrant vector DB (port 6333)
- **phpmyadmin** - DB admin UI (port 8080)

### Volumes (Production)
- `rust-storage:/app/storage` - File storage (writable)
- `/var/www/codered-astra/rust-engine/demo-data:/app/demo-data:ro` - Demo PDFs (read-only)
- `~/astra-logs:/var/log` - Log files

## Common Issues

### 1. SQL Syntax Error
**Problem**: `error near 'CREATE TABLE'`
**Cause**: Multiple CREATE TABLE in one query
**Fix**: Split into separate `sqlx::query()` calls

### 2. Permission Denied
**Problem**: `Permission denied (os error 13)`
**Cause**: Container user can't write to storage
**Fix**: Use Docker volume, ensure ownership matches container user

### 3. Worker Not Processing
**Problem**: Files/queries stuck in Queued
**Cause**: Worker crashed or not started
**Fix**: Check logs, ensure workers spawned in main.rs

### 4. Qdrant Connection Failed
**Problem**: `qdrant upsert/search failed`
**Cause**: Qdrant not running or wrong URL
**Fix**: Verify QDRANT_URL, check qdrant container health

## Development Commands

```bash
# Build and run locally
cd rust-engine
cargo build
cargo run

# Check code
cargo check

# Run with logs
RUST_LOG=info cargo run

# Docker compose (dev)
docker-compose up --build

# Docker compose (production)
docker-compose -f docker-compose.prod.yml up -d

# View logs
docker logs rust-engine -f

# Rebuild single service
docker-compose build rust-engine
docker-compose up -d rust-engine
```

## Testing Flow

1. Start services: `docker-compose up -d`
2. Import demo data: `curl -X POST http://localhost:3001/api/files/import-demo`
3. Wait for FileWorker to complete (~30 seconds for 20 files)
4. Check file status: `curl http://localhost:3001/api/files/list`
5. Create query: `curl -X POST http://localhost:3001/api/query/create -H "Content-Type: application/json" -d '{"q": "ISS main bus voltage", "top_k": 5}'`
6. Check status: `curl http://localhost:3001/api/query/status?id=<id>`
7. Get result: `curl http://localhost:3001/api/query/result?id=<id>`

## Performance Notes

- FileWorker: ~1-2 sec per file (demo embeddings)
- QueryWorker: ~3-5 sec per query (search + 2 Gemini calls)
- Qdrant search: <100ms for 1000s of vectors
- MySQL queries: <10ms for simple selects

## Security Considerations

- Store GEMINI_API_KEY in GitHub Secrets (production)
- Use environment variables for all credentials
- Don't commit `.env` files
- Restrict phpmyadmin to internal network only
- Use HTTPS in production deployment
