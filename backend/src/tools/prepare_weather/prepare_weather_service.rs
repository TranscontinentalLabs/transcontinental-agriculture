use serde::{Deserialize, Serialize};

use crate::error::AppError;

/// SI Stefan–Boltzmann constant, W/(m² K⁴).
/// Keep this centralized so we can match the reference model's
/// numerical constant if its value differs.
const STEFAN_BOLTZMANN: f64 = 5.670_374_419e-8;

/// Largest integer exactly representable by f64.
const MAX_EXACT_INTEGER: i64 = 9_007_199_254_740_992;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum DataOrigin {
    Measured,
    Reanalysis,
    Estimated,
    Assumed,
    Derived,
}

/// Each supplied quantity carries its own provenance.
///
/// `source` should identify the dataset, column, instrument,
/// assumption, or calculation method.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct WeatherValue {
    pub value: f64,
    pub origin: DataOrigin,
    pub source: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct WeatherLocation {
    /// North positive, south negative.
    pub latitude_deg: f64,

    /// East positive, west negative.
    pub longitude_deg: f64,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "method", rename_all = "snake_case")]
pub enum SkyTemperatureInput {
    /// An already calculated effective sky temperature.
    EffectiveTemperature {
        temperature_k: WeatherValue,
    },

    /// Downward hemispherical longwave irradiance.
    ///
    /// T_sky = (L_down / sigma)^(1/4)
    DownwardLongwave {
        downward_longwave_w_m2: WeatherValue,
    },

    /// Research equation with the instrument's net convention.
    ///
    /// T_sky^4 = T_pyr^4 - I_net / sigma
    ///
    /// I_net is positive when the pyrgeometer loses net radiation
    /// toward the sky. Do not supply downward longwave here.
    PyrgeometerNet {
        pyrgeometer_temperature_k: WeatherValue,
        net_longwave_w_m2: WeatherValue,
    },
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct WeatherRecordInput {
    /// UTC Unix timestamp, seconds.
    pub timestamp_utc_s: i64,

    pub outdoor_temperature_c: WeatherValue,
    pub relative_humidity_percent: WeatherValue,
    pub atmospheric_pressure_pa: WeatherValue,

    pub wind_speed_m_s: WeatherValue,

    /// Meteorological convention: direction the wind comes FROM.
    /// North=0°, east=90°, south=180°, west=270°.
    pub wind_direction_deg: WeatherValue,

    pub global_horizontal_irradiance_w_m2: WeatherValue,
    pub diffuse_horizontal_irradiance_w_m2: WeatherValue,
    pub direct_normal_irradiance_w_m2: WeatherValue,

    /// Geometric solar zenith, radians, between 0 and π.
    pub solar_zenith_rad: WeatherValue,

    pub sky: SkyTemperatureInput,

    /// Optional deep-ground boundary temperature.
    /// Its depth and interpretation must be documented in `source`.
    pub ground_temperature_c: Option<WeatherValue>,

    /// Optional rainfall rate, not accumulated rainfall.
    pub rain_rate_mm_h: Option<WeatherValue>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PrepareWeatherInput {
    pub location: WeatherLocation,

    /// UTC timestamp corresponding to simulation time zero.
    ///
    /// This is not necessarily the first weather record:
    /// earlier records may be needed for interpolation.
    pub simulation_epoch_utc_s: i64,

    /// Allowed absolute disagreement in the radiation identity.
    ///
    /// Set from dataset precision/measurement uncertainty.
    /// This is explicit rather than silently hard-coded.
    pub radiation_tolerance_w_m2: f64,

    pub records: Vec<WeatherRecordInput>,
}

#[derive(Debug, Serialize)]
pub struct PreparedWeatherRecord {
    pub timestamp_utc_s: i64,
    pub time_s: f64,

    pub outdoor_temperature_k: WeatherValue,
    pub relative_humidity: WeatherValue,
    pub atmospheric_pressure_pa: WeatherValue,

    pub wind_speed_m_s: WeatherValue,
    pub wind_direction_rad: WeatherValue,

    pub global_horizontal_irradiance_w_m2: WeatherValue,
    pub diffuse_horizontal_irradiance_w_m2: WeatherValue,
    pub direct_normal_irradiance_w_m2: WeatherValue,
    pub direct_horizontal_irradiance_w_m2: WeatherValue,

    pub solar_zenith_rad: WeatherValue,
    pub sky_temperature_k: WeatherValue,

    pub ground_temperature_k: Option<WeatherValue>,
    pub rain_rate_mm_h: Option<WeatherValue>,

    /// GHI - (DHI + direct horizontal irradiance), W/m².
    /// Retained for inspection; measurements are not overwritten.
    pub radiation_residual_w_m2: f64,
}

#[derive(Debug, Serialize)]
pub struct PrepareWeatherOutput {
    pub location: WeatherLocation,
    pub simulation_epoch_utc_s: i64,
    pub radiation_tolerance_w_m2: f64,
    pub records: Vec<PreparedWeatherRecord>,
}

fn record_error(index: usize, message: impl Into<String>) -> AppError {
    AppError::BadRequest(format!(
        "weather record {index}: {}",
        message.into()
    ))
}

fn validate_value(
    index: usize,
    name: &str,
    quantity: &WeatherValue,
) -> Result<(), AppError> {
    if !quantity.value.is_finite() {
        return Err(record_error(
            index,
            format!("{name} must be finite"),
        ));
    }

    if quantity.source.trim().is_empty() {
        return Err(record_error(
            index,
            format!("{name} must identify its source or method"),
        ));
    }

    Ok(())
}

/// Unit conversion preserves the observation's original provenance.
fn convert_units(
    quantity: WeatherValue,
    conversion: impl FnOnce(f64) -> f64,
) -> WeatherValue {
    WeatherValue {
        value: conversion(quantity.value),
        ..quantity
    }
}

fn derived_value(value: f64, source: String) -> WeatherValue {
    WeatherValue {
        value,
        origin: DataOrigin::Derived,
        source,
    }
}

fn describe_source(quantity: &WeatherValue) -> String {
    format!("{:?}: {}", quantity.origin, quantity.source)
}

fn prepare_sky_temperature(
    index: usize,
    input: SkyTemperatureInput,
) -> Result<WeatherValue, AppError> {
    let result = match input {
        SkyTemperatureInput::EffectiveTemperature {
            temperature_k,
        } => {
            validate_value(index, "sky temperature", &temperature_k)?;
            temperature_k
        }

        SkyTemperatureInput::DownwardLongwave {
            downward_longwave_w_m2,
        } => {
            validate_value(
                index,
                "downward longwave",
                &downward_longwave_w_m2,
            )?;

            if downward_longwave_w_m2.value <= 0.0 {
                return Err(record_error(
                    index,
                    "downward longwave must be positive",
                ));
            }

            let temperature_k =
                (downward_longwave_w_m2.value / STEFAN_BOLTZMANN)
                    .sqrt()
                    .sqrt();

            derived_value(
                temperature_k,
                format!(
                    "T_sky=(L_down/sigma)^(1/4); input [{}]",
                    describe_source(&downward_longwave_w_m2)
                ),
            )
        }

        SkyTemperatureInput::PyrgeometerNet {
            pyrgeometer_temperature_k,
            net_longwave_w_m2,
        } => {
            validate_value(
                index,
                "pyrgeometer temperature",
                &pyrgeometer_temperature_k,
            )?;

            validate_value(
                index,
                "net longwave",
                &net_longwave_w_m2,
            )?;

            if pyrgeometer_temperature_k.value <= 0.0 {
                return Err(record_error(
                    index,
                    "pyrgeometer temperature must be above 0 K",
                ));
            }

            // Net longwave is signed: negative values are allowed.
            let sky_temperature_fourth_power =
                pyrgeometer_temperature_k.value.powi(4)
                    - net_longwave_w_m2.value / STEFAN_BOLTZMANN;

            if !sky_temperature_fourth_power.is_finite()
                || sky_temperature_fourth_power <= 0.0
            {
                return Err(record_error(
                    index,
                    "pyrgeometer inputs produce an invalid sky temperature; \
                     check units and the net-radiation sign convention",
                ));
            }

            derived_value(
                sky_temperature_fourth_power.sqrt().sqrt(),
                format!(
                    "T_sky=(T_pyr^4-I_net/sigma)^(1/4); \
                     T_pyr [{}]; I_net [{}]",
                    describe_source(&pyrgeometer_temperature_k),
                    describe_source(&net_longwave_w_m2)
                ),
            )
        }
    };

    if !result.value.is_finite() || result.value <= 0.0 {
        return Err(record_error(
            index,
            "sky temperature must be finite and above 0 K",
        ));
    }

    Ok(result)
}

pub fn prepare_weather(
    payload: PrepareWeatherInput,
) -> Result<PrepareWeatherOutput, AppError> {
    let PrepareWeatherInput {
        location,
        simulation_epoch_utc_s,
        radiation_tolerance_w_m2,
        records: input_records,
    } = payload;

    if !location.latitude_deg.is_finite()
        || !(-90.0..=90.0).contains(&location.latitude_deg)
    {
        return Err(AppError::BadRequest(
            "latitude_deg must be finite and between -90 and 90".into(),
        ));
    }

    if !location.longitude_deg.is_finite()
        || !(-180.0..=180.0).contains(&location.longitude_deg)
    {
        return Err(AppError::BadRequest(
            "longitude_deg must be finite and between -180 and 180".into(),
        ));
    }

    if !radiation_tolerance_w_m2.is_finite()
        || radiation_tolerance_w_m2 < 0.0
    {
        return Err(AppError::BadRequest(
            "radiation_tolerance_w_m2 must be finite and non-negative"
                .into(),
        ));
    }

    if input_records.is_empty() {
        return Err(AppError::BadRequest(
            "at least one weather record is required".into(),
        ));
    }

    let mut records = Vec::with_capacity(input_records.len());
    let mut previous_timestamp: Option<i64> = None;

    for (index, record) in input_records.into_iter().enumerate() {
        if let Some(previous) = previous_timestamp {
            if record.timestamp_utc_s <= previous {
                return Err(record_error(
                    index,
                    "timestamps must be strictly increasing",
                ));
            }
        }

        previous_timestamp = Some(record.timestamp_utc_s);

        let elapsed_s = record
            .timestamp_utc_s
            .checked_sub(simulation_epoch_utc_s)
            .ok_or_else(|| {
                record_error(index, "timestamp difference overflows")
            })?;

        if !(-MAX_EXACT_INTEGER..=MAX_EXACT_INTEGER).contains(&elapsed_s) {
            return Err(record_error(
                index,
                "timestamp difference cannot be represented exactly",
            ));
        }

        let quantities = [
            ("outdoor_temperature_c", &record.outdoor_temperature_c),
            (
                "relative_humidity_percent",
                &record.relative_humidity_percent,
            ),
            (
                "atmospheric_pressure_pa",
                &record.atmospheric_pressure_pa,
            ),
            ("wind_speed_m_s", &record.wind_speed_m_s),
            ("wind_direction_deg", &record.wind_direction_deg),
            (
                "global_horizontal_irradiance_w_m2",
                &record.global_horizontal_irradiance_w_m2,
            ),
            (
                "diffuse_horizontal_irradiance_w_m2",
                &record.diffuse_horizontal_irradiance_w_m2,
            ),
            (
                "direct_normal_irradiance_w_m2",
                &record.direct_normal_irradiance_w_m2,
            ),
            ("solar_zenith_rad", &record.solar_zenith_rad),
        ];

        for (name, quantity) in quantities {
            validate_value(index, name, quantity)?;
        }

        if record.outdoor_temperature_c.value <= -273.15 {
            return Err(record_error(
                index,
                "outdoor temperature must be above absolute zero",
            ));
        }

        if !(0.0..=100.0).contains(&record.relative_humidity_percent.value) {
            return Err(record_error(
                index,
                "relative humidity must be between 0 and 100 percent",
            ));
        }

        if record.atmospheric_pressure_pa.value <= 0.0 {
            return Err(record_error(
                index,
                "atmospheric pressure must be positive",
            ));
        }

        if record.wind_speed_m_s.value < 0.0 {
            return Err(record_error(
                index,
                "wind speed must be non-negative",
            ));
        }

        if !(0.0..360.0).contains(&record.wind_direction_deg.value) {
            return Err(record_error(
                index,
                "wind direction must be in [0, 360) degrees",
            ));
        }

        let zenith = record.solar_zenith_rad.value;

        if !(0.0..=std::f64::consts::PI).contains(&zenith) {
            return Err(record_error(
                index,
                "solar zenith must be between 0 and pi radians",
            ));
        }

        let ghi = record.global_horizontal_irradiance_w_m2.value;
        let dhi = record.diffuse_horizontal_irradiance_w_m2.value;
        let dni = record.direct_normal_irradiance_w_m2.value;

        if ghi < 0.0 || dhi < 0.0 || dni < 0.0 {
            return Err(record_error(
                index,
                "solar irradiance values must be non-negative",
            ));
        }

        let sun_above_horizon = zenith < std::f64::consts::FRAC_PI_2;

        if !sun_above_horizon && dni > radiation_tolerance_w_m2 {
            return Err(record_error(
                index,
                "direct normal irradiance is positive below the horizon",
            ));
        }

        // A below-horizon Sun contributes no direct horizontal flux.
        let direct_horizontal = if sun_above_horizon {
            dni * zenith.cos()
        } else {
            0.0
        };

        let reconstructed_ghi = dhi + direct_horizontal;
        let residual = ghi - reconstructed_ghi;

        if !reconstructed_ghi.is_finite()
            || !residual.is_finite()
            || residual.abs() > radiation_tolerance_w_m2
        {
            return Err(record_error(
                index,
                format!(
                    "radiation components disagree: \
                     GHI - (DHI + direct horizontal) = {residual} W/m²"
                ),
            ));
        }

        let direct_horizontal_irradiance_w_m2 = derived_value(
            direct_horizontal,
            format!(
                "DNI*cos(zenith) above the horizon, otherwise zero; \
                 DNI [{}]; zenith [{}]",
                describe_source(&record.direct_normal_irradiance_w_m2),
                describe_source(&record.solar_zenith_rad)
            ),
        );

        let sky_temperature_k = prepare_sky_temperature(index, record.sky)?;

        let ground_temperature_k = match record.ground_temperature_c {
            Some(value) => {
                validate_value(index, "ground_temperature_c", &value)?;

                if value.value <= -273.15 {
                    return Err(record_error(
                        index,
                        "ground temperature must be above absolute zero",
                    ));
                }

                Some(convert_units(value, |c| c + 273.15))
            }
            None => None,
        };

        if let Some(rain) = &record.rain_rate_mm_h {
            validate_value(index, "rain_rate_mm_h", rain)?;

            if rain.value < 0.0 {
                return Err(record_error(
                    index,
                    "rain rate must be non-negative",
                ));
            }
        }

        records.push(PreparedWeatherRecord {
            timestamp_utc_s: record.timestamp_utc_s,
            time_s: elapsed_s as f64,

            outdoor_temperature_k: convert_units(
                record.outdoor_temperature_c,
                |c| c + 273.15,
            ),
            relative_humidity: convert_units(
                record.relative_humidity_percent,
                |percent| percent / 100.0,
            ),
            atmospheric_pressure_pa: record.atmospheric_pressure_pa,

            wind_speed_m_s: record.wind_speed_m_s,
            wind_direction_rad: convert_units(
                record.wind_direction_deg,
                |degrees| degrees.to_radians(),
            ),

            global_horizontal_irradiance_w_m2:
            record.global_horizontal_irradiance_w_m2,
            diffuse_horizontal_irradiance_w_m2:
            record.diffuse_horizontal_irradiance_w_m2,
            direct_normal_irradiance_w_m2:
            record.direct_normal_irradiance_w_m2,
            direct_horizontal_irradiance_w_m2,

            solar_zenith_rad: record.solar_zenith_rad,
            sky_temperature_k,

            ground_temperature_k,
            rain_rate_mm_h: record.rain_rate_mm_h,

            radiation_residual_w_m2: residual,
        });
    }

    Ok(PrepareWeatherOutput {
        location,
        simulation_epoch_utc_s,
        radiation_tolerance_w_m2,
        records,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn measured(value: f64) -> WeatherValue {
        WeatherValue {
            value,
            origin: DataOrigin::Measured,
            source: "synthetic test fixture".into(),
        }
    }

    #[test]
    fn zero_net_longwave_gives_pyrgeometer_temperature() {
        let result = prepare_sky_temperature(
            0,
            SkyTemperatureInput::PyrgeometerNet {
                pyrgeometer_temperature_k: measured(300.0),
                net_longwave_w_m2: measured(0.0),
            },
        )
            .expect("valid sky inputs");

        assert!((result.value - 300.0).abs() < 1e-10);
    }

    #[test]
    fn positive_outgoing_net_longwave_gives_colder_sky() {
        let result = prepare_sky_temperature(
            0,
            SkyTemperatureInput::PyrgeometerNet {
                pyrgeometer_temperature_k: measured(300.0),
                net_longwave_w_m2: measured(100.0),
            },
        )
            .expect("valid sky inputs");

        assert!(result.value > 0.0);
        assert!(result.value < 300.0);
    }

    #[test]
    fn rejects_impossible_pyrgeometer_balance() {
        let result = prepare_sky_temperature(
            0,
            SkyTemperatureInput::PyrgeometerNet {
                pyrgeometer_temperature_k: measured(300.0),
                net_longwave_w_m2: measured(1000.0),
            },
        );

        assert!(matches!(result, Err(AppError::BadRequest(_))));
    }

    #[test]
    fn converts_direct_normal_to_direct_horizontal() {
        let result = prepare_weather(PrepareWeatherInput {
            location: WeatherLocation {
                latitude_deg: 0.0,
                longitude_deg: 0.0,
            },
            simulation_epoch_utc_s: 0,
            radiation_tolerance_w_m2: 1e-9,
            records: vec![WeatherRecordInput {
                timestamp_utc_s: 3600,
                outdoor_temperature_c: measured(20.0),
                relative_humidity_percent: measured(75.0),
                atmospheric_pressure_pa: measured(101_325.0),
                wind_speed_m_s: measured(2.0),
                wind_direction_deg: measured(90.0),
                global_horizontal_irradiance_w_m2: measured(500.0),
                diffuse_horizontal_irradiance_w_m2: measured(100.0),
                direct_normal_irradiance_w_m2: measured(800.0),
                solar_zenith_rad: measured(
                    std::f64::consts::PI / 3.0,
                ),
                sky: SkyTemperatureInput::EffectiveTemperature {
                    temperature_k: measured(280.0),
                },
                ground_temperature_c: None,
                rain_rate_mm_h: None,
            }],
        })
            .expect("valid synthetic weather");

        let record = &result.records[0];

        assert_eq!(record.time_s, 3600.0);
        assert!((record.outdoor_temperature_k.value - 293.15).abs() < 1e-10);
        assert!((record.relative_humidity.value - 0.75).abs() < 1e-10);

        // DNI=800 at zenith=60° gives direct horizontal=400.
        assert!(
            (record.direct_horizontal_irradiance_w_m2.value - 400.0)
                .abs()
                < 1e-9
        );

        assert!(record.radiation_residual_w_m2.abs() < 1e-9);
        assert!(record.ground_temperature_k.is_none());
        assert!(record.rain_rate_mm_h.is_none());
    }
}