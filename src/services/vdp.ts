export interface VDPInput {
  air_temperature: number;
  relative_humidity: number;
}

export interface VDPOutput {
  vdp: number;
}

function saturationVaporPressure(t: number): number {
  return 0.6108 * Math.exp((17.27 * t) / (t + 237.3));
}

function vaporPressure(svp: number, rh: number): number {
  return svp * (rh / 100.0);
}

export function calculateVDP(input: VDPInput): VDPOutput {
  const { air_temperature, relative_humidity } = input;
  if (!Number.isFinite(air_temperature) || !Number.isFinite(relative_humidity)) {
    throw new Error('air_temperature and relative_humidity must be finite numbers');
  }
  const svp = saturationVaporPressure(air_temperature);
  const vp = vaporPressure(svp, relative_humidity);
  const vdpValue = svp - vp;

  return {
    vdp: Math.round(vdpValue * 100.0) / 100.0,
  };
}
