use serde::{Deserialize, Serialize};

use crate::error::AppError;

#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GreenhouseConfigurationInput {
    /// Greenhouse length, metres.
    pub length: f64,

    /// Total greenhouse width across all spans, metres.
    pub width: f64,

    /// Cultivated fraction of total floor area, between 0 and 1.
    pub r_cul: f64,

    /// Gutter height above the floor, metres.
    pub h_gutter: f64,

    /// Ridge height above the floor, metres.
    pub h_ridge: f64,
}

#[derive(Debug, Clone, Copy, Serialize)]
pub struct GreenhouseConfigurationOutput {
    /// Total greenhouse floor area, m².
    pub floor_area_m2: f64,

    /// Cultivated floor area, m².
    pub cultivated_area_m2: f64,

    /// Whole-greenhouse enclosed geometric volume, m³.
    pub volume_m3: f64,
}

pub fn get_greenhouse_configuration(
    payload: GreenhouseConfigurationInput,
) -> Result<GreenhouseConfigurationOutput, AppError> {
    let fields = [
        ("length", payload.length),
        ("width", payload.width),
        ("r_cul", payload.r_cul),
        ("h_gutter", payload.h_gutter),
        ("h_ridge", payload.h_ridge),
    ];

    for (name, value) in fields {
        if !value.is_finite() {
            return Err(AppError::BadRequest(format!(
                "{name} must be a finite number"
            )));
        }
    }

    if payload.length <= 0.0 {
        return Err(AppError::BadRequest(
            "length must be greater than zero metres".into(),
        ));
    }

    if payload.width <= 0.0 {
        return Err(AppError::BadRequest(
            "width must be greater than zero metres".into(),
        ));
    }

    if payload.h_gutter <= 0.0 {
        return Err(AppError::BadRequest(
            "h_gutter must be greater than zero metres".into(),
        ));
    }

    // This tool currently represents a pitched-roof greenhouse.
    if payload.h_ridge <= payload.h_gutter {
        return Err(AppError::BadRequest(
            "h_ridge must be greater than h_gutter".into(),
        ));
    }

    if !(0.0..=1.0).contains(&payload.r_cul) {
        return Err(AppError::BadRequest(
            "r_cul must be between 0 and 1".into(),
        ));
    }

    let floor_area_m2 = payload.length * payload.width;

    let cultivated_area_m2 = payload.r_cul * floor_area_m2;

    let roof_height_m = payload.h_ridge - payload.h_gutter;

    let mean_height_m = payload.h_gutter + roof_height_m / 2.0;

    let volume_m3 = floor_area_m2 * mean_height_m;

    // Finite inputs can still overflow or underflow during arithmetic.
    if !floor_area_m2.is_finite()
        || floor_area_m2 <= 0.0
        || !cultivated_area_m2.is_finite()
        || (payload.r_cul > 0.0 && cultivated_area_m2 <= 0.0)
        || !volume_m3.is_finite()
        || volume_m3 <= 0.0
    {
        return Err(AppError::BadRequest(
            "dimensions produce areas or volume outside the supported numeric range"
                .into(),
        ));
    }

    Ok(GreenhouseConfigurationOutput {
        floor_area_m2,
        cultivated_area_m2,
        volume_m3,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn reference_input() -> GreenhouseConfigurationInput {
        GreenhouseConfigurationInput {
            length: 43.2,
            width: 24.0,
            r_cul: 0.91,
            h_gutter: 6.97,
            h_ridge: 7.78,
        }
    }

    #[test]
    fn calculates_reference_geometry() {
        let result = get_greenhouse_configuration(reference_input())
            .expect("reference geometry should be valid");

        assert!((result.floor_area_m2 - 1036.8).abs() < 1e-9);
        assert!((result.cultivated_area_m2 - 943.488).abs() < 1e-9);
        assert!((result.volume_m3 - 7646.4).abs() < 1e-9);
    }

    #[test]
    fn rejects_invalid_geometry() {
        let reference = reference_input();

        let invalid_inputs = [
            GreenhouseConfigurationInput {
                length: 0.0,
                ..reference
            },
            GreenhouseConfigurationInput {
                width: f64::NAN,
                ..reference
            },
            GreenhouseConfigurationInput {
                r_cul: 1.1,
                ..reference
            },
            GreenhouseConfigurationInput {
                h_ridge: reference.h_gutter,
                ..reference
            },
            GreenhouseConfigurationInput {
                length: f64::MAX,
                ..reference
            },
        ];

        for input in invalid_inputs {
            assert!(matches!(
                get_greenhouse_configuration(input),
                Err(AppError::BadRequest(_))
            ));
        }
    }

    #[test]
    fn allows_an_uncultivated_greenhouse() {
        let result = get_greenhouse_configuration(
            GreenhouseConfigurationInput {
                r_cul: 0.0,
                ..reference_input()
            },
        )
            .expect("an empty greenhouse still has valid geometry");

        assert_eq!(result.cultivated_area_m2, 0.0);
        assert!(result.volume_m3 > 0.0);
    }
}