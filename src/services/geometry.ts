export interface GreenhouseConfigurationInput {
  preset_id?: string;
  length?: number;
  width?: number;
  r_cul?: number;
  h_gutter?: number;
  h_ridge?: number;
}

export interface GreenhouseConfigurationOutput {
  floor_area_m2: number;
  cultivated_area_m2: number;
  volume_m3: number;
}

export const REFERENCE_GEOMETRY = {
  length: 43.2,
  width: 24.0,
  r_cul: 0.91,
  h_gutter: 6.97,
  h_ridge: 7.78,
};

export function getGreenhouseConfiguration(
  input: GreenhouseConfigurationInput = {}
): GreenhouseConfigurationOutput {
  const length = input.length ?? REFERENCE_GEOMETRY.length;
  const width = input.width ?? REFERENCE_GEOMETRY.width;
  const r_cul = input.r_cul ?? REFERENCE_GEOMETRY.r_cul;
  const h_gutter = input.h_gutter ?? REFERENCE_GEOMETRY.h_gutter;
  const h_ridge = input.h_ridge ?? REFERENCE_GEOMETRY.h_ridge;

  const fields: [string, number][] = [
    ['length', length],
    ['width', width],
    ['r_cul', r_cul],
    ['h_gutter', h_gutter],
    ['h_ridge', h_ridge],
  ];

  for (const [name, val] of fields) {
    if (!Number.isFinite(val)) {
      throw new Error(`${name} must be a finite number`);
    }
  }

  if (length <= 0) {
    throw new Error('length must be greater than zero metres');
  }
  if (width <= 0) {
    throw new Error('width must be greater than zero metres');
  }
  if (h_gutter <= 0) {
    throw new Error('h_gutter must be greater than zero metres');
  }
  if (h_ridge <= h_gutter) {
    throw new Error('h_ridge must be greater than h_gutter');
  }
  if (r_cul < 0 || r_cul > 1) {
    throw new Error('r_cul must be between 0 and 1');
  }

  const floor_area_m2 = length * width;
  const cultivated_area_m2 = r_cul * floor_area_m2;
  const roof_height_m = h_ridge - h_gutter;
  const mean_height_m = h_gutter + roof_height_m / 2.0;
  const volume_m3 = floor_area_m2 * mean_height_m;

  return {
    floor_area_m2,
    cultivated_area_m2,
    volume_m3,
  };
}
