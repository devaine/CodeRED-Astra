import express from 'express';
import path from 'node:path';
import helmet from 'helmet';
import cors from 'cors';
import fetch from 'node-fetch';
import { fileURLToPath } from 'node:url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

const app = express();
const PORT = Number(process.env.PORT) || 3000;
const HOST = process.env.HOST || '0.0.0.0';
const RUST_ENGINE_BASE =
  process.env.RUST_ENGINE_BASE ||
  process.env.RUST_ENGINE_URL ||
  'http://rust-engine:8000';
const STORAGE_DIR = path.resolve(process.env.ASTRA_STORAGE || '/app/storage');

app.set('trust proxy', true);
app.use(helmet({ contentSecurityPolicy: false }));
app.use(cors());
app.use(express.json());

app.get('/api/healthz', (_req, res) => {
  res.json({ status: 'ok', upstream: RUST_ENGINE_BASE });
});

// Proxy minimal API needed by the UI to the rust-engine container
app.post('/api/files/import-demo', async (req, res) => {
  try {
    const qs = req.url.includes('?') ? req.url.substring(req.url.indexOf('?')) : '';
    const url = `${RUST_ENGINE_BASE}/api/files/import-demo${qs}`;
    const upstream = await fetch(url, { method: 'POST', headers: { 'content-type': 'application/json' }, body: req.body ? JSON.stringify(req.body) : undefined });
    const text = await upstream.text();
    res.status(upstream.status).type(upstream.headers.get('content-type') || 'application/json').send(text);
  } catch (err) {
    console.error('import-demo proxy failed:', err);
    res.status(502).json({ error: 'proxy_failed' });
  }
});

// Serve static frontend
const distDir = path.resolve(__dirname, 'dist');
app.use(express.static(distDir));

// Expose imported files for the UI (read-only)
app.use('/storage', express.static(STORAGE_DIR));

// SPA fallback (Express 5 requires middleware instead of bare '*')
app.use((req, res) => {
  res.sendFile(path.join(distDir, 'index.html'));
});

app.listen(PORT, HOST, () => {
  console.log(`Web app server listening on http://${HOST}:${PORT}`);
  console.log(`Proxying to rust engine at ${RUST_ENGINE_BASE}`);
});
