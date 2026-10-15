use serde::{Deserialize, Serialize};

use crate::error::AppError;

pub const DEFAULT_AIR_LEAKAGE_PRESET_ID: &str = "ctifl_nantes_2014_2015";

/// ASHRAE power-law flow exponent n (-) used in Eq. (11) (Section 3.4.2).
pub const REFERENCE_FLOW_EXPONENT_N: f64 = 0.65;

pub const FLOOR_AREA_M2: f64 = 1_036.8;
pub const TOTAL_VOLUME_M3: f64 = 7_646.4;

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PrepareAirLeakageInput {
    /// Optional preset identifier. Defaults to `"ctifl_nantes_2014_2015"`.
    pub preset_id: Option<String>,

    /// Optional filter matching Models 3–8 period labels:
    /// `"winter_night"`, `"winter_noon"`, `"equinox_noon"`, `"summer_afternoon"`, or `"all"`.
    pub period_filter: Option<String>,
}

#[derive(Debug, Clone, Copy, Serialize)]
pub struct LeakagePathParametersOutput {
    pub flow_exponent_n: f64,
    pub eta_top_air_to_outdoor_m3_s_pa_n: f64,
    pub eta_main_air_to_north_corridor_m3_s_pa_n: f64,
    pub eta_main_air_to_east_corridor_m3_s_pa_n: f64,
    pub eta_south_wall_m3_s_pa_n: f64,
    pub eta_west_compartment_m3_s_pa_n: f64,
    pub closed_vent_virtual_crack_height_mm: f64,
    pub boundary_assumptions_note: &'static str,
    pub published_water_loss_share_via_pressure_leakage_pct: f64,
}

#[derive(Debug, Clone, Copy, Serialize)]
pub struct CrackPathFlowOutput {
    pub upstream_zone: &'static str,
    pub downstream_zone: &'static str,
    pub pressure_inside_pa: f64,
    pub pressure_outside_pa: f64,

    /// delta_p = p_inside - p_outside, Pa.
    /// Positive means exfiltration (inside -> outside);
    /// negative means infiltration (outside -> inside).
    pub delta_p_pa: f64,

    pub flow_coefficient_eta: f64,

    /// Signed volumetric flow F_crack = sign(delta_p) * eta * |delta_p|^n, m³/s.
    pub signed_flow_m3_s: f64,

    /// Unilateral transport direction coefficients (k1, k2) for Model 11:
    /// (1.0, 0.0) when delta_p > 0; (0.0, 1.0) when delta_p <= 0.
    pub k1_inside_to_outside: f64,
    pub k2_outside_to_inside: f64,
}

#[derive(Debug, Clone, Serialize)]
pub struct PreparedAirLeakageRecord {
    pub period_label: &'static str,
    pub timestamp_iso_utc: &'static str,
    pub timestamp_utc_s: i64,
    pub time_s: f64,

    pub vents_commanded_closed: bool,

    pub top_air_to_outdoor: CrackPathFlowOutput,
    pub main_air_to_north_corridor: CrackPathFlowOutput,
    pub main_air_to_east_corridor: CrackPathFlowOutput,

    /// Sum of |F_crack| across all pressure-driven paths, m³/s.
    pub total_pressure_crack_magnitude_m3_s: f64,

    /// Residual closed-vent crack exchange from Model 8 (h_vent,leak = 6 mm)
    /// when vents are commanded closed; 0.0 when vents are actively opened.
    pub closed_vent_residual_exchange_m3_s: f64,

    /// Total unintended infiltration/exfiltration magnitude when closed, m³/s.
    pub total_unintended_exchange_m3_s: f64,
    pub total_unintended_exchange_per_floor_m2_s: f64,

    /// Air changes per hour on whole-greenhouse volume (7646.4 m³).
    pub ach_pressure_cracks_only_per_h: f64,
    pub ach_total_unintended_per_h: f64,
}

#[derive(Debug, Clone, Serialize)]
pub struct PrepareAirLeakageOutput {
    pub preset_id: &'static str,
    pub execution_mode: &'static str,
    pub source_reference: &'static str,
    pub parameters: LeakagePathParametersOutput,
    pub records: Vec<PreparedAirLeakageRecord>,
}

fn reference_records() -> Vec<PreparedAirLeakageRecord> {
    vec![
        // 1. Winter night (11 Jan 2015 02:00 UTC) — vents closed, both pressure cracks and 6 mm vent crack active
        PreparedAirLeakageRecord {
            period_label: "winter_night",
            timestamp_iso_utc: "2015-01-11T02:00:00Z",
            timestamp_utc_s: 1_420_941_600,
            time_s: 3_376_800.0,
            vents_commanded_closed: true,
            top_air_to_outdoor: CrackPathFlowOutput {
                upstream_zone: "TopAir",
                downstream_zone: "Outdoor",
                pressure_inside_pa: 101_795.0,
                pressure_outside_pa: 101_800.0,
                delta_p_pa: -5.0,
                flow_coefficient_eta: 0.045,
                signed_flow_m3_s: -0.128_098_196_871,
                k1_inside_to_outside: 0.0,
                k2_outside_to_inside: 1.0,
            },
            main_air_to_north_corridor: CrackPathFlowOutput {
                upstream_zone: "MainAir",
                downstream_zone: "NorthCorridor",
                pressure_inside_pa: 101_798.0,
                pressure_outside_pa: 101_799.2,
                delta_p_pa: -1.2,
                flow_coefficient_eta: 0.032,
                signed_flow_m3_s: -0.036_026_144_634,
                k1_inside_to_outside: 0.0,
                k2_outside_to_inside: 1.0,
            },
            main_air_to_east_corridor: CrackPathFlowOutput {
                upstream_zone: "MainAir",
                downstream_zone: "EastCorridor",
                pressure_inside_pa: 101_798.0,
                pressure_outside_pa: 101_798.8,
                delta_p_pa: -0.8,
                flow_coefficient_eta: 0.028,
                signed_flow_m3_s: -0.024_219_575_274,
                k1_inside_to_outside: 0.0,
                k2_outside_to_inside: 1.0,
            },
            total_pressure_crack_magnitude_m3_s: 0.188_343_916_779,
            closed_vent_residual_exchange_m3_s: 0.432_922_213_651,
            total_unintended_exchange_m3_s: 0.621_266_130_430,
            total_unintended_exchange_per_floor_m2_s: 0.621_266_130_430 / FLOOR_AREA_M2,
            ach_pressure_cracks_only_per_h: 0.088_674_160_442,
            ach_total_unintended_per_h: 0.292_498_178_169,
        },
        // 2. Winter noon (11 Jan 2015 12:00 UTC) — leeward vents open (4°), pressure cracks active
        PreparedAirLeakageRecord {
            period_label: "winter_noon",
            timestamp_iso_utc: "2015-01-11T12:00:00Z",
            timestamp_utc_s: 1_420_977_600,
            time_s: 3_412_800.0,
            vents_commanded_closed: false,
            top_air_to_outdoor: CrackPathFlowOutput {
                upstream_zone: "TopAir",
                downstream_zone: "Outdoor",
                pressure_inside_pa: 101_746.0,
                pressure_outside_pa: 101_750.0,
                delta_p_pa: -4.0,
                flow_coefficient_eta: 0.045,
                signed_flow_m3_s: -0.110_802_997_201,
                k1_inside_to_outside: 0.0,
                k2_outside_to_inside: 1.0,
            },
            main_air_to_north_corridor: CrackPathFlowOutput {
                upstream_zone: "MainAir",
                downstream_zone: "NorthCorridor",
                pressure_inside_pa: 101_748.0,
                pressure_outside_pa: 101_749.0,
                delta_p_pa: -1.0,
                flow_coefficient_eta: 0.032,
                signed_flow_m3_s: -0.032_000_000_000,
                k1_inside_to_outside: 0.0,
                k2_outside_to_inside: 1.0,
            },
            main_air_to_east_corridor: CrackPathFlowOutput {
                upstream_zone: "MainAir",
                downstream_zone: "EastCorridor",
                pressure_inside_pa: 101_748.0,
                pressure_outside_pa: 101_748.6,
                delta_p_pa: -0.6,
                flow_coefficient_eta: 0.028,
                signed_flow_m3_s: -0.020_088_908_659,
                k1_inside_to_outside: 0.0,
                k2_outside_to_inside: 1.0,
            },
            total_pressure_crack_magnitude_m3_s: 0.162_891_905_860,
            closed_vent_residual_exchange_m3_s: 0.0,
            total_unintended_exchange_m3_s: 0.162_891_905_860,
            total_unintended_exchange_per_floor_m2_s: 0.162_891_905_860 / FLOOR_AREA_M2,
            ach_pressure_cracks_only_per_h: 0.076_691_104_454,
            ach_total_unintended_per_h: 0.076_691_104_454,
        },
        // 3. Spring equinox noon (20 Mar 2015 12:00 UTC)
        PreparedAirLeakageRecord {
            period_label: "equinox_noon",
            timestamp_iso_utc: "2015-03-20T12:00:00Z",
            timestamp_utc_s: 1_426_852_800,
            time_s: 9_288_000.0,
            vents_commanded_closed: false,
            top_air_to_outdoor: CrackPathFlowOutput {
                upstream_zone: "TopAir",
                downstream_zone: "Outdoor",
                pressure_inside_pa: 101_496.0,
                pressure_outside_pa: 101_500.0,
                delta_p_pa: -4.0,
                flow_coefficient_eta: 0.045,
                signed_flow_m3_s: -0.110_802_997_201,
                k1_inside_to_outside: 0.0,
                k2_outside_to_inside: 1.0,
            },
            main_air_to_north_corridor: CrackPathFlowOutput {
                upstream_zone: "MainAir",
                downstream_zone: "NorthCorridor",
                pressure_inside_pa: 101_498.0,
                pressure_outside_pa: 101_498.8,
                delta_p_pa: -0.8,
                flow_coefficient_eta: 0.032,
                signed_flow_m3_s: -0.027_679_514_599,
                k1_inside_to_outside: 0.0,
                k2_outside_to_inside: 1.0,
            },
            main_air_to_east_corridor: CrackPathFlowOutput {
                upstream_zone: "MainAir",
                downstream_zone: "EastCorridor",
                pressure_inside_pa: 101_498.0,
                pressure_outside_pa: 101_498.5,
                delta_p_pa: -0.5,
                flow_coefficient_eta: 0.028,
                signed_flow_m3_s: -0.017_843_848_782,
                k1_inside_to_outside: 0.0,
                k2_outside_to_inside: 1.0,
            },
            total_pressure_crack_magnitude_m3_s: 0.156_326_360_583,
            closed_vent_residual_exchange_m3_s: 0.0,
            total_unintended_exchange_m3_s: 0.156_326_360_583,
            total_unintended_exchange_per_floor_m2_s: 0.156_326_360_583 / FLOOR_AREA_M2,
            ach_pressure_cracks_only_per_h: 0.073_599_981_442,
            ach_total_unintended_per_h: 0.073_599_981_442,
        },
        // 4. Summer afternoon (16 Jul 2015 13:00 UTC)
        PreparedAirLeakageRecord {
            period_label: "summer_afternoon",
            timestamp_iso_utc: "2015-07-16T13:00:00Z",
            timestamp_utc_s: 1_437_051_600,
            time_s: 19_486_800.0,
            vents_commanded_closed: false,
            top_air_to_outdoor: CrackPathFlowOutput {
                upstream_zone: "TopAir",
                downstream_zone: "Outdoor",
                pressure_inside_pa: 101_321.0,
                pressure_outside_pa: 101_325.0,
                delta_p_pa: -4.0,
                flow_coefficient_eta: 0.045,
                signed_flow_m3_s: -0.110_802_997_201,
                k1_inside_to_outside: 0.0,
                k2_outside_to_inside: 1.0,
            },
            main_air_to_north_corridor: CrackPathFlowOutput {
                upstream_zone: "MainAir",
                downstream_zone: "NorthCorridor",
                pressure_inside_pa: 101_323.0,
                pressure_outside_pa: 101_323.6,
                delta_p_pa: -0.6,
                flow_coefficient_eta: 0.032,
                signed_flow_m3_s: -0.022_958_752_753,
                k1_inside_to_outside: 0.0,
                k2_outside_to_inside: 1.0,
            },
            main_air_to_east_corridor: CrackPathFlowOutput {
                upstream_zone: "MainAir",
                downstream_zone: "EastCorridor",
                pressure_inside_pa: 101_323.0,
                pressure_outside_pa: 101_323.4,
                delta_p_pa: -0.4,
                flow_coefficient_eta: 0.028,
                signed_flow_m3_s: -0.015_434_658_527,
                k1_inside_to_outside: 0.0,
                k2_outside_to_inside: 1.0,
            },
            total_pressure_crack_magnitude_m3_s: 0.149_196_408_481,
            closed_vent_residual_exchange_m3_s: 0.0,
            total_unintended_exchange_m3_s: 0.149_196_408_481,
            total_unintended_exchange_per_floor_m2_s: 0.149_196_408_481 / FLOOR_AREA_M2,
            ach_pressure_cracks_only_per_h: 0.070_243_130_170,
            ach_total_unintended_per_h: 0.070_243_130_170,
        },
    ]
}

pub fn prepare_air_leakage_service(
    payload: PrepareAirLeakageInput,
) -> Result<PrepareAirLeakageOutput, AppError> {
    let preset = payload
        .preset_id
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .unwrap_or(DEFAULT_AIR_LEAKAGE_PRESET_ID);

    if preset != DEFAULT_AIR_LEAKAGE_PRESET_ID {
        return Err(AppError::BadRequest(format!(
            "unknown simulation air leakage preset '{preset}'; \
             supported preset is '{DEFAULT_AIR_LEAKAGE_PRESET_ID}'"
        )));
    }

    let filter = payload
        .period_filter
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .unwrap_or("all");

    let all_records = reference_records();

    let records: Vec<PreparedAirLeakageRecord> = if filter == "all" {
        all_records
    } else {
        let filtered: Vec<PreparedAirLeakageRecord> = all_records
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

    Ok(PrepareAirLeakageOutput {
        preset_id: DEFAULT_AIR_LEAKAGE_PRESET_ID,
        execution_mode: "reference_dataset_direct",
        source_reference: "Sourisseau et al. (2026) PLOS One 21(2): e0340619, \
                           Section 3.4.2 (Eq. 11), Section 4.4.1 (Fig. 13) & ASHRAE (2013)",
        parameters: LeakagePathParametersOutput {
            flow_exponent_n: REFERENCE_FLOW_EXPONENT_N,
            eta_top_air_to_outdoor_m3_s_pa_n: 0.045,
            eta_main_air_to_north_corridor_m3_s_pa_n: 0.032,
            eta_main_air_to_east_corridor_m3_s_pa_n: 0.028,
            eta_south_wall_m3_s_pa_n: 0.0,
            eta_west_compartment_m3_s_pa_n: 0.0,
            closed_vent_virtual_crack_height_mm: 6.0,
            boundary_assumptions_note:
            "South wall has no window/door and is assumed airtight; West wall adjoins an identical compartment with neutral net exchange (Section 3.4.2)",
            published_water_loss_share_via_pressure_leakage_pct: 6.3,
        },
        records,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn power_law_equation_11_and_totals_hold_across_all_records() {
        let output =
            prepare_air_leakage_service(PrepareAirLeakageInput::default())
                .expect("default air leakage preset should succeed");

        let n = output.parameters.flow_exponent_n;
        assert!((n - 0.65).abs() < 1e-12);
        assert_eq!(output.records.len(), 4);

        for record in &output.records {
            for path in [
                &record.top_air_to_outdoor,
                &record.main_air_to_north_corridor,
                &record.main_air_to_east_corridor,
            ] {
                let expected_dp =
                    path.pressure_inside_pa - path.pressure_outside_pa;
                assert!((path.delta_p_pa - expected_dp).abs() < 1e-9);

                let expected_flow = expected_dp.signum()
                    * path.flow_coefficient_eta
                    * expected_dp.abs().powf(n);
                assert!((path.signed_flow_m3_s - expected_flow).abs() < 1e-11);

                // Unilateral k1/k2 check (Section 3.4.4):
                assert_eq!(
                    path.k1_inside_to_outside + path.k2_outside_to_inside,
                    1.0
                );
            }

            let expected_mag = record.top_air_to_outdoor.signed_flow_m3_s.abs()
                + record.main_air_to_north_corridor.signed_flow_m3_s.abs()
                + record.main_air_to_east_corridor.signed_flow_m3_s.abs();
            assert!(
                (record.total_pressure_crack_magnitude_m3_s - expected_mag)
                    .abs()
                    < 1e-11
            );

            let expected_tot =
                expected_mag + record.closed_vent_residual_exchange_m3_s;
            assert!(
                (record.total_unintended_exchange_m3_s - expected_tot).abs()
                    < 1e-11
            );

            let expected_ach = 3_600.0 * expected_tot / TOTAL_VOLUME_M3;
            assert!(
                (record.ach_total_unintended_per_h - expected_ach).abs() < 1e-11
            );
        }
    }
}