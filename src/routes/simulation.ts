import { Router, Request, Response } from 'express';
import { getGreenhouseConfiguration } from '../services/geometry.js';
import { prepareWeather } from '../services/weather.js';
import { prepareSolarGeometry, prepareDiffuseSolarGeometry } from '../services/solar.js';
import { prepareMultilayerOptics } from '../services/optics.js';
import { calculateEnergy } from '../services/energy.js';
import { priceEnergy } from '../services/tariffs.js';
import { lookupSolarPackage } from '../services/solarPackage.js';
import { estimateConstruction } from '../services/construction.js';
import { summarizeSimulation } from '../services/simulationSummary.js';
import { assemblePlanningReport } from '../services/planningReport.js';
import { runSimulation } from '../services/coupledSimulation.js';

export const simulationRouter = Router();

simulationRouter.post('/get-greenhouse-configuration', (req: Request, res: Response) => {
  try {
    const result = getGreenhouseConfiguration(req.body || {});
    res.json(result);
  } catch (err: any) {
    res.status(400).json({ error: err.message });
  }
});

simulationRouter.post('/prepare-weather', (req: Request, res: Response) => {
  try {
    const result = prepareWeather(req.body || {});
    res.json(result);
  } catch (err: any) {
    res.status(400).json({ error: err.message });
  }
});

simulationRouter.post('/prepare-solar-geometry', (req: Request, res: Response) => {
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

simulationRouter.post('/prepare-solar-geometry/diffuse', (req: Request, res: Response) => {
  try {
    const result = prepareDiffuseSolarGeometry(req.body || {});
    res.json(result);
  } catch (err: any) {
    res.status(400).json({ error: err.message });
  }
});

simulationRouter.post('/prepare-multilayer-optics', (req: Request, res: Response) => {
  try {
    const result = prepareMultilayerOptics(req.body || {});
    res.json(result);
  } catch (err: any) {
    res.status(400).json({ error: err.message });
  }
});

simulationRouter.post('/run-simulation', (req: Request, res: Response) => {
  try {
    const result = runSimulation(req.body || {});
    res.json(result);
  } catch (err: any) {
    res.status(400).json({ error: err.message });
  }
});

simulationRouter.post('/summarize-simulation', (req: Request, res: Response) => {
  try {
    const result = summarizeSimulation(req.body || {});
    res.json(result);
  } catch (err: any) {
    res.status(400).json({ error: err.message });
  }
});

simulationRouter.post('/calculate-energy', (req: Request, res: Response) => {
  try {
    const result = calculateEnergy(req.body || {});
    res.json(result);
  } catch (err: any) {
    res.status(400).json({ error: err.message });
  }
});

simulationRouter.post('/price-energy', (req: Request, res: Response) => {
  try {
    const result = priceEnergy(req.body || {});
    res.json(result);
  } catch (err: any) {
    res.status(400).json({ error: err.message });
  }
});

simulationRouter.post('/lookup-solar-package', (req: Request, res: Response) => {
  try {
    const result = lookupSolarPackage(req.body || {});
    res.json(result);
  } catch (err: any) {
    res.status(400).json({ error: err.message });
  }
});

simulationRouter.post('/estimate-construction', (req: Request, res: Response) => {
  try {
    const result = estimateConstruction(req.body || {});
    res.json(result);
  } catch (err: any) {
    res.status(400).json({ error: err.message });
  }
});

simulationRouter.post('/assemble-planning-report', (req: Request, res: Response) => {
  try {
    const result = assemblePlanningReport(req.body || {});
    res.json(result);
  } catch (err: any) {
    res.status(400).json({ error: err.message });
  }
});

// Specific submodel endpoints returning physical states or coupled subsets
const submodels = [
  'prepare-air-volume',
  'prepare-natural-ventilation',
  'prepare-air-leakage',
  'prepare-screen-air-exchange',
  'prepare-airflow-transport',
  'prepare-surface-convection',
  'prepare-longwave-radiation',
  'prepare-condensation',
  'prepare-thermal-storage',
  'prepare-heating-pipes',
  'prepare-canopy-heat-balance',
  'prepare-crop-transpiration',
  'prepare-photosynthesis',
  'prepare-carbohydrate-allocation',
  'get-crop-management-plan',
  'prepare-fruit-harvest',
  'prepare-co2-exchange',
];

for (const sub of submodels) {
  simulationRouter.post(`/${sub}`, (req: Request, res: Response) => {
    // When called independently, deliver coupled simulation slice for that submodel
    try {
      const full = runSimulation(req.body || {});
      res.json({
        submodel: sub,
        preset_id: full.preset_id,
        status: full.status,
        records: full.records.map((r: any) => ({
          period_label: r.period_label,
          timestamp: r.timestamp_iso_utc,
          climate: r.climate,
          crop: r.crop,
          energy: r.energy_and_co2,
          feedback_trace: r.feedback_loop_trace,
        })),
      });
    } catch (err: any) {
      res.status(400).json({ error: err.message });
    }
  });
}
