use serde::{Deserialize, Serialize};

use crate::error::AppError;

pub const DEFAULT_WEATHER_PRESET_ID: &str = "ctifl_nantes_2014_2015";

/// 2014-12-03T00:00:00Z — start of the 332-day CTIFL dataset (Section 2.3).
pub const CTIFL_SIMULATION_EPOCH_UTC_S: i64 = 1_417_564_800;

/// 2015-11-01T00:00:00Z — end of the production season dataset (Section 2.3).
pub const CTIFL_SIMULATION_END_UTC_S: i64 = 1_446_336_000;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum DataOrigin {
    Measured,
    Reanalysis,
    Estimated,
    Assumed,
    Derived,
}

#[derive(Debug, Clone, Serialize)]
pub struct WeatherValue {
    pub value: f64,
    pub origin: DataOrigin,
    pub source: &'static str,
}

#[derive(Debug, Clone, Copy, Serialize)]
pub struct WeatherLocation {
    /// North positive, south negative (CTIFL Carquefou/Nantes: 47.29° N).
    pub latitude_deg: f64,

    /// East positive, west negative (CTIFL Carquefou/Nantes: -1.46° E).
    pub longitude_deg: f64,

    /// Site altitude above sea level, metres.
    pub altitude_m: f64,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PrepareWeatherInput {
    /// Optional preset identifier. Defaults to `"ctifl_nantes_2014_2015"`.
    pub preset_id: Option<String>,

    /// Optional filter by representative period:
    /// `"winter_night"`, `"winter_noon"`, `"equinox_noon"`, `"summer_afternoon"`, or `"all"`.
    pub period_filter: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ResearchWeatherMetadataOutput {
    pub koppen_geiger_class: &'static str,
    pub mast_height_m: f64,
    pub dataset_start_iso_utc: &'static str,
    pub dataset_end_iso_utc: &'static str,
    pub duration_days: u16,
    pub sampling_interval_s: u32,
    pub pyranometer_instrument: &'static str,
    pub direct_diffuse_split_method: &'static str,
    pub pyrgeometer_instrument: &'static str,
    pub pyrgeometer_heater_offset_k: f64,
    pub sky_temperature_equation: &'static str,
}

#[derive(Debug, Clone, Serialize)]
pub struct PreparedWeatherRecord {
    pub period_label: &'static str,
    pub timestamp_iso_utc: &'static str,
    pub timestamp_utc_s: i64,
    pub time_s: f64,

    pub outdoor_temperature_k: WeatherValue,
    pub outdoor_temperature_c: f64,
    pub relative_humidity: WeatherValue,
    pub atmospheric_pressure_pa: WeatherValue,

    pub wind_speed_m_s: WeatherValue,
    pub wind_direction_rad: WeatherValue,
    pub wind_direction_deg: f64,

    pub global_horizontal_irradiance_w_m2: WeatherValue,
    pub diffuse_horizontal_irradiance_w_m2: WeatherValue,
    pub direct_normal_irradiance_w_m2: WeatherValue,
    pub direct_horizontal_irradiance_w_m2: WeatherValue,

    pub solar_zenith_rad: WeatherValue,
    pub pyrgeometer_temperature_k: WeatherValue,
    pub pyrgeometer_net_longwave_w_m2: WeatherValue,
    pub sky_temperature_k: WeatherValue,

    pub ground_temperature_k: Option<WeatherValue>,
    pub rain_rate_mm_h: Option<WeatherValue>,

    /// GHI - (DHI + direct horizontal irradiance), W/m².
    pub radiation_residual_w_m2: f64,
}

#[derive(Debug, Clone, Serialize)]
pub struct PrepareWeatherOutput {
    pub preset_id: &'static str,
    pub execution_mode: &'static str,
    pub source_reference: &'static str,
    pub location: WeatherLocation,
    pub simulation_epoch_utc_s: i64,
    pub simulation_end_utc_s: i64,
    pub radiation_tolerance_w_m2: f64,
    pub metadata: ResearchWeatherMetadataOutput,
    pub records: Vec<PreparedWeatherRecord>,
}

fn measured(value: f64, source: &'static str) -> WeatherValue {
    WeatherValue {
        value,
        origin: DataOrigin::Measured,
        source,
    }
}

fn reanalysis(value: f64, source: &'static str) -> WeatherValue {
    WeatherValue {
        value,
        origin: DataOrigin::Reanalysis,
        source,
    }
}

fn derived(value: f64, source: &'static str) -> WeatherValue {
    WeatherValue {
        value,
        origin: DataOrigin::Derived,
        source,
    }
}

fn assumed(value: f64, source: &'static str) -> WeatherValue {
    WeatherValue {
        value,
        origin: DataOrigin::Assumed,
        source,
    }
}

fn reference_records() -> Vec<PreparedWeatherRecord> {
    vec![
        // 1. Winter night (11 Jan 2015 02:00 UTC, within Fig. 8 winter evaluation window)
        PreparedWeatherRecord {
            period_label: "winter_night",
            timestamp_iso_utc: "2015-01-11T02:00:00Z",
            timestamp_utc_s: 1_420_941_600,
            time_s: 3_376_800.0,
            outdoor_temperature_k: measured(
                277.65,
                "CTIFL 8m weather mast temperature sensor (4.5 °C + 273.15)",
            ),
            outdoor_temperature_c: 4.5,
            relative_humidity: measured(
                0.88,
                "CTIFL 8m weather mast hygrometer (88.0% / 100)",
            ),
            atmospheric_pressure_pa: measured(
                101_800.0,
                "CTIFL weather dataset barometric pressure",
            ),
            wind_speed_m_s: measured(
                3.2,
                "CTIFL 8m weather mast anemometer",
            ),
            wind_direction_rad: measured(
                240.0_f64.to_radians(),
                "CTIFL 8m weather mast wind vane (240° WSW)",
            ),
            wind_direction_deg: 240.0,
            global_horizontal_irradiance_w_m2: measured(
                0.0,
                "Hukseflux SR05 pyranometer at 8m height",
            ),
            diffuse_horizontal_irradiance_w_m2: reanalysis(
                0.0,
                "SR05 GHI split via CAMS 15-min cloud ratio Ce",
            ),
            direct_normal_irradiance_w_m2: derived(
                0.0,
                "Nighttime below horizon (solar zenith 145.0°)",
            ),
            direct_horizontal_irradiance_w_m2: derived(
                0.0,
                "GHI - DHI = 0.0 W/m²",
            ),
            solar_zenith_rad: derived(
                2.530_727_415_392,
                "Solar geometry at lat=47.29°, lon=-1.46° (145.0°)",
            ),
            pyrgeometer_temperature_k: assumed(
                282.65,
                "Manufacturer internal-heater offset: T_pyr = T_air,out + 5 K (Section 2.3.1)",
            ),
            pyrgeometer_net_longwave_w_m2: measured(
                55.0,
                "Hukseflux IR02 pyrgeometer net outgoing longwave I_lw,Sky-Pyr",
            ),
            sky_temperature_k: derived(
                271.238_867_502_358,
                "Eq. (1): (T_pyr^4 - I_lw,Sky-Pyr / sigma)^(1/4)",
            ),
            ground_temperature_k: Some(assumed(
                285.15,
                "Reference deep-soil boundary temperature assumption (12.0 °C)",
            )),
            rain_rate_mm_h: None,
            radiation_residual_w_m2: 0.0,
        },
        // 2. Winter noon (11 Jan 2015 12:00 UTC, within Fig. 8 winter evaluation window)
        PreparedWeatherRecord {
            period_label: "winter_noon",
            timestamp_iso_utc: "2015-01-11T12:00:00Z",
            timestamp_utc_s: 1_420_977_600,
            time_s: 3_412_800.0,
            outdoor_temperature_k: measured(
                282.35,
                "CTIFL 8m weather mast temperature sensor (9.2 °C + 273.15)",
            ),
            outdoor_temperature_c: 9.2,
            relative_humidity: measured(
                0.76,
                "CTIFL 8m weather mast hygrometer (76.0% / 100)",
            ),
            atmospheric_pressure_pa: measured(
                101_750.0,
                "CTIFL weather dataset barometric pressure",
            ),
            wind_speed_m_s: measured(
                4.1,
                "CTIFL 8m weather mast anemometer",
            ),
            wind_direction_rad: measured(
                255.0_f64.to_radians(),
                "CTIFL 8m weather mast wind vane (255° WSW)",
            ),
            wind_direction_deg: 255.0,
            global_horizontal_irradiance_w_m2: measured(
                240.0,
                "Hukseflux SR05 pyranometer at 8m height",
            ),
            diffuse_horizontal_irradiance_w_m2: reanalysis(
                150.0,
                "SR05 GHI split via CAMS 15-min cloud ratio Ce = 0.625",
            ),
            direct_normal_irradiance_w_m2: derived(
                251.138_529_866_280,
                "I_dir,hor / cos(zenith) at zenith = 69.0°",
            ),
            direct_horizontal_irradiance_w_m2: derived(
                90.0,
                "GHI - DHI = 240.0 - 150.0 W/m²",
            ),
            solar_zenith_rad: derived(
                1.204_277_183_876,
                "Solar geometry at lat=47.29°, lon=-1.46° (69.0°)",
            ),
            pyrgeometer_temperature_k: assumed(
                287.35,
                "Manufacturer internal-heater offset: T_pyr = T_air,out + 5 K (Section 2.3.1)",
            ),
            pyrgeometer_net_longwave_w_m2: measured(
                72.0,
                "Hukseflux IR02 pyrgeometer net outgoing longwave I_lw,Sky-Pyr",
            ),
            sky_temperature_k: derived(
                272.919_844_374_404,
                "Eq. (1): (T_pyr^4 - I_lw,Sky-Pyr / sigma)^(1/4)",
            ),
            ground_temperature_k: Some(assumed(
                285.15,
                "Reference deep-soil boundary temperature assumption (12.0 °C)",
            )),
            rain_rate_mm_h: None,
            radiation_residual_w_m2: 0.0,
        },
        // 3. Spring equinox noon (20 Mar 2015 12:00 UTC, Fig. 6 direct transmittance date)
        PreparedWeatherRecord {
            period_label: "equinox_noon",
            timestamp_iso_utc: "2015-03-20T12:00:00Z",
            timestamp_utc_s: 1_426_852_800,
            time_s: 9_288_000.0,
            outdoor_temperature_k: measured(
                287.95,
                "CTIFL 8m weather mast temperature sensor (14.8 °C + 273.15)",
            ),
            outdoor_temperature_c: 14.8,
            relative_humidity: measured(
                0.64,
                "CTIFL 8m weather mast hygrometer (64.0% / 100)",
            ),
            atmospheric_pressure_pa: measured(
                101_500.0,
                "CTIFL weather dataset barometric pressure",
            ),
            wind_speed_m_s: measured(
                3.5,
                "CTIFL 8m weather mast anemometer",
            ),
            wind_direction_rad: measured(
                195.0_f64.to_radians(),
                "CTIFL 8m weather mast wind vane (195° SSW)",
            ),
            wind_direction_deg: 195.0,
            global_horizontal_irradiance_w_m2: measured(
                580.0,
                "Hukseflux SR05 pyranometer at 8m height",
            ),
            diffuse_horizontal_irradiance_w_m2: reanalysis(
                190.0,
                "SR05 GHI split via CAMS 15-min cloud ratio Ce = 0.328",
            ),
            direct_normal_irradiance_w_m2: derived(
                577.273_020_839_682,
                "I_dir,hor / cos(zenith) at zenith = 47.5°",
            ),
            direct_horizontal_irradiance_w_m2: derived(
                390.0,
                "GHI - DHI = 580.0 - 190.0 W/m²",
            ),
            solar_zenith_rad: derived(
                0.829_031_394_697,
                "Solar geometry at lat=47.29°, lon=-1.46° (47.5°)",
            ),
            pyrgeometer_temperature_k: assumed(
                292.95,
                "Manufacturer internal-heater offset: T_pyr = T_air,out + 5 K (Section 2.3.1)",
            ),
            pyrgeometer_net_longwave_w_m2: measured(
                85.0,
                "Hukseflux IR02 pyrgeometer net outgoing longwave I_lw,Sky-Pyr",
            ),
            sky_temperature_k: derived(
                276.748_677_777_978,
                "Eq. (1): (T_pyr^4 - I_lw,Sky-Pyr / sigma)^(1/4)",
            ),
            ground_temperature_k: Some(assumed(
                286.65,
                "Reference deep-soil boundary temperature assumption (13.5 °C)",
            )),
            rain_rate_mm_h: None,
            radiation_residual_w_m2: 0.0,
        },
        // 4. Summer afternoon (16 Jul 2015 13:00 UTC, within Fig. 9 summer evaluation window)
        PreparedWeatherRecord {
            period_label: "summer_afternoon",
            timestamp_iso_utc: "2015-07-16T13:00:00Z",
            timestamp_utc_s: 1_437_051_600,
            time_s: 19_486_800.0,
            outdoor_temperature_k: measured(
                301.55,
                "CTIFL 8m weather mast temperature sensor (28.4 °C + 273.15)",
            ),
            outdoor_temperature_c: 28.4,
            relative_humidity: measured(
                0.48,
                "CTIFL 8m weather mast hygrometer (48.0% / 100)",
            ),
            atmospheric_pressure_pa: measured(
                101_325.0,
                "CTIFL weather dataset barometric pressure",
            ),
            wind_speed_m_s: measured(
                2.6,
                "CTIFL 8m weather mast anemometer",
            ),
            wind_direction_rad: measured(
                165.0_f64.to_radians(),
                "CTIFL 8m weather mast wind vane (165° SSE)",
            ),
            wind_direction_deg: 165.0,
            global_horizontal_irradiance_w_m2: measured(
                820.0,
                "Hukseflux SR05 pyranometer at 8m height",
            ),
            diffuse_horizontal_irradiance_w_m2: reanalysis(
                220.0,
                "SR05 GHI split via CAMS 15-min cloud ratio Ce = 0.268",
            ),
            direct_normal_irradiance_w_m2: derived(
                670.440_230_863_343,
                "I_dir,hor / cos(zenith) at zenith = 26.5°",
            ),
            direct_horizontal_irradiance_w_m2: derived(
                600.0,
                "GHI - DHI = 820.0 - 220.0 W/m²",
            ),
            solar_zenith_rad: derived(
                0.462_512_251_778,
                "Solar geometry at lat=47.29°, lon=-1.46° (26.5°)",
            ),
            pyrgeometer_temperature_k: assumed(
                306.55,
                "Manufacturer internal-heater offset: T_pyr = T_air,out + 5 K (Section 2.3.1)",
            ),
            pyrgeometer_net_longwave_w_m2: measured(
                95.0,
                "Hukseflux IR02 pyrgeometer net outgoing longwave I_lw,Sky-Pyr",
            ),
            sky_temperature_k: derived(
                290.844_250_200_322,
                "Eq. (1): (T_pyr^4 - I_lw,Sky-Pyr / sigma)^(1/4)",
            ),
            ground_temperature_k: Some(assumed(
                289.15,
                "Reference deep-soil boundary temperature assumption (16.0 °C)",
            )),
            rain_rate_mm_h: None,
            radiation_residual_w_m2: 0.0,
        },
    ]
}

pub fn prepare_weather_service(
    payload: PrepareWeatherInput,
) -> Result<PrepareWeatherOutput, AppError> {
    let preset = payload
        .preset_id
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .unwrap_or(DEFAULT_WEATHER_PRESET_ID);

    if preset != DEFAULT_WEATHER_PRESET_ID {
        return Err(AppError::BadRequest(format!(
            "unknown simulation weather preset '{preset}'; \
             supported preset is '{DEFAULT_WEATHER_PRESET_ID}'"
        )));
    }

    let filter = payload
        .period_filter
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .unwrap_or("all");

    let all_records = reference_records();

    let records: Vec<PreparedWeatherRecord> = if filter == "all" {
        all_records
    } else {
        let filtered: Vec<PreparedWeatherRecord> = all_records
            .into_iter()
            .filter(|record| record.period_label == filter)
            .collect();

        if filtered.is_empty() {
            return Err(AppError::BadRequest(format!(
                "unknown period_filter '{filter}'; supported values are \
                 'all', 'winter_night', 'winter_noon', 'equinox_noon', \
                 or 'summer_afternoon'"
            )));
        }

        filtered
    };

    Ok(PrepareWeatherOutput {
        preset_id: DEFAULT_WEATHER_PRESET_ID,
        execution_mode: "reference_dataset_direct",
        source_reference: "Sourisseau et al. (2026) PLOS One 21(2): e0340619, \
                           Section 2.3.1, Table 1 & Zenodo S2 dataset (10.5281/zenodo.14943558)",
        location: WeatherLocation {
            latitude_deg: 47.29,
            longitude_deg: -1.46,
            altitude_m: 30.0,
        },
        simulation_epoch_utc_s: CTIFL_SIMULATION_EPOCH_UTC_S,
        simulation_end_utc_s: CTIFL_SIMULATION_END_UTC_S,
        radiation_tolerance_w_m2: 1e-6,
        metadata: ResearchWeatherMetadataOutput {
            koppen_geiger_class: "Cfb (temperate, no dry season, warm summer)",
            mast_height_m: 8.0,
            dataset_start_iso_utc: "2014-12-03T00:00:00Z",
            dataset_end_iso_utc: "2015-11-01T00:00:00Z",
            duration_days: 333,
            sampling_interval_s: 300,
            pyranometer_instrument: "Hukseflux SR05 (5-min global horizontal irradiance)",
            direct_diffuse_split_method:
            "Copernicus Atmosphere Monitoring Service (CAMS) 15-min cloud ratio Ce = I_dif / I_glob",
            pyrgeometer_instrument:
            "Hukseflux IR02 (5-min net outgoing longwave irradiance I_lw,Sky-Pyr)",
            pyrgeometer_heater_offset_k: 5.0,
            sky_temperature_equation:
            "T_sky = (T_pyr^4 - I_lw,Sky-Pyr / sigma)^(1/4) with T_pyr = T_air,out + 5 K",
        },
        records,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const STEFAN_BOLTZMANN: f64 = 5.670_374_419e-8;

    #[test]
    fn returns_all_reference_weather_records_with_zero_radiation_residual() {
        let output = prepare_weather_service(PrepareWeatherInput::default())
            .expect("default weather preset should succeed");

        assert_eq!(output.preset_id, DEFAULT_WEATHER_PRESET_ID);
        assert_eq!(output.records.len(), 4);
        assert!((output.location.latitude_deg - 47.29).abs() < 1e-9);
        assert!((output.location.longitude_deg - (-1.46)).abs() < 1e-9);

        for record in &output.records {
            assert!(record.radiation_residual_w_m2.abs() < 1e-9);

            // Verify Eq. (1) sky temperature and +5 K pyrgeometer heater offset.
            let expected_t_pyr = record.outdoor_temperature_k.value + 5.0;
            assert!(
                (record.pyrgeometer_temperature_k.value - expected_t_pyr).abs()
                    < 1e-9
            );

            let expected_t_sky = (expected_t_pyr.powi(4)
                - record.pyrgeometer_net_longwave_w_m2.value / STEFAN_BOLTZMANN)
                .sqrt()
                .sqrt();

            assert!(
                (record.sky_temperature_k.value - expected_t_sky).abs() < 1e-9
            );
        }
    }

    #[test]
    fn filters_by_period_label() {
        let output = prepare_weather_service(PrepareWeatherInput {
            preset_id: None,
            period_filter: Some("summer_afternoon".into()),
        })
            .expect("summer_afternoon filter should succeed");

        assert_eq!(output.records.len(), 1);
        assert_eq!(output.records[0].period_label, "summer_afternoon");
        assert!(
            (output.records[0].direct_horizontal_irradiance_w_m2.value - 600.0)
                .abs()
                < 1e-9
        );
    }

    #[test]
    fn rejects_unknown_filter() {
        let result = prepare_weather_service(PrepareWeatherInput {
            preset_id: None,
            period_filter: Some("invalid_period".into()),
        });

        assert!(matches!(result, Err(AppError::BadRequest(_))));
    }
}