# CodeRED-Astra Architecture

## Overview

CodeRED-Astra is a Retrieval-Augmented Generation (RAG) system for querying ISS technical documentation using vector search, MySQL metadata storage, and Gemini AI for analysis and response generation.

## System Components

### 1. **Rust Backend** (`rust-engine/`)
High-performance Rust backend using Warp for HTTP, SQLx for MySQL, and Reqwest for external API calls.

#### Modules

**`main.rs`** - Entry point
- Initializes tracing, database, storage
- Spawns FileWorker and QueryWorker background tasks
- Serves API routes on port 8000

**`db.rs`** - Database initialization
- Connects to MySQL
- Creates `files` table (id, filename, path, description, pending_analysis, analysis_status)
- Creates `queries` table (id, status, payload, result, timestamps)

**`api.rs`** - HTTP endpoints
- `POST /api/files` - Upload file (multipart/form-data)
- `POST /api/files/import-demo` - Bulk import from demo-data directory
- `GET /api/files/list` - List all files with status
- `GET /api/files/delete?id=` - Delete file and remove from Qdrant
- `POST /api/query/create` - Create new query (returns query ID)
- `GET /api/query/status?id=` - Check query status
- `GET /api/query/result?id=` - Get query result
- `GET /api/query/cancel?id=` - Cancel in-progress query

**`file_worker.rs`** - File analysis pipeline
- **Background worker** that processes files with `pending_analysis = TRUE`
- Claims stale/queued files (requeues if stuck >10 min)
- **Stage 1**: Call Gemini 1.5 Flash for initial description
- **Stage 2**: Call Gemini 1.5 Pro for deep vector graph data (keywords, relationships)
- **Stage 3**: Generate embedding and upsert to Qdrant
- **Stage 4**: Mark file as ready (`pending_analysis = FALSE`, `analysis_status = 'Completed'`)
- Resumable: Can recover from crashes/restarts

**`worker.rs`** - Query processing pipeline
- **Background worker** that processes queries with `status = 'Queued'`
- Requeues stale InProgress jobs (>10 min)
- **Stage 1**: Embed query text
- **Stage 2**: Search top-K similar vectors in Qdrant
- **Stage 3**: Fetch file metadata from MySQL (only completed files)
- **Stage 4**: Call Gemini to analyze relationships between files
- **Stage 5**: Call Gemini for final answer synthesis (strict: no speculation)
- **Stage 6**: Save results to database
- Supports cancellation checks between stages

**`gemini_client.rs`** - Gemini API integration
- `generate_text(prompt)` - Text generation with model switching via GEMINI_MODEL env var
- `demo_text_embedding(text)` - Demo 64-dim embeddings (replace with real Gemini embeddings)
- Falls back to demo responses if GEMINI_API_KEY not set

**`vector_db.rs`** - Qdrant client
- `ensure_files_collection(dim)` - Create 'files' collection with Cosine distance
- `upsert_point(id, vector)` - Store file embedding
- `search_top_k(vector, k)` - Find k nearest neighbors
- `delete_point(id)` - Remove file from index

**`storage.rs`** - File storage utilities
- `storage_dir()` - Get storage path from ASTRA_STORAGE env or default `/app/storage`
- `ensure_storage_dir()` - Create storage directory if missing
- `save_file(filename, contents)` - Save file to storage
- `delete_file(path)` - Remove file from storage

**`models.rs`** - Data structures
- `FileRecord` - File metadata (mirrors files table)
- `QueryRecord` - Query metadata (mirrors queries table)
- `QueryStatus` enum - Queued, InProgress, Completed, Cancelled, Failed

### 2. **Web App** (`web-app/`)
React + Vite frontend with Express backend for API proxying.

#### Backend (`server.mjs`)
- Express server that proxies API calls to rust-engine:8000
- Serves React static build from `/dist`
- **Why needed**: Docker networking - React can't call rust-engine directly from browser

#### Frontend (`src/`)
- `App.jsx` - Main chat interface component
- `components/ui/chat/chat-header.jsx` - Header with debug-only "Seed Demo Data" button (visible with `?debug=1`)
- Calls `/api/files/import-demo` endpoint to bulk-load ISS PDFs

### 3. **MySQL Database**
Two tables for metadata storage:

**`files` table**
```sql
id VARCHAR(36) PRIMARY KEY
filename TEXT NOT NULL
path TEXT NOT NULL
description TEXT
created_at DATETIME DEFAULT CURRENT_TIMESTAMP
pending_analysis BOOLEAN DEFAULT TRUE
analysis_status VARCHAR(32) DEFAULT 'Queued'
```

**`queries` table**
```sql
id VARCHAR(36) PRIMARY KEY
status VARCHAR(32) NOT NULL
payload JSON
result JSON
created_at DATETIME DEFAULT CURRENT_TIMESTAMP
updated_at DATETIME DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
```

### 4. **Qdrant Vector Database**
- Collection: `files`
- Dimension: 64 (demo) - replace with real Gemini embedding dimension
- Distance: Cosine similarity
- Stores file embeddings for semantic search

### 5. **Demo Data** (`rust-engine/demo-data/`)
~20 ISS technical PDFs organized by subsystem:
- Electrical Power System (EPS)
- Environmental Control & Life Support (ECLSS)
- Command & Data Handling (C&DH)
- Structures & Mechanisms

## Data Flow

### File Upload & Analysis
```
1. User uploads PDF → POST /api/files
2. API saves file to storage, inserts DB record (pending_analysis=true)
3. FileWorker claims pending file
4. Gemini 1.5 Flash generates description
5. Gemini 1.5 Pro generates vector graph data
6. Embed text → upsert to Qdrant
7. Mark file as ready (pending_analysis=false)
```

### Query Processing
```
1. User submits query → POST /api/query/create
2. API inserts query record (status='Queued')
3. QueryWorker claims queued query
4. Embed query text
5. Search Qdrant for top-K similar files
6. Fetch file metadata from MySQL
7. Gemini analyzes relationships between files
8. Gemini synthesizes final answer (no speculation)
9. Save results to database
```

## Deployment

### Development (`docker-compose.yml`)
- Local testing with hot-reload
- Bind mounts for code

### Production (`docker-compose.prod.yml`)
- Used by GitHub Actions for deployment
- Runs rust-engine as user "1004" (github-actions)
- Docker volume: `rust-storage` → `/app/storage`
- Bind mount: `/var/www/codered-astra/rust-engine/demo-data` → `/app/demo-data:ro`
- Environment variables:
  - `ASTRA_STORAGE=/app/storage`
  - `DEMO_DATA_DIR=/app/demo-data`
  - `QDRANT_URL=http://qdrant:6333`
  - `GEMINI_API_KEY=<secret>`
  - `DATABASE_URL=mysql://astraadmin:password@mysql:3306/astra`

## Key Design Decisions

### 1. **Two-Stage Analysis (Flash → Pro)**
- Flash is faster/cheaper for initial description
- Pro is better for deep analysis and relationship extraction
- Enables cost-effective scaling

### 2. **Resumable Workers**
- Workers requeue stale jobs (>10 min in InProgress)
- Survives container restarts without data loss
- Atomic state transitions via SQL

### 3. **Separation of Concerns**
- FileWorker: Makes files searchable
- QueryWorker: Answers user queries
- Independent scaling and failure isolation

### 4. **Strict Answer Generation**
- Gemini prompted to not speculate
- Must state uncertainty when info is insufficient
- Prevents hallucination in critical ISS documentation

### 5. **Demo Embeddings**
- Current: 64-dim deterministic embeddings from text hash
- Production: Replace with real Gemini text embeddings API
- Allows development/testing without embedding API credits

## API Usage Examples

### Upload File
```bash
curl -F "file=@document.pdf" http://localhost:3001/api/files
```

### Import Demo Data
```bash
curl -X POST http://localhost:3001/api/files/import-demo
```

### Create Query
```bash
curl -X POST http://localhost:3001/api/query/create \
  -H "Content-Type: application/json" \
  -d '{"q": "What is the voltage of the ISS main bus?", "top_k": 5}'
```

### Check Status
```bash
curl http://localhost:3001/api/query/status?id=<query-id>
```

### Get Result
```bash
curl http://localhost:3001/api/query/result?id=<query-id>
```

## Future Enhancements

### High Priority
1. Real Gemini text embeddings (replace demo embeddings)
2. File status UI panel (show processing progress)
3. Health check endpoint (`/health`)
4. Data purge endpoint (clear all files/queries)

### Medium Priority
1. Streaming query responses (SSE/WebSocket)
2. Query result caching
3. File chunking for large PDFs
4. User authentication

### Low Priority
1. Multi-collection support (different document types)
2. Query history UI
3. File preview in chat
4. Export results to PDF

## Troubleshooting

### Storage Permission Errors
- Ensure `/app/storage` is owned by container user
- Docker volume must be writable by user 1004 in production

### SQL Syntax Errors
- MySQL requires separate `CREATE TABLE` statements
- Cannot combine multiple DDL statements in one `sqlx::query()`

### Qdrant Connection Issues
- Check QDRANT_URL environment variable
- Ensure qdrant service is running and healthy
- Verify network connectivity between containers

### Worker Not Processing
- Check logs: `docker logs rust-engine`
- Verify database connectivity
- Look for stale InProgress jobs in queries/files tables

## Demo Presentation (3 minutes)

See `rust-engine/DEMODETAILS.md` for curated demo script with example queries.
