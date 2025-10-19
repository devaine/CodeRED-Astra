# Rust Engine API and Worker

## Overview

- HTTP API (warp) under /api for file management and query lifecycle
- MySQL for metadata, Qdrant for vector similarity
- Background worker resumes queued work and re-queues stale InProgress jobs at startup

## Environment variables

- DATABASE_URL: mysql://USER:PASS@HOST:3306/DB
- QDRANT_URL: default <http://qdrant:6333>
- GEMINI_API_KEY: used for Gemini content generation (optional in demo)
- DEMO_DATA_DIR: path to the folder containing PDF demo data (default resolves to `demo-data` under the repo or `/app/demo-data` in containers)
- ASTRA_STORAGE: directory for uploaded file blobs (default `/app/storage`)
- AUTO_IMPORT_DEMO: set to `false`, `0`, `off`, or `no` to disable automatic demo import at startup (defaults to `true`)

## Endpoints (JSON)

- POST /api/files (multipart)
  - Form: file=@path
  - Response: {"success": true}

- GET /api/files/list
  - Response: {"files": [{"id","filename","path","storage_url","description"}]}

- POST /api/files/import-demo[?force=1]
  - Copies PDFs from the demo directory into storage and queues them for analysis.
  - Response: {"imported": N, "skipped": M, "files_found": K, "source_dir": "...", "attempted_paths": [...], "force": bool}
  - `force=1` deletes prior records with the same filename before re-importing.

- GET /api/files/delete?id=<file_id>
  - Response: {"deleted": true|false}

- POST /api/query/create
  - Body: {"q": "text", "top_k": 5}
  - Response: {"id": "uuid"}

- GET /api/query/status?id=<query_id>
  - Response: {"status": "Queued"|"InProgress"|"Completed"|"Cancelled"|"Failed"|"not_found"}

- GET /api/query/result?id=<query_id>
  - Response (Completed):
    {
      "result": {
        "summary": "Found N related files",
        "related_files": [
          {"id","filename","path","description","score"}
        ],
        "relationships": "...",
        "final_answer": "..."
      }
    }

- GET /api/query/cancel?id=<query_id>
  - Response: {"cancelled": true}

## Worker behavior

- Ensures Qdrant collection exists (dim 64, cosine)
- Re-queues InProgress older than 10 minutes
- Processing stages:
  1) Set InProgress
  2) Embed query text (demo now; pluggable Gemini later)
  3) Search Qdrant top_k (default 5)
  4) Join file metadata (MySQL)
  5) Gemini step: relationship analysis (strictly from provided files)
  6) Gemini step: final answer (no speculation; say unknown if insufficient)
  7) Persist result (JSON) and set Completed
  - Checks for cancellation between stages

## Local quickstart

1. docker compose up -d mysql qdrant
2. set env DATABASE_URL and QDRANT_URL
3. cargo run
4. (optional) import demo PDFs
  - Populate a folder with PDFs under `rust-engine/demo-data` (or point `DEMO_DATA_DIR` to a custom path). The server auto-resolves common locations such as the repo root, `/app/demo-data`, and the working directory when running in Docker. When the engine boots it automatically attempts this import (can be disabled by setting `AUTO_IMPORT_DEMO=false`).
   - Call the endpoint:
     - POST <http://localhost:8000/api/files/import-demo>
  - Optional query `?force=1` to overwrite existing by filename. The JSON response also echoes where the engine looked (`source_dir`, `attempted_paths`) and how many PDFs were detected (`files_found`) so misconfigurations are easy to spot. Imported files are written to the shared `/app/storage` volume; the web-app container mounts this volume read-only and serves the contents at `/storage/<filename>`.
   - Or run the PowerShell helper:
     - `./scripts/import_demo.ps1` (adds all PDFs in demo-data)
     - `./scripts/import_demo.ps1 -Force` (overwrite existing)

## Notes

- Replace demo embeddings with real Gemini calls for production
- Add auth to endpoints if needed (API key/JWT)
