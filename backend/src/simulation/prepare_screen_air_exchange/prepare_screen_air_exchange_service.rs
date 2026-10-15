use serde::{Deserialize, Serialize};

use crate::error::AppError;

pub const DEFAULT_SCREEN_AIR_EXCHANGE_PRESET_ID: &str = "ctifl_nantes_2014_2015";

pub const GRAVITY_M_S2: f64 = 9.81;
pub const COMPARTMENT_WIDTH_M: f64 = 24.0;
pub const FLOOR_AREA_M2: f64 = 1_036.8;
pub const MAIN_AIR_VOLUME_M3: f64 = 7_226.496;
pub const TOP_AIR_VOLUME_M3: f64 = 419.904;

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PrepareScreenAirExchangeInput {
    /// Optional preset identifier. Defaults to `"ctifl_nantes_2014_2015"`.
    pub preset_id: Option<String>,

    /// Optional filter matching Models 3–9 period labels (plus `"dual_screen_overlap"`):
    /// `"winter_night"`, `"winter_noon"`, `"equinox_noon"`, `"summer_afternoon"`,
    /// `"dual_screen_overlap"`, or `"all"`.
    pub period_filter: Option<String>,
}

#[derive(Debug, Clone, Copy, Serialize)]
pub struct ScreenSystemAirflowParametersOutput {
    pub lower_screen_model: &'static str,
    pub upper_screen_model: &'static str,
    pub lower_permeability_kl_m2: f64,
    pub lower_thickness_dl_m: f64,
    pub upper_permeability_ku_m2: f64,
    pub upper_thickness_du_m: f64,
    pub fabric_temperature_coefficient: f64,
    pub compartment_width_m: f64,
    pub edge_leakage_coefficient: f64,
}

#[derive(Debug, Clone, Copy, Serialize)]
pub struct ScreenAreaSuperpositionOutput {
    /// s_L: lower (thermal) screen deployment fraction in [0, 1].
    pub lower_deployment_sl: f64,

    /// s_U: upper (shading) screen deployment fraction in [0, 1].
    pub upper_deployment_su: f64,

    /// s_eq = max(s_L, s_U).
    pub equivalent_covered_fraction_seq: f64,

    /// State 1: 1 - max(s_L, s_U) (neither screen deployed).
    pub neither_deployed_fraction: f64,

    /// State 2: max(0, s_L - s_U) (only lower thermal screen deployed).
    pub lower_only_fraction: f64,

    /// State 3: max(0, s_U - s_L) (only upper shading screen deployed).
    pub upper_only_fraction: f64,

    /// State 4: min(s_L, s_U) (both screens deployed in series).
    pub both_deployed_fraction: f64,
}

#[derive(Debug, Clone, Serialize)]
pub struct PreparedScreenAirExchangeRecord {
    pub period_label: &'static str,
    pub timestamp_iso_utc: &'static str,
    pub timestamp_utc_s: i64,
    pub time_s: f64,

    pub area_states: ScreenAreaSuperpositionOutput,

    pub main_air_temperature_c: f64,
    pub top_air_temperature_c: f64,
    pub abs_temperature_diff_c: f64,

    pub main_air_density_kg_m3: f64,
    pub top_air_density_kg_m3: f64,
    pub abs_density_diff_kg_m3: f64,
    pub mean_density_kg_m3: f64,

    /// Area-weighted conductance sum in brackets of the f_fabric equation, m.
    pub equivalent_fabric_conductance_m: f64,

    /// Volumetric exchange rates per m² of greenhouse floor, m³/(s·m²).
    pub fabric_flow_per_floor_m2_s: f64,
    pub gap_flow_per_floor_m2_s: f64,
    pub edge_leakage_per_floor_m2_s: f64,
    pub total_screen_exchange_per_floor_m2_s: f64,

    /// Whole-compartment bilateral exchange rate between MainAir and TopAir, m³/s.
    pub whole_compartment_screen_exchange_m3_s: f64,

    /// Equivalent zone turnover rates (3600 * F_whole / V_zone), h⁻¹.
    pub main_air_turnover_per_h: f64,
    pub top_air_turnover_per_h: f64,

    /// Bilateral transport coefficients (k1 = k2 = 1.0) for Model 11.
    pub k1_main_to_top: f64,
    pub k2_top_to_main: f64,
}

#[derive(Debug, Clone, Serialize)]
pub struct PrepareScreenAirExchangeOutput {
    pub preset_id: &'static str,
    pub execution_mode: &'static str,
    pub source_reference: &'static str,
    pub parameters: ScreenSystemAirflowParametersOutput,
    pub records: Vec<PreparedScreenAirExchangeRecord>,
}

fn reference_records() -> Vec<PreparedScreenAirExchangeRecord> {
    vec![
        // 1. Winter night (11 Jan 2015 02:00 UTC) — State 2: lower thermal screen fully deployed (s_L=1, s_U=0)
        PreparedScreenAirExchangeRecord {
            period_label: "winter_night",
            timestamp_iso_utc: "2015-01-11T02:00:00Z",
            timestamp_utc_s: 1_420_941_600,
            time_s: 3_376_800.0,
            area_states: ScreenAreaSuperpositionOutput {
                lower_deployment_sl: 1.0,
                upper_deployment_su: 0.0,
                equivalent_covered_fraction_seq: 1.0,
                neither_deployed_fraction: 0.0,
                lower_only_fraction: 1.0,
                upper_only_fraction: 0.0,
                both_deployed_fraction: 0.0,
            },
            main_air_temperature_c: 17.2,
            top_air_temperature_c: 11.4,
            abs_temperature_diff_c: 5.8,
            main_air_density_kg_m3: 1.214_470_796_700,
            top_air_density_kg_m3: 1.240_907_556_900,
            abs_density_diff_kg_m3: 0.026_436_760_200,
            mean_density_kg_m3: 1.227_689_176_800,
            equivalent_fabric_conductance_m: 0.000_000_514_285_714,
            fabric_flow_per_floor_m2_s: 0.006_860_571_429,
            gap_flow_per_floor_m2_s: 0.0,
            edge_leakage_per_floor_m2_s: 0.000_551_538_269,
            total_screen_exchange_per_floor_m2_s: 0.007_412_109_698,
            whole_compartment_screen_exchange_m3_s: 7.684_875_334_778,
            main_air_turnover_per_h: 3.828_349_341_811,
            top_air_turnover_per_h: 65.885_419_536_849,
            k1_main_to_top: 1.0,
            k2_top_to_main: 1.0,
        },
        // 2. Winter noon (11 Jan 2015 12:00 UTC) — State 1: neither screen deployed (s_L=0, s_U=0)
        PreparedScreenAirExchangeRecord {
            period_label: "winter_noon",
            timestamp_iso_utc: "2015-01-11T12:00:00Z",
            timestamp_utc_s: 1_420_977_600,
            time_s: 3_412_800.0,
            area_states: ScreenAreaSuperpositionOutput {
                lower_deployment_sl: 0.0,
                upper_deployment_su: 0.0,
                equivalent_covered_fraction_seq: 0.0,
                neither_deployed_fraction: 1.0,
                lower_only_fraction: 0.0,
                upper_only_fraction: 0.0,
                both_deployed_fraction: 0.0,
            },
            main_air_temperature_c: 20.4,
            top_air_temperature_c: 18.9,
            abs_temperature_diff_c: 1.5,
            main_air_density_kg_m3: 1.199_440_958_200,
            top_air_density_kg_m3: 1.206_101_282_000,
            abs_density_diff_kg_m3: 0.006_660_323_800,
            mean_density_kg_m3: 1.202_771_120_100,
            equivalent_fabric_conductance_m: 0.0,
            fabric_flow_per_floor_m2_s: 0.0,
            gap_flow_per_floor_m2_s: 0.807_386_163_791,
            edge_leakage_per_floor_m2_s: 0.0,
            total_screen_exchange_per_floor_m2_s: 0.807_386_163_791,
            whole_compartment_screen_exchange_m3_s: 837.097_974_618_460,
            main_air_turnover_per_h: 417.014_374_411_396,
            top_air_turnover_per_h: 7_176.765_900_364_029,
            k1_main_to_top: 1.0,
            k2_top_to_main: 1.0,
        },
        // 3. Spring equinox noon (20 Mar 2015 12:00 UTC) — State 1: neither screen deployed (s_L=0, s_U=0)
        PreparedScreenAirExchangeRecord {
            period_label: "equinox_noon",
            timestamp_iso_utc: "2015-03-20T12:00:00Z",
            timestamp_utc_s: 1_426_852_800,
            time_s: 9_288_000.0,
            area_states: ScreenAreaSuperpositionOutput {
                lower_deployment_sl: 0.0,
                upper_deployment_su: 0.0,
                equivalent_covered_fraction_seq: 0.0,
                neither_deployed_fraction: 1.0,
                lower_only_fraction: 0.0,
                upper_only_fraction: 0.0,
                both_deployed_fraction: 0.0,
            },
            main_air_temperature_c: 23.6,
            top_air_temperature_c: 22.8,
            abs_temperature_diff_c: 0.8,
            main_air_density_kg_m3: 1.182_241_541_700,
            top_air_density_kg_m3: 1.186_101_008_200,
            abs_density_diff_kg_m3: 0.003_859_466_500,
            mean_density_kg_m3: 1.184_171_274_950,
            equivalent_fabric_conductance_m: 0.0,
            fabric_flow_per_floor_m2_s: 0.0,
            gap_flow_per_floor_m2_s: 0.619_414_693_848,
            edge_leakage_per_floor_m2_s: 0.0,
            total_screen_exchange_per_floor_m2_s: 0.619_414_693_848,
            whole_compartment_screen_exchange_m3_s: 642.209_154_581_549,
            main_air_turnover_per_h: 319.927_245_029_068,
            top_air_turnover_per_h: 5_505.908_389_759_509,
            k1_main_to_top: 1.0,
            k2_top_to_main: 1.0,
        },
        // 4. Summer afternoon (16 Jul 2015 13:00 UTC, Fig. 9) — State 3: upper shading screen deployed (s_L=0, s_U=1)
        PreparedScreenAirExchangeRecord {
            period_label: "summer_afternoon",
            timestamp_iso_utc: "2015-07-16T13:00:00Z",
            timestamp_utc_s: 1_437_051_600,
            time_s: 19_486_800.0,
            area_states: ScreenAreaSuperpositionOutput {
                lower_deployment_sl: 0.0,
                upper_deployment_su: 1.0,
                equivalent_covered_fraction_seq: 1.0,
                neither_deployed_fraction: 0.0,
                lower_only_fraction: 0.0,
                upper_only_fraction: 1.0,
                both_deployed_fraction: 0.0,
            },
            main_air_temperature_c: 31.2,
            top_air_temperature_c: 34.6,
            abs_temperature_diff_c: 3.4,
            main_air_density_kg_m3: 1.147_601_263_300,
            top_air_density_kg_m3: 1.134_247_491_100,
            abs_density_diff_kg_m3: 0.013_353_772_200,
            mean_density_kg_m3: 1.140_924_377_200,
            equivalent_fabric_conductance_m: 0.000_001_625_000_000,
            fabric_flow_per_floor_m2_s: 0.012_707_500_000,
            gap_flow_per_floor_m2_s: 0.0,
            edge_leakage_per_floor_m2_s: 0.000_406_620_530,
            total_screen_exchange_per_floor_m2_s: 0.013_114_120_530,
            whole_compartment_screen_exchange_m3_s: 13.596_720_165_133,
            main_air_turnover_per_h: 6.773_433_846_013,
            top_air_turnover_per_h: 116.569_960_263_484,
            k1_main_to_top: 1.0,
            k2_top_to_main: 1.0,
        },
        // 5. Dual-screen partial overlap state (s_L=0.8, s_U=0.5) — exercises State 4 series overlap
        PreparedScreenAirExchangeRecord {
            period_label: "dual_screen_overlap",
            timestamp_iso_utc: "2015-02-05T07:30:00Z",
            timestamp_utc_s: 1_423_121_400,
            time_s: 5_556_600.0,
            area_states: ScreenAreaSuperpositionOutput {
                lower_deployment_sl: 0.8,
                upper_deployment_su: 0.5,
                equivalent_covered_fraction_seq: 0.8,
                neither_deployed_fraction: 0.2,
                lower_only_fraction: 0.3,
                upper_only_fraction: 0.0,
                both_deployed_fraction: 0.5,
            },
            main_air_temperature_c: 18.5,
            top_air_temperature_c: 13.5,
            abs_temperature_diff_c: 5.0,
            main_air_density_kg_m3: 1.208_000_000_000,
            top_air_density_kg_m3: 1.231_000_000_000,
            abs_density_diff_kg_m3: 0.023_000_000_000,
            mean_density_kg_m3: 1.219_500_000_000,
            equivalent_fabric_conductance_m: 0.000_000_349_611_257,
            fabric_flow_per_floor_m2_s: 0.004_020_529_454,
            gap_flow_per_floor_m2_s: 0.133_273_295_216,
            edge_leakage_per_floor_m2_s: 0.000_412_932_202,
            total_screen_exchange_per_floor_m2_s: 0.137_706_756_872,
            whole_compartment_screen_exchange_m3_s: 142.774_365_524_966,
            main_air_turnover_per_h: 71.125_441_139_091,
            top_air_turnover_per_h: 1_224.060_061_085_097,
            k1_main_to_top: 1.0,
            k2_top_to_main: 1.0,
        },
    ]
}

pub fn prepare_screen_air_exchange_service(
    payload: PrepareScreenAirExchangeInput,
) -> Result<PrepareScreenAirExchangeOutput, AppError> {
    let preset = payload
        .preset_id
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .unwrap_or(DEFAULT_SCREEN_AIR_EXCHANGE_PRESET_ID);

    if preset != DEFAULT_SCREEN_AIR_EXCHANGE_PRESET_ID {
        return Err(AppError::BadRequest(format!(
            "unknown simulation screen air exchange preset '{preset}'; \
             supported preset is '{DEFAULT_SCREEN_AIR_EXCHANGE_PRESET_ID}'"
        )));
    }

    let filter = payload
        .period_filter
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .unwrap_or("all");

    let all_records = reference_records();

    let records: Vec<PreparedScreenAirExchangeRecord> = if filter == "all" {
        all_records
    } else {
        let filtered: Vec<PreparedScreenAirExchangeRecord> = all_records
            .into_iter()
            .filter(|record| record.period_label == filter)
            .collect();

        if filtered.is_empty() {
            return Err(AppError::BadRequest(format!(
                "unknown period_filter '{filter}'; supported values are \
                 'all', 'winter_night', 'winter_noon', 'equinox_noon', \
                 'summer_afternoon', or 'dual_screen_overlap'"
            )));
        }

        filtered
    };

    Ok(PrepareScreenAirExchangeOutput {
        preset_id: DEFAULT_SCREEN_AIR_EXCHANGE_PRESET_ID,
        execution_mode: "reference_dataset_direct",
        source_reference: "Sourisseau et al. (2026) PLOS One 21(2): e0340619, \
                           Section 3.4.3, Sourisseau et al. (2023) Acta Hortic. 1377 & TiGwerLib 0.7.0",
        parameters: ScreenSystemAirflowParametersOutput {
            lower_screen_model: "Svensson Luxous 1547 D FR (thermal screen)",
            upper_screen_model: "Svensson Harmony 3315 O FR (shading screen)",
            lower_permeability_kl_m2: 1.8e-10,
            lower_thickness_dl_m: 0.000_35,
            upper_permeability_ku_m2: 6.5e-10,
            upper_thickness_du_m: 0.000_40,
            fabric_temperature_coefficient: 2_300.0,
            compartment_width_m: COMPARTMENT_WIDTH_M,
            edge_leakage_coefficient: 0.0012,
        },
        records,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn four_area_states_and_fabric_gap_edge_equations_hold_across_all_records() {
        let output = prepare_screen_air_exchange_service(
            PrepareScreenAirExchangeInput::default(),
        )
            .expect("default screen air exchange preset should succeed");

        let p = &output.parameters;
        assert_eq!(output.records.len(), 5);

        for record in &output.records {
            let s = &record.area_states;

            // Area superposition fractions must sum to 1.
            let area_sum = s.neither_deployed_fraction
                + s.lower_only_fraction
                + s.upper_only_fraction
                + s.both_deployed_fraction;
            assert!((area_sum - 1.0).abs() < 1e-12);

            // Series-parallel fabric conductance check.
            let cond_l = p.lower_permeability_kl_m2 / p.lower_thickness_dl_m;
            let cond_u = p.upper_permeability_ku_m2 / p.upper_thickness_du_m;
            let cond_series = 1.0
                / (p.lower_thickness_dl_m / p.lower_permeability_kl_m2
                + p.upper_thickness_du_m / p.upper_permeability_ku_m2);

            let expected_cond = s.lower_only_fraction * cond_l
                + s.upper_only_fraction * cond_u
                + s.both_deployed_fraction * cond_series;
            assert!(
                (record.equivalent_fabric_conductance_m - expected_cond).abs()
                    < 1e-15
            );

            let expected_fabric = p.fabric_temperature_coefficient
                * record.abs_temperature_diff_c
                * expected_cond;
            assert!(
                (record.fabric_flow_per_floor_m2_s - expected_fabric).abs()
                    < 1e-11
            );

            let open = s.neither_deployed_fraction;
            let expected_gap = open
                * ((0.5
                * p.compartment_width_m
                * open
                * GRAVITY_M_S2
                * record.abs_density_diff_kg_m3)
                / record.mean_density_kg_m3)
                .sqrt();
            assert!(
                (record.gap_flow_per_floor_m2_s - expected_gap).abs() < 1e-11
            );

            let expected_tot = record.fabric_flow_per_floor_m2_s
                + record.gap_flow_per_floor_m2_s
                + record.edge_leakage_per_floor_m2_s;
            assert!(
                (record.total_screen_exchange_per_floor_m2_s - expected_tot)
                    .abs()
                    < 1e-11
            );
        }
    }
}