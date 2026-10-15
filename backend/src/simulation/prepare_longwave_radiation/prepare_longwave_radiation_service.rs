use serde::{Deserialize, Serialize};

use crate::error::AppError;

pub const DEFAULT_LONGWAVE_RADIATION_PRESET_ID: &str = "ctifl_nantes_2014_2015";

/// SI Stefan–Boltzmann constant sigma, W/(m²·K⁴), matching Model 3.
pub const STEFAN_BOLTZMANN_W_M2_K4: f64 = 5.670_374_419e-8;

pub const FLOOR_AREA_M2: f64 = 1_036.8;
pub const CULTIVATED_AREA_M2: f64 = 943.488;
pub const HEATING_PIPE_SURFACE_AREA_M2: f64 = 235.0;

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PrepareLongwaveRadiationInput {
    /// Optional preset identifier. Defaults to `"ctifl_nantes_2014_2015"`.
    pub preset_id: Option<String>,

    /// Optional filter matching Models 3–12 period labels:
    /// `"winter_night"`, `"winter_noon"`, `"equinox_noon"`, `"summer_afternoon"`, or `"all"`.
    pub period_filter: Option<String>,
}

#[derive(Debug, Clone, Copy, Serialize)]
pub struct LongwaveNetworkParametersOutput {
    pub stefan_boltzmann_w_m2_k4: f64,
    pub roof_glass_emissivity: f64,
    pub canopy_leaf_emissivity: f64,
    pub floor_sheet_emissivity: f64,
    pub heating_pipe_emissivity: f64,
    pub thermal_screen_material_tau_lw: f64,
    pub shading_screen_material_tau_lw: f64,
    pub view_factor_methodology: &'static str,
}

#[derive(Debug, Clone, Copy, Serialize)]
pub struct LongwaveRadiationPairOutput {
    pub surface_1_name: &'static str,
    pub surface_2_name: &'static str,

    pub area_1_m2: f64,
    pub emissivity_1: f64,
    pub reflectance_1: f64,
    pub emissivity_2: f64,
    pub reflectance_2: f64,

    pub view_factor_f12: f64,
    pub view_factor_f21: f64,

    /// Product of longwave transmittances of intervening layers: prod_k(tau_k).
    pub intervening_tau_product: f64,

    pub temperature_1_c: f64,
    pub temperature_1_k: f64,
    pub temperature_2_c: f64,
    pub temperature_2_k: f64,

    /// Denominator of Eq. (14): 1 - rho_1 * rho_2 * F_21 * F_12 * prod_k(tau_k^2).
    pub interreflection_denominator: f64,

    /// Net radiative heat transfer Q_dot_{1->2} (Eq. 14), W.
    /// Positive means net thermal radiation flows from Surface 1 to Surface 2.
    pub net_heat_flow_1_to_2_w: f64,

    /// Equivalent flux per m² of greenhouse floor: Q_dot_{1->2} / A_floor, W/m²_floor.
    pub flux_per_floor_m2_w_m2: f64,
}

#[derive(Debug, Clone, Serialize)]
pub struct PreparedLongwaveRadiationRecord {
    pub period_label: &'static str,
    pub timestamp_iso_utc: &'static str,
    pub timestamp_utc_s: i64,
    pub time_s: f64,

    /// Measured/scheduled crop height (m) driving the PyViewFactor regressions (Section 3.5.2).
    pub crop_height_m: f64,

    pub shading_screen_deployment: f64,
    pub thermal_screen_deployment: f64,
    pub effective_screens_tau_lw_product: f64,

    pub roof_outer_to_sky: LongwaveRadiationPairOutput,
    pub canopy_to_roof_inner: LongwaveRadiationPairOutput,
    pub heating_pipes_to_canopy: LongwaveRadiationPairOutput,
    pub canopy_to_floor: LongwaveRadiationPairOutput,

    /// Counterfactual canopy-to-roof radiative heat loss if screens were retracted (prod_tau = 1), W.
    pub unscreened_canopy_to_roof_heat_flow_w: f64,

    /// Radiative cooling saved at the canopy by active screen deployment, W.
    pub screen_radiative_protection_saving_w: f64,
}

#[derive(Debug, Clone, Serialize)]
pub struct PrepareLongwaveRadiationOutput {
    pub preset_id: &'static str,
    pub execution_mode: &'static str,
    pub source_reference: &'static str,
    pub parameters: LongwaveNetworkParametersOutput,
    pub records: Vec<PreparedLongwaveRadiationRecord>,
}

struct PairSpec {
    surface_1_name: &'static str,
    surface_2_name: &'static str,
    area_1_m2: f64,
    emissivity_1: f64,
    reflectance_1: f64,
    emissivity_2: f64,
    reflectance_2: f64,
    view_factor_f12: f64,
    view_factor_f21: f64,
    intervening_tau_product: f64,
    temperature_1_c: f64,
    temperature_2_c: f64,
}

fn evaluate_pair(spec: PairSpec) -> LongwaveRadiationPairOutput {
    let temperature_1_k = spec.temperature_1_c + 273.15;
    let temperature_2_k = spec.temperature_2_c + 273.15;

    let interreflection_denominator = 1.0
        - spec.reflectance_1
        * spec.reflectance_2
        * spec.view_factor_f21
        * spec.view_factor_f12
        * spec.intervening_tau_product.powi(2);

    let numerator = spec.emissivity_1
        * spec.emissivity_2
        * spec.view_factor_f12
        * spec.area_1_m2
        * spec.intervening_tau_product
        * STEFAN_BOLTZMANN_W_M2_K4
        * (temperature_1_k.powi(4) - temperature_2_k.powi(4));

    let net_heat_flow_1_to_2_w = numerator / interreflection_denominator;
    let flux_per_floor_m2_w_m2 = net_heat_flow_1_to_2_w / FLOOR_AREA_M2;

    LongwaveRadiationPairOutput {
        surface_1_name: spec.surface_1_name,
        surface_2_name: spec.surface_2_name,
        area_1_m2: spec.area_1_m2,
        emissivity_1: spec.emissivity_1,
        reflectance_1: spec.reflectance_1,
        emissivity_2: spec.emissivity_2,
        reflectance_2: spec.reflectance_2,
        view_factor_f12: spec.view_factor_f12,
        view_factor_f21: spec.view_factor_f21,
        intervening_tau_product: spec.intervening_tau_product,
        temperature_1_c: spec.temperature_1_c,
        temperature_1_k,
        temperature_2_c: spec.temperature_2_c,
        temperature_2_k,
        interreflection_denominator,
        net_heat_flow_1_to_2_w,
        flux_per_floor_m2_w_m2,
    }
}

struct RecordSpec {
    period_label: &'static str,
    timestamp_iso_utc: &'static str,
    timestamp_utc_s: i64,
    time_s: f64,
    crop_height_m: f64,
    shading_screen_deployment: f64,
    thermal_screen_deployment: f64,
    t_roof_outer_c: f64,
    t_sky_k: f64,
    t_canopy_leaf_c: f64,
    t_roof_inner_c: f64,
    t_heating_pipe_c: f64,
    t_floor_c: f64,
    f_canopy_to_roof: f64,
    f_roof_to_canopy: f64,
    f_pipe_to_canopy: f64,
    f_canopy_to_pipe: f64,
    f_canopy_to_floor: f64,
    f_floor_to_canopy: f64,
}

fn evaluate_record(spec: RecordSpec) -> PreparedLongwaveRadiationRecord {
    let tau_thermal_eff = 1.0 - spec.thermal_screen_deployment * (1.0 - 0.18);
    let tau_shading_eff = 1.0 - spec.shading_screen_deployment * (1.0 - 0.32);
    let effective_screens_tau_lw_product = tau_thermal_eff * tau_shading_eff;

    let t_sky_c = spec.t_sky_k - 273.15;

    let roof_outer_to_sky = evaluate_pair(PairSpec {
        surface_1_name: "RoofOuter",
        surface_2_name: "SkyBlackbody",
        area_1_m2: FLOOR_AREA_M2,
        emissivity_1: 0.89,
        reflectance_1: 0.11,
        emissivity_2: 1.00,
        reflectance_2: 0.00,
        view_factor_f12: 1.00,
        view_factor_f21: 1.00,
        intervening_tau_product: 1.00,
        temperature_1_c: spec.t_roof_outer_c,
        temperature_2_c: t_sky_c,
    });

    let canopy_to_roof_inner = evaluate_pair(PairSpec {
        surface_1_name: "Canopy",
        surface_2_name: "RoofInner",
        area_1_m2: CULTIVATED_AREA_M2,
        emissivity_1: 0.96,
        reflectance_1: 0.04,
        emissivity_2: 0.89,
        reflectance_2: 0.11,
        view_factor_f12: spec.f_canopy_to_roof,
        view_factor_f21: spec.f_roof_to_canopy,
        intervening_tau_product: effective_screens_tau_lw_product,
        temperature_1_c: spec.t_canopy_leaf_c,
        temperature_2_c: spec.t_roof_inner_c,
    });

    let unscreened_canopy_to_roof = evaluate_pair(PairSpec {
        surface_1_name: "Canopy",
        surface_2_name: "RoofInner",
        area_1_m2: CULTIVATED_AREA_M2,
        emissivity_1: 0.96,
        reflectance_1: 0.04,
        emissivity_2: 0.89,
        reflectance_2: 0.11,
        view_factor_f12: spec.f_canopy_to_roof,
        view_factor_f21: spec.f_roof_to_canopy,
        intervening_tau_product: 1.00,
        temperature_1_c: spec.t_canopy_leaf_c,
        temperature_2_c: spec.t_roof_inner_c,
    });

    let heating_pipes_to_canopy = evaluate_pair(PairSpec {
        surface_1_name: "HeatingPipes",
        surface_2_name: "Canopy",
        area_1_m2: HEATING_PIPE_SURFACE_AREA_M2,
        emissivity_1: 0.91,
        reflectance_1: 0.09,
        emissivity_2: 0.96,
        reflectance_2: 0.04,
        view_factor_f12: spec.f_pipe_to_canopy,
        view_factor_f21: spec.f_canopy_to_pipe,
        intervening_tau_product: 1.00,
        temperature_1_c: spec.t_heating_pipe_c,
        temperature_2_c: spec.t_canopy_leaf_c,
    });

    let canopy_to_floor = evaluate_pair(PairSpec {
        surface_1_name: "Canopy",
        surface_2_name: "Floor",
        area_1_m2: CULTIVATED_AREA_M2,
        emissivity_1: 0.96,
        reflectance_1: 0.04,
        emissivity_2: 0.90,
        reflectance_2: 0.10,
        view_factor_f12: spec.f_canopy_to_floor,
        view_factor_f21: spec.f_floor_to_canopy,
        intervening_tau_product: 1.00,
        temperature_1_c: spec.t_canopy_leaf_c,
        temperature_2_c: spec.t_floor_c,
    });

    let unscreened_canopy_to_roof_heat_flow_w =
        unscreened_canopy_to_roof.net_heat_flow_1_to_2_w;
    let screen_radiative_protection_saving_w =
        unscreened_canopy_to_roof_heat_flow_w
            - canopy_to_roof_inner.net_heat_flow_1_to_2_w;

    PreparedLongwaveRadiationRecord {
        period_label: spec.period_label,
        timestamp_iso_utc: spec.timestamp_iso_utc,
        timestamp_utc_s: spec.timestamp_utc_s,
        time_s: spec.time_s,
        crop_height_m: spec.crop_height_m,
        shading_screen_deployment: spec.shading_screen_deployment,
        thermal_screen_deployment: spec.thermal_screen_deployment,
        effective_screens_tau_lw_product,
        roof_outer_to_sky,
        canopy_to_roof_inner,
        heating_pipes_to_canopy,
        canopy_to_floor,
        unscreened_canopy_to_roof_heat_flow_w,
        screen_radiative_protection_saving_w,
    }
}

fn reference_records() -> Vec<PreparedLongwaveRadiationRecord> {
    vec![
        // 1. Winter night (11 Jan 2015 02:00 UTC)
        evaluate_record(RecordSpec {
            period_label: "winter_night",
            timestamp_iso_utc: "2015-01-11T02:00:00Z",
            timestamp_utc_s: 1_420_941_600,
            time_s: 3_376_800.0,
            crop_height_m: 1.0,
            shading_screen_deployment: 0.0,
            thermal_screen_deployment: 1.0,
            t_roof_outer_c: 6.2,
            t_sky_k: 271.238_867_502_358,
            t_canopy_leaf_c: 16.8,
            t_roof_inner_c: 7.4,
            t_heating_pipe_c: 48.0,
            t_floor_c: 15.6,
            f_canopy_to_roof: 0.58,
            f_roof_to_canopy: 0.53,
            f_pipe_to_canopy: 0.42,
            f_canopy_to_pipe: 0.10,
            f_canopy_to_floor: 0.52,
            f_floor_to_canopy: 0.47,
        }),
        // 2. Winter noon (11 Jan 2015 12:00 UTC)
        evaluate_record(RecordSpec {
            period_label: "winter_noon",
            timestamp_iso_utc: "2015-01-11T12:00:00Z",
            timestamp_utc_s: 1_420_977_600,
            time_s: 3_412_800.0,
            crop_height_m: 1.0,
            shading_screen_deployment: 0.0,
            thermal_screen_deployment: 0.0,
            t_roof_outer_c: 13.1,
            t_sky_k: 272.919_844_374_404,
            t_canopy_leaf_c: 21.1,
            t_roof_inner_c: 14.2,
            t_heating_pipe_c: 36.0,
            t_floor_c: 21.8,
            f_canopy_to_roof: 0.58,
            f_roof_to_canopy: 0.53,
            f_pipe_to_canopy: 0.42,
            f_canopy_to_pipe: 0.10,
            f_canopy_to_floor: 0.52,
            f_floor_to_canopy: 0.47,
        }),
        // 3. Spring equinox noon (20 Mar 2015 12:00 UTC)
        evaluate_record(RecordSpec {
            period_label: "equinox_noon",
            timestamp_iso_utc: "2015-03-20T12:00:00Z",
            timestamp_utc_s: 1_426_852_800,
            time_s: 9_288_000.0,
            crop_height_m: 2.8,
            shading_screen_deployment: 0.0,
            thermal_screen_deployment: 0.0,
            t_roof_outer_c: 19.4,
            t_sky_k: 276.748_677_777_978,
            t_canopy_leaf_c: 24.5,
            t_roof_inner_c: 20.1,
            t_heating_pipe_c: 28.0,
            t_floor_c: 25.9,
            f_canopy_to_roof: 0.76,
            f_roof_to_canopy: 0.69,
            f_pipe_to_canopy: 0.64,
            f_canopy_to_pipe: 0.16,
            f_canopy_to_floor: 0.68,
            f_floor_to_canopy: 0.62,
        }),
        // 4. Summer afternoon (16 Jul 2015 13:00 UTC)
        evaluate_record(RecordSpec {
            period_label: "summer_afternoon",
            timestamp_iso_utc: "2015-07-16T13:00:00Z",
            timestamp_utc_s: 1_437_051_600,
            time_s: 19_486_800.0,
            crop_height_m: 3.9,
            shading_screen_deployment: 1.0,
            thermal_screen_deployment: 0.0,
            t_roof_outer_c: 33.5,
            t_sky_k: 290.844_250_200_322,
            t_canopy_leaf_c: 30.4,
            t_roof_inner_c: 34.1,
            t_heating_pipe_c: 30.4,
            t_floor_c: 29.8,
            f_canopy_to_roof: 0.84,
            f_roof_to_canopy: 0.76,
            f_pipe_to_canopy: 0.72,
            f_canopy_to_pipe: 0.18,
            f_canopy_to_floor: 0.75,
            f_floor_to_canopy: 0.68,
        }),
    ]
}

pub fn prepare_longwave_radiation_service(
    payload: PrepareLongwaveRadiationInput,
) -> Result<PrepareLongwaveRadiationOutput, AppError> {
    let preset = payload
        .preset_id
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .unwrap_or(DEFAULT_LONGWAVE_RADIATION_PRESET_ID);

    if preset != DEFAULT_LONGWAVE_RADIATION_PRESET_ID {
        return Err(AppError::BadRequest(format!(
            "unknown simulation longwave radiation preset '{preset}'; \
             supported preset is '{DEFAULT_LONGWAVE_RADIATION_PRESET_ID}'"
        )));
    }

    let filter = payload
        .period_filter
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .unwrap_or("all");

    let all_records = reference_records();

    let records: Vec<PreparedLongwaveRadiationRecord> = if filter == "all" {
        all_records
    } else {
        let filtered: Vec<PreparedLongwaveRadiationRecord> = all_records
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

    Ok(PrepareLongwaveRadiationOutput {
        preset_id: DEFAULT_LONGWAVE_RADIATION_PRESET_ID,
        execution_mode: "reference_dataset_direct",
        source_reference: "Sourisseau et al. (2026) PLOS One 21(2): e0340619, \
                           Section 3.5.2 (Eq. 14, Table 2), Sourisseau et al. (2025) Acta Hortic. 1426 & PyViewFactor",
        parameters: LongwaveNetworkParametersOutput {
            stefan_boltzmann_w_m2_k4: STEFAN_BOLTZMANN_W_M2_K4,
            roof_glass_emissivity: 0.89,
            canopy_leaf_emissivity: 0.96,
            floor_sheet_emissivity: 0.90,
            heating_pipe_emissivity: 0.91,
            thermal_screen_material_tau_lw: 0.18,
            shading_screen_material_tau_lw: 0.32,
            view_factor_methodology:
            "PyVista + PyViewFactor raytraced contour discretization with crop-height regression normalization (Section 3.5.2)",
        },
        records,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn equation_14_and_screen_protection_hold_across_all_records() {
        let output = prepare_longwave_radiation_service(
            PrepareLongwaveRadiationInput::default(),
        )
            .expect("default longwave radiation preset should succeed");

        assert_eq!(output.records.len(), 4);

        for record in &output.records {
            for pair in [
                &record.roof_outer_to_sky,
                &record.canopy_to_roof_inner,
                &record.heating_pipes_to_canopy,
                &record.canopy_to_floor,
            ] {
                let expected_denom = 1.0
                    - pair.reflectance_1
                    * pair.reflectance_2
                    * pair.view_factor_f21
                    * pair.view_factor_f12
                    * pair.intervening_tau_product.powi(2);
                assert!(
                    (pair.interreflection_denominator - expected_denom).abs()
                        < 1e-12
                );

                let expected_q = (pair.emissivity_1
                    * pair.emissivity_2
                    * pair.view_factor_f12
                    * pair.area_1_m2
                    * pair.intervening_tau_product
                    * STEFAN_BOLTZMANN_W_M2_K4
                    * (pair.temperature_1_k.powi(4)
                    - pair.temperature_2_k.powi(4)))
                    / expected_denom;
                assert!(
                    (pair.net_heat_flow_1_to_2_w - expected_q).abs() < 1e-9
                );
                assert!(
                    (pair.flux_per_floor_m2_w_m2 * FLOOR_AREA_M2 - expected_q)
                        .abs()
                        < 1e-9
                );
            }
        }

        let winter_night = &output.records[0];
        assert_eq!(winter_night.thermal_screen_deployment, 1.0);
        assert!(winter_night.screen_radiative_protection_saving_w > 15_000.0);
    }
}