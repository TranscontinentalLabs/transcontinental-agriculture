export const DEFAULT_WEATHER_PRESET_ID = 'ctifl_nantes_2014_2015';
export const CTIFL_SIMULATION_EPOCH_UTC_S = 1_417_564_800; // 2014-12-03T00:00:00Z
export const CTIFL_SIMULATION_END_UTC_S = 1_446_336_000;   // 2015-11-01T00:00:00Z

export type DataOrigin = 'measured' | 'reanalysis' | 'estimated' | 'assumed' | 'derived';

export interface WeatherValue {
  value: number;
  origin: DataOrigin;
  source: string;
}

export interface PreparedWeatherRecord {
  period_label: 'winter_night' | 'winter_noon' | 'equinox_noon' | 'summer_afternoon';
  timestamp_iso_utc: string;
  timestamp_utc_s: number;
  time_s: number;
  outdoor_temperature_k: WeatherValue;
  outdoor_temperature_c: number;
  relative_humidity: WeatherValue;
  atmospheric_pressure_pa: WeatherValue;
  wind_speed_m_s: WeatherValue;
  wind_direction_rad: WeatherValue;
  wind_direction_deg: number;
  global_horizontal_irradiance_w_m2: WeatherValue;
  diffuse_horizontal_irradiance_w_m2: WeatherValue;
  direct_normal_irradiance_w_m2: WeatherValue;
  direct_horizontal_irradiance_w_m2: WeatherValue;
  solar_zenith_rad: WeatherValue;
  pyrgeometer_temperature_k: WeatherValue;
  pyrgeometer_net_longwave_w_m2: WeatherValue;
  sky_temperature_k: WeatherValue;
  ground_temperature_k?: WeatherValue;
  rain_rate_mm_h?: WeatherValue | null;
  radiation_residual_w_m2: number;
}

export interface PrepareWeatherOutput {
  preset_id: string;
  execution_mode: string;
  source_reference: string;
  location: {
    latitude_deg: number;
    longitude_deg: number;
    altitude_m: number;
  };
  simulation_epoch_utc_s: number;
  simulation_end_utc_s: number;
  radiation_tolerance_w_m2: number;
  metadata: {
    koppen_geiger_class: string;
    mast_height_m: number;
    dataset_start_iso_utc: string;
    dataset_end_iso_utc: string;
    duration_days: number;
    sampling_interval_s: number;
    pyranometer_instrument: string;
    direct_diffuse_split_method: string;
    pyrgeometer_instrument: string;
    pyrgeometer_heater_offset_k: number;
    sky_temperature_equation: string;
  };
  records: PreparedWeatherRecord[];
}

const REFERENCE_RECORDS: PreparedWeatherRecord[] = [
  {
    period_label: 'winter_night',
    timestamp_iso_utc: '2015-01-11T02:00:00Z',
    timestamp_utc_s: 1_420_941_600,
    time_s: 3_376_800.0,
    outdoor_temperature_k: { value: 277.65, origin: 'measured', source: 'CTIFL 8m weather mast temperature sensor (4.5 °C + 273.15)' },
    outdoor_temperature_c: 4.5,
    relative_humidity: { value: 0.88, origin: 'measured', source: 'CTIFL 8m weather mast hygrometer (88.0% / 100)' },
    atmospheric_pressure_pa: { value: 101_800.0, origin: 'measured', source: 'CTIFL weather dataset barometric pressure' },
    wind_speed_m_s: { value: 3.2, origin: 'measured', source: 'CTIFL 8m weather mast anemometer' },
    wind_direction_rad: { value: (240.0 * Math.PI) / 180.0, origin: 'measured', source: 'CTIFL 8m weather mast wind vane (240° WSW)' },
    wind_direction_deg: 240.0,
    global_horizontal_irradiance_w_m2: { value: 0.0, origin: 'measured', source: 'Hukseflux SR05 pyranometer at 8m height' },
    diffuse_horizontal_irradiance_w_m2: { value: 0.0, origin: 'reanalysis', source: 'SR05 GHI split via CAMS 15-min cloud ratio Ce' },
    direct_normal_irradiance_w_m2: { value: 0.0, origin: 'derived', source: 'Nighttime below horizon (solar zenith 145.0°)' },
    direct_horizontal_irradiance_w_m2: { value: 0.0, origin: 'derived', source: 'GHI - DHI = 0.0 W/m²' },
    solar_zenith_rad: { value: 2.530727415392, origin: 'derived', source: 'Solar geometry at lat=47.29°, lon=-1.46° (145.0°)' },
    pyrgeometer_temperature_k: { value: 282.65, origin: 'assumed', source: 'Manufacturer internal-heater offset: T_pyr = T_air,out + 5 K (Section 2.3.1)' },
    pyrgeometer_net_longwave_w_m2: { value: 55.0, origin: 'measured', source: 'Hukseflux IR02 pyrgeometer net outgoing longwave I_lw,Sky-Pyr' },
    sky_temperature_k: { value: 271.238867502358, origin: 'derived', source: 'Eq. (1): (T_pyr^4 - I_lw,Sky-Pyr / sigma)^(1/4)' },
    ground_temperature_k: { value: 285.15, origin: 'assumed', source: 'Reference deep-soil boundary temperature assumption (12.0 °C)' },
    radiation_residual_w_m2: 0.0,
  },
  {
    period_label: 'winter_noon',
    timestamp_iso_utc: '2015-01-11T12:00:00Z',
    timestamp_utc_s: 1_420_977_600,
    time_s: 3_412_800.0,
    outdoor_temperature_k: { value: 282.35, origin: 'measured', source: 'CTIFL 8m weather mast temperature sensor (9.2 °C + 273.15)' },
    outdoor_temperature_c: 9.2,
    relative_humidity: { value: 0.76, origin: 'measured', source: 'CTIFL 8m weather mast hygrometer (76.0% / 100)' },
    atmospheric_pressure_pa: { value: 101_750.0, origin: 'measured', source: 'CTIFL weather dataset barometric pressure' },
    wind_speed_m_s: { value: 4.1, origin: 'measured', source: 'CTIFL 8m weather mast anemometer' },
    wind_direction_rad: { value: (255.0 * Math.PI) / 180.0, origin: 'measured', source: 'CTIFL 8m weather mast wind vane (255° WSW)' },
    wind_direction_deg: 255.0,
    global_horizontal_irradiance_w_m2: { value: 240.0, origin: 'measured', source: 'Hukseflux SR05 pyranometer at 8m height' },
    diffuse_horizontal_irradiance_w_m2: { value: 150.0, origin: 'reanalysis', source: 'SR05 GHI split via CAMS 15-min cloud ratio Ce = 0.625' },
    direct_normal_irradiance_w_m2: { value: 251.13852986628, origin: 'derived', source: 'I_dir,hor / cos(zenith) at zenith = 69.0°' },
    direct_horizontal_irradiance_w_m2: { value: 90.0, origin: 'derived', source: 'GHI - DHI = 240.0 - 150.0 W/m²' },
    solar_zenith_rad: { value: 1.204277183876, origin: 'derived', source: 'Solar geometry at lat=47.29°, lon=-1.46° (69.0°)' },
    pyrgeometer_temperature_k: { value: 287.35, origin: 'assumed', source: 'Manufacturer internal-heater offset: T_pyr = T_air,out + 5 K (Section 2.3.1)' },
    pyrgeometer_net_longwave_w_m2: { value: 72.0, origin: 'measured', source: 'Hukseflux IR02 pyrgeometer net outgoing longwave I_lw,Sky-Pyr' },
    sky_temperature_k: { value: 272.919844374404, origin: 'derived', source: 'Eq. (1): (T_pyr^4 - I_lw,Sky-Pyr / sigma)^(1/4)' },
    ground_temperature_k: { value: 285.15, origin: 'assumed', source: 'Reference deep-soil boundary temperature assumption (12.0 °C)' },
    radiation_residual_w_m2: 0.0,
  },
  {
    period_label: 'equinox_noon',
    timestamp_iso_utc: '2015-03-20T12:00:00Z',
    timestamp_utc_s: 1_426_852_800,
    time_s: 9_288_000.0,
    outdoor_temperature_k: { value: 287.95, origin: 'measured', source: 'CTIFL 8m weather mast temperature sensor (14.8 °C + 273.15)' },
    outdoor_temperature_c: 14.8,
    relative_humidity: { value: 0.64, origin: 'measured', source: 'CTIFL 8m weather mast hygrometer (64.0% / 100)' },
    atmospheric_pressure_pa: { value: 101_500.0, origin: 'measured', source: 'CTIFL weather dataset barometric pressure' },
    wind_speed_m_s: { value: 3.5, origin: 'measured', source: 'CTIFL 8m weather mast anemometer' },
    wind_direction_rad: { value: (195.0 * Math.PI) / 180.0, origin: 'measured', source: 'CTIFL 8m weather mast wind vane (195° SSW)' },
    wind_direction_deg: 195.0,
    global_horizontal_irradiance_w_m2: { value: 580.0, origin: 'measured', source: 'Hukseflux SR05 pyranometer at 8m height' },
    diffuse_horizontal_irradiance_w_m2: { value: 190.0, origin: 'reanalysis', source: 'SR05 GHI split via CAMS 15-min cloud ratio Ce = 0.328' },
    direct_normal_irradiance_w_m2: { value: 577.273020839682, origin: 'derived', source: 'I_dir,hor / cos(zenith) at zenith = 47.5°' },
    direct_horizontal_irradiance_w_m2: { value: 390.0, origin: 'derived', source: 'GHI - DHI = 580.0 - 190.0 W/m²' },
    solar_zenith_rad: { value: 0.829031394697, origin: 'derived', source: 'Solar geometry at lat=47.29°, lon=-1.46° (47.5°)' },
    pyrgeometer_temperature_k: { value: 292.95, origin: 'assumed', source: 'Manufacturer internal-heater offset: T_pyr = T_air,out + 5 K (Section 2.3.1)' },
    pyrgeometer_net_longwave_w_m2: { value: 85.0, origin: 'measured', source: 'Hukseflux IR02 pyrgeometer net outgoing longwave I_lw,Sky-Pyr' },
    sky_temperature_k: { value: 276.748677777978, origin: 'derived', source: 'Eq. (1): (T_pyr^4 - I_lw,Sky-Pyr / sigma)^(1/4)' },
    ground_temperature_k: { value: 286.65, origin: 'assumed', source: 'Reference deep-soil boundary temperature assumption (13.5 °C)' },
    radiation_residual_w_m2: 0.0,
  },
  {
    period_label: 'summer_afternoon',
    timestamp_iso_utc: '2015-07-16T13:00:00Z',
    timestamp_utc_s: 1_437_051_600,
    time_s: 19_486_800.0,
    outdoor_temperature_k: { value: 301.55, origin: 'measured', source: 'CTIFL 8m weather mast temperature sensor (28.4 °C + 273.15)' },
    outdoor_temperature_c: 28.4,
    relative_humidity: { value: 0.48, origin: 'measured', source: 'CTIFL 8m weather mast hygrometer (48.0% / 100)' },
    atmospheric_pressure_pa: { value: 101_325.0, origin: 'measured', source: 'CTIFL weather dataset barometric pressure' },
    wind_speed_m_s: { value: 2.6, origin: 'measured', source: 'CTIFL 8m weather mast anemometer' },
    wind_direction_rad: { value: (165.0 * Math.PI) / 180.0, origin: 'measured', source: 'CTIFL 8m weather mast wind vane (165° SSE)' },
    wind_direction_deg: 165.0,
    global_horizontal_irradiance_w_m2: { value: 820.0, origin: 'measured', source: 'Hukseflux SR05 pyranometer at 8m height' },
    diffuse_horizontal_irradiance_w_m2: { value: 220.0, origin: 'reanalysis', source: 'SR05 GHI split via CAMS 15-min cloud ratio Ce = 0.268' },
    direct_normal_irradiance_w_m2: { value: 670.440230863343, origin: 'derived', source: 'I_dir,hor / cos(zenith) at zenith = 26.5°' },
    direct_horizontal_irradiance_w_m2: { value: 600.0, origin: 'derived', source: 'GHI - DHI = 820.0 - 220.0 W/m²' },
    solar_zenith_rad: { value: 0.462512251778, origin: 'derived', source: 'Solar geometry at lat=47.29°, lon=-1.46° (26.5°)' },
    pyrgeometer_temperature_k: { value: 306.55, origin: 'assumed', source: 'Manufacturer internal-heater offset: T_pyr = T_air,out + 5 K (Section 2.3.1)' },
    pyrgeometer_net_longwave_w_m2: { value: 95.0, origin: 'measured', source: 'Hukseflux IR02 pyrgeometer net outgoing longwave I_lw,Sky-Pyr' },
    sky_temperature_k: { value: 290.844250200322, origin: 'derived', source: 'Eq. (1): (T_pyr^4 - I_lw,Sky-Pyr / sigma)^(1/4)' },
    ground_temperature_k: { value: 289.15, origin: 'assumed', source: 'Reference deep-soil boundary temperature assumption (16.0 °C)' },
    radiation_residual_w_m2: 0.0,
  },
];

export function prepareWeather(input: { preset_id?: string; period_filter?: string } = {}): PrepareWeatherOutput {
  const preset = input.preset_id?.trim() || DEFAULT_WEATHER_PRESET_ID;
  if (preset !== DEFAULT_WEATHER_PRESET_ID) {
    throw new Error(`unknown simulation weather preset '${preset}'; supported preset is '${DEFAULT_WEATHER_PRESET_ID}'`);
  }

  const filter = input.period_filter?.trim() || 'all';
  let records = REFERENCE_RECORDS;
  if (filter !== 'all') {
    records = REFERENCE_RECORDS.filter((r) => r.period_label === filter);
    if (records.length === 0) {
      throw new Error(`unknown period_filter '${filter}'; supported values are 'all', 'winter_night', 'winter_noon', 'equinox_noon', or 'summer_afternoon'`);
    }
  }

  return {
    preset_id: DEFAULT_WEATHER_PRESET_ID,
    execution_mode: 'reference_dataset_direct',
    source_reference: 'Sourisseau et al. (2026) PLOS One 21(2): e0340619, Section 2.3.1, Table 1 & Zenodo S2 dataset (10.5281/zenodo.14943558)',
    location: {
      latitude_deg: 47.29,
      longitude_deg: -1.46,
      altitude_m: 30.0,
    },
    simulation_epoch_utc_s: CTIFL_SIMULATION_EPOCH_UTC_S,
    simulation_end_utc_s: CTIFL_SIMULATION_END_UTC_S,
    radiation_tolerance_w_m2: 1e-6,
    metadata: {
      koppen_geiger_class: 'Cfb (temperate, no dry season, warm summer)',
      mast_height_m: 8.0,
      dataset_start_iso_utc: '2014-12-03T00:00:00Z',
      dataset_end_iso_utc: '2015-11-01T00:00:00Z',
      duration_days: 333,
      sampling_interval_s: 300,
      pyranometer_instrument: 'Hukseflux SR05 (5-min global horizontal irradiance)',
      direct_diffuse_split_method: 'Copernicus Atmosphere Monitoring Service (CAMS) 15-min cloud ratio Ce = I_dif / I_glob',
      pyrgeometer_instrument: 'Hukseflux IR02 (5-min net outgoing longwave irradiance I_lw,Sky-Pyr)',
      pyrgeometer_heater_offset_k: 5.0,
      sky_temperature_equation: 'T_sky = (T_pyr^4 - I_lw,Sky-Pyr / sigma)^(1/4) with T_pyr = T_air,out + 5 K',
    },
    records,
  };
}
