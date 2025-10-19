import express from 'express';
import path from 'node:path';
import helmet from 'helmet';
import cors from 'cors';
import { fileURLToPath } from 'node:url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

const app = express();
const PORT = process.env.PORT || 3000;
const RUST_ENGINE_BASE = process.env.RUST_ENGINE_BASE || 'http://rust-engine:8000';

app.use(helmet());
app.use(cors());
app.use(express.json());

// Proxy minimal API needed by the UI to the rust-engine container
app.post('/api/files/import-demo', async (req, res) => {
  try {
    const qs = req.url.includes('?') ? req.url.substring(req.url.indexOf('?')) : '';
    const url = `${RUST_ENGINE_BASE}/api/files/import-demo${qs}`;
    const upstream = await fetch(url, { method: 'POST' });
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

// SPA fallback
app.get('*', (req, res) => {
  res.sendFile(path.join(distDir, 'index.html'));
});

app.listen(PORT, '0.0.0.0', () => {
  console.log(`Web app server listening on http://0.0.0.0:${PORT}`);
  console.log(`Proxying to rust engine at ${RUST_ENGINE_BASE}`);
});
