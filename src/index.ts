import express, { Request, Response } from 'express';
import cors from 'cors';
import dotenv from 'dotenv';
import path from 'path';
import { healthRouter } from './routes/health.js';
import { messagingRouter } from './routes/messaging.js';
import { toolsRouter } from './routes/tools.js';
import { simulationRouter } from './routes/simulation.js';
import { renderConsoleHtml } from './ui/consoleHtml.js';

dotenv.config();

const app = express();
const port = parseInt(process.env.PORT || '3000', 10);
const host = process.env.HOST || '0.0.0.0';

app.use(cors());
app.use(express.json());
app.use(express.urlencoded({ extended: true }));

// Serve static assets (images, visuals)
app.use('/assets', express.static(path.join(process.cwd(), 'src/assets')));

// Serve web UI dashboard at root
app.get('/', (_req: Request, res: Response) => {
  res.setHeader('Content-Type', 'text/html; charset=utf-8');
  res.send(renderConsoleHtml());
});

// Mount routers exactly matching Axum gateway routers
app.use('/', healthRouter);
app.use('/messaging', messagingRouter);
app.use('/tools', toolsRouter);
app.use('/simulation', simulationRouter);

// Global 404 handler
app.use((req: Request, res: Response) => {
  res.status(404).json({ error: `Route not found: ${req.method} ${req.path}` });
});

// Error handling middleware
app.use((err: any, _req: Request, res: Response, _next: any) => {
  console.error('[Server Error]', err);
  res.status(500).json({ error: err.message || 'Internal Server Error' });
});

app.listen(port, host, () => {
  console.log(`Transcontinental Agriculture server running at http://${host}:${port}`);
});
