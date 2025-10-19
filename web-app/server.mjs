import express from 'express';
import path from 'node:path';
import helmet from 'helmet';
import cors from 'cors';
import http from 'node:http';
import https from 'node:https';
import { URL, fileURLToPath } from 'node:url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

const app = express();
const PORT = Number(process.env.PORT) || 3000;
const HOST = process.env.HOST || '0.0.0.0';
const RUST_ENGINE_BASE =
  process.env.RUST_ENGINE_BASE ||
  process.env.RUST_ENGINE_URL ||
  'http://rust-engine:8000';

app.set('trust proxy', true);
app.use(helmet({ contentSecurityPolicy: false }));
app.use(cors());

app.get('/api/healthz', (_req, res) => {
  res.json({ status: 'ok', upstream: RUST_ENGINE_BASE });
});

// Stream all /api requests directly to the rust engine (supports JSON, multipart, SSE, etc.)
app.use('/api', (req, res) => {
  const targetUrl = new URL(req.originalUrl, RUST_ENGINE_BASE);
  const client = targetUrl.protocol === 'https:' ? https : http;

  const headers = { ...req.headers, host: targetUrl.host };

  const proxyReq = client.request(
    targetUrl,
    {
      method: req.method,
      headers,
    },
    (upstream) => {
      res.status(upstream.statusCode || 502);
      for (const [key, value] of Object.entries(upstream.headers)) {
        if (typeof value !== 'undefined') {
          res.setHeader(key, value);
        }
      }
      upstream.pipe(res);
    }
  );

  proxyReq.on('error', (err) => {
    console.error('API proxy error:', err);
    if (!res.headersSent) {
      res.status(502).json({ error: 'proxy_failed' });
    } else {
      res.end();
    }
  });

  req.pipe(proxyReq);
});

// Serve static frontend
const distDir = path.resolve(__dirname, 'dist');
app.use(express.static(distDir));

// SPA fallback (Express 5 requires middleware instead of bare '*')
app.use((req, res) => {
  res.sendFile(path.join(distDir, 'index.html'));
});

app.listen(PORT, HOST, () => {
  console.log(`Web app server listening on http://${HOST}:${PORT}`);
  console.log(`Proxying to rust engine at ${RUST_ENGINE_BASE}`);
});
