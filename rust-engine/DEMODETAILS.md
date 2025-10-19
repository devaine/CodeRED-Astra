## Demo Runbook: ISS Systems (3-minute showcase)

This demo uses ~20 public NASA PDFs covering ISS Electrical Power, ECLSS, Avionics, and Structures. They live in `rust-engine/demo-data` and are automatically ingested via the server.

### 1) Seed demo data (one-click)

- Trigger ingestion (cloud): POST `/api/files/import-demo` (UI button available when `?debug=1` is present)
- The backend copies PDFs into storage, inserts DB rows with `pending_analysis = true`, and the FileWorker processes them.
- Processing pipeline per file:
  - Gemini Flash → comprehensive description (facts/keywords/components)
  - Gemini Pro → deep vector graph data (keywords/use cases/relationships)
  - Embed + upsert to Qdrant, mark file ready (`pending_analysis = false`)

Tip: You can list files at `GET /api/files/list`. Ready files will start to appear as analysis completes.

### 2) Showcase flow (suggested script)

1. “We ingested real ISS technical PDFs. The worker analyzes each file with Gemini and builds vector graph data for robust retrieval.”
2. Show the files list. Point out a couple of recognizable titles.
3. Run two queries (examples below) and open their results (the app calls `POST /api/query/create` then polls `/api/query/result`).
4. Highlight the grounded answer: ‘related_files’, ‘relationships’, and ‘final_answer’ fields.
5. Call out that if info isn’t present in the PDFs, the system explicitly states uncertainty (no guessing).

### 3) Demo queries (pick 2–3)

- Electrical Power System (EPS)
  - “Trace the power path from the P6 solar array to the BCDU. Where are likely ground fault points?”
  - “What is the role of the DC Switching Unit in array power management?”
- ECLSS
  - “Which modules are part of water recovery, and how does the Oxygen Generator Assembly interface?”
  - “Summarize the CDRA cycle and downstream subsystems it impacts.”
- C&DH / Avionics
  - “In the US Lab, a blue/white wire connects to MDM ‘LAB1’. What are possible data pathways?”
  - “Describe the onboard LAN segments and links to MDMs.”
- Structures / Robotics
  - “Where does the Latching End Effector connect on S1 truss?”
  - “What is the Mobile Transporter’s role in SSRMS operations?”

### 4) Reset/refresh (optional)

- POST `/api/files/import-demo?force=1` to overwrite by filename and re-queue analysis.

### Appendix: Example sources

- EPS: 20110014867, 20040171627, 19900007297, 20120002931, 20100029672
- ECLSS: 20170008316, 20070019910, 20080039691, 20100029191, 20070019929
- C&DH: 20000012543, 20100029690, 19950014639, 20010023477, 19980227289
- Structures/Robotics: 20020054238, 20010035542, 20140001008, Destiny fact sheet, 20020088289