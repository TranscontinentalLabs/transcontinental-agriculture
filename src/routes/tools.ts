import { Router, Request, Response } from 'express';
import { calculateVDP } from '../services/vdp.js';
import { getGreenhouseConfiguration } from '../services/geometry.js';
import { prepareWeather } from '../services/weather.js';
import { prepareSolarGeometry, prepareDiffuseSolarGeometry } from '../services/solar.js';

export const toolsRouter = Router();

toolsRouter.post('/start-simulation', (_req: Request, res: Response) => {
  res.json({ message: 'The simulation finished successfully' });
});

toolsRouter.post('/calculate-vdp', (req: Request, res: Response) => {
  try {
    const { air_temperature, relative_humidity } = req.body || {};
    if (air_temperature === undefined || relative_humidity === undefined) {
      return res.status(400).json({ error: 'air_temperature and relative_humidity are required' });
    }
    const result = calculateVDP({
      air_temperature: Number(air_temperature),
      relative_humidity: Number(relative_humidity),
    });
    res.json(result);
  } catch (err: any) {
    res.status(400).json({ error: err.message });
  }
});

toolsRouter.post(['/get-greenhouse-configuration', '/tools/get-greenhouse-configuration'], (req: Request, res: Response) => {
  try {
    const result = getGreenhouseConfiguration(req.body || {});
    res.json(result);
  } catch (err: any) {
    res.status(400).json({ error: err.message });
  }
});

toolsRouter.post('/prepare-weather', (req: Request, res: Response) => {
  try {
    const result = prepareWeather(req.body || {});
    res.json(result);
  } catch (err: any) {
    res.status(400).json({ error: err.message });
  }
});

toolsRouter.post('/prepare-solar-geometry', (req: Request, res: Response) => {
  try {
    if (!req.body || !req.body.roof || !req.body.glass) {
      return res.status(400).json({ error: 'roof and glass specifications are required' });
    }
    const result = prepareSolarGeometry(req.body);
    res.json(result);
  } catch (err: any) {
    res.status(400).json({ error: err.message });
  }
});

toolsRouter.post('/prepare-solar-geometry/diffuse', (req: Request, res: Response) => {
  try {
    const result = prepareDiffuseSolarGeometry(req.body || {});
    res.json(result);
  } catch (err: any) {
    res.status(400).json({ error: err.message });
  }
});
