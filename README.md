# Transcontinental Agriculture

Greenhouse engineering simulation, climate matching, energy analysis, and construction feasibility planning backend migrated to Node.js / Express.

## API Endpoints

- **GET `/health`**: Service health status
- **POST `/tools/calculate-vdp`**: Vapor Pressure Deficit calculation from air temperature and relative humidity
- **POST `/tools/get-greenhouse-configuration`**: Greenhouse geometry (floor area, cultivated area, enclosed volume)
- **POST `/tools/start-simulation`**: Launch greenhouse simulation
- **POST `/messaging/user-message`**: Intent interpretation and recommendation routing
- **POST `/simulation/prepare-weather`**: Research-grade reference weather dataset (Nantes CTIFL)
- **POST `/simulation/run-simulation`**: Coupled 22-submodel physics and biology simulation
- **POST `/simulation/calculate-energy`**: Electrical and thermal peak demand & seasonal usage
- **POST `/simulation/price-energy`**: Tariff pricing (KPLC Kenya / EDF France) and OPEX breakdown
- **POST `/simulation/lookup-solar-package`**: Sized PV, inverter, and LiFePO4 BESS solar packages
- **POST `/simulation/estimate-construction`**: Bill of Quantities (BoQ) and CAPEX estimation
- **POST `/simulation/assemble-planning-report`**: Full executive feasibility highlights, revenue, and payback cycles

## Running Locally

```bash
npm install
npm run dev
```

App runs on port 3000 (0.0.0.0:3000).
