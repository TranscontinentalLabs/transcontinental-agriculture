use serde::{Deserialize, Serialize};

use crate::error::AppError;

pub const DEFAULT_SURFACE_CONVECTION_PRESET_ID: &str = "ctifl_nantes_2014_2015";

pub const FLOOR_AREA_M2: f64 = 1_036.8;
pub const ROOF_SLOPE_RAD: f64 = 0.383_972_435_438_75;
pub const SOUTH_GABLE_WALL_AREA_M2: f64 = 177.0;
pub const NORTH_CORRIDOR_WALL_AREA_M2: f64 = 301.104;

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PrepareSurfaceConvectionInput {
    /// Optional preset identifier. Defaults to `"ctifl_nantes_2014_2015"`.
    pub preset_id: Option<String>,

    /// Optional filter matching Models 3–11 period labels:
    /// `"winter_night"`, `"winter_noon"`, `"equinox_noon"`, `"summer_afternoon"`, or `"all"`.
    pub period_filter: Option<String>,
}

#[derive(Debug, Clone, Copy, Serialize)]
pub struct ConvectionGeometryParametersOutput {
    pub floor_area_m2: f64,
    pub sloped_roof_area_m2: f64,
    pub roof_to_floor_area_ratio: f64,
    pub south_wall_area_m2: f64,
    pub north_corridor_wall_area_m2: f64,
    pub floor_warm_Formula: &'static str,
    pub floor_cool_formula: &'static str,
    pub vertical_wall_formula: &'static str,
    pub external_roof_low_wind_formula: &'static str,
    pub external_roof_high_wind_formula: &'static str,
}

#[derive(Debug, Clone, Copy, Serialize)]
pub struct SurfaceConvectionElementOutput {
    pub surface_name: &'static str,
    pub adjacent_air_zone: &'static str,
    pub correlation_branch: &'static str,

    /// Actual heat-transfer surface area A, m².
    pub surface_area_m2: f64,

    pub surface_temperature_c: f64,
    pub air_temperature_c: f64,

    /// T_s - T_a, °C (or K). Positive when surface is warmer than air.
    pub temperature_diff_s_minus_a_c: f64,

    /// Convective heat-transfer coefficient h_c on the surface's own area basis, W/(m²·K).
    pub hc_surface_basis_w_m2_k: f64,

    /// Heat flux per m² of the surface itself: h_c * (T_s - T_a), W/m²_surface.
    pub heat_flux_per_surface_m2_w_m2: f64,

    /// Whole-surface convective heat flow: A * h_c * (T_s - T_a), W.
    /// Positive means heat transfers from the surface into the air zone.
    pub total_convective_heat_flow_w: f64,

    /// Equivalent heat flux normalized per m² of greenhouse floor:
    /// total_convective_heat_flow_w / A_floor, W/m²_floor.
    pub equivalent_flux_per_floor_m2_w_m2: f64,
}

#[derive(Debug, Clone, Serialize)]
pub struct PreparedSurfaceConvectionRecord {
    pub period_label: &'static str,
    pub timestamp_iso_utc: &'static str,
    pub timestamp_utc_s: i64,
    pub time_s: f64,

    pub outdoor_wind_speed_m_s: f64,

    pub floor_to_main_air: SurfaceConvectionElementOutput,
    pub south_wall_inner_to_main_air: SurfaceConvectionElementOutput,
    pub north_wall_inner_to_main_air: SurfaceConvectionElementOutput,
    pub roof_outer_to_outdoor_air: SurfaceConvectionElementOutput,
}

#[derive(Debug, Clone, Serialize)]
pub struct PrepareSurfaceConvectionOutput {
    pub preset_id: &'static str,
    pub execution_mode: &'static str,
    pub source_reference: &'static str,
    pub parameters: ConvectionGeometryParametersOutput,
    pub records: Vec<PreparedSurfaceConvectionRecord>,
}

pub fn floor_hc_w_m2_k(t_surface_c: f64, t_air_c: f64) -> (f64, &'static str) {
    let dt_abs = (t_surface_c - t_air_c).abs();
    if t_surface_c > t_air_c {
        (1.86 * dt_abs.powf(0.33), "floor_warm (T_s > T_a): 1.86 * |dT|^0.33")
    } else {
        (1.3 * dt_abs.powf(0.25), "floor_cool (T_s <= T_a): 1.3 * |dT|^0.25")
    }
}

pub fn vertical_wall_hc_w_m2_k(t_surface_c: f64, t_air_c: f64) -> (f64, &'static str) {
    let dt_abs = (t_surface_c - t_air_c).abs();
    (
        1.31 * dt_abs.powf(0.33),
        "vertical_wall: 1.31 * |dT|^0.33",
    )
}

pub fn roof_outer_hc_w_m2_k(wind_speed_m_s: f64) -> (f64, &'static str) {
    if wind_speed_m_s < 4.0 {
        (
            2.8 + 1.2 * wind_speed_m_s,
            "roof_out_low_wind (u < 4 m/s): 2.8 + 1.2 * u",
        )
    } else {
        (
            2.5 * wind_speed_m_s.powf(0.8),
            "roof_out_high_wind (u >= 4 m/s): 2.5 * u^0.8",
        )
    }
}

fn build_convection_element(
    surface_name: &'static str,
    adjacent_air_zone: &'static str,
    correlation_branch: &'static str,
    surface_area_m2: f64,
    surface_temperature_c: f64,
    air_temperature_c: f64,
    hc_surface_basis_w_m2_k: f64,
) -> SurfaceConvectionElementOutput {
    let temperature_diff_s_minus_a_c = surface_temperature_c - air_temperature_c;
    let heat_flux_per_surface_m2_w_m2 =
        hc_surface_basis_w_m2_k * temperature_diff_s_minus_a_c;
    let total_convective_heat_flow_w =
        surface_area_m2 * heat_flux_per_surface_m2_w_m2;
    let equivalent_flux_per_floor_m2_w_m2 =
        total_convective_heat_flow_w / FLOOR_AREA_M2;

    SurfaceConvectionElementOutput {
        surface_name,
        adjacent_air_zone,
        correlation_branch,
        surface_area_m2,
        surface_temperature_c,
        air_temperature_c,
        temperature_diff_s_minus_a_c,
        hc_surface_basis_w_m2_k,
        heat_flux_per_surface_m2_w_m2,
        total_convective_heat_flow_w,
        equivalent_flux_per_floor_m2_w_m2,
    }
}

fn build_record(
    period_label: &'static str,
    timestamp_iso_utc: &'static str,
    timestamp_utc_s: i64,
    time_s: f64,
    outdoor_wind_speed_m_s: f64,
    t_main_air_c: f64,
    t_outdoor_air_c: f64,
    t_floor_c: f64,
    t_south_wall_inner_c: f64,
    t_north_wall_inner_c: f64,
    t_roof_outer_c: f64,
) -> PreparedSurfaceConvectionRecord {
    let sloped_roof_area_m2 = FLOOR_AREA_M2 / ROOF_SLOPE_RAD.cos();

    let (hc_floor, branch_floor) = floor_hc_w_m2_k(t_floor_c, t_main_air_c);
    let (hc_south, branch_south) =
        vertical_wall_hc_w_m2_k(t_south_wall_inner_c, t_main_air_c);
    let (hc_north, branch_north) =
        vertical_wall_hc_w_m2_k(t_north_wall_inner_c, t_main_air_c);
    let (hc_roof_out, branch_roof_out) =
        roof_outer_hc_w_m2_k(outdoor_wind_speed_m_s);

    PreparedSurfaceConvectionRecord {
        period_label,
        timestamp_iso_utc,
        timestamp_utc_s,
        time_s,
        outdoor_wind_speed_m_s,
        floor_to_main_air: build_convection_element(
            "Floor",
            "MainAir",
            branch_floor,
            FLOOR_AREA_M2,
            t_floor_c,
            t_main_air_c,
            hc_floor,
        ),
        south_wall_inner_to_main_air: build_convection_element(
            "SouthWallInner",
            "MainAir",
            branch_south,
            SOUTH_GABLE_WALL_AREA_M2,
            t_south_wall_inner_c,
            t_main_air_c,
            hc_south,
        ),
        north_wall_inner_to_main_air: build_convection_element(
            "NorthCorridorWallInner",
            "MainAir",
            branch_north,
            NORTH_CORRIDOR_WALL_AREA_M2,
            t_north_wall_inner_c,
            t_main_air_c,
            hc_north,
        ),
        roof_outer_to_outdoor_air: build_convection_element(
            "RoofOuter",
            "Outdoor",
            branch_roof_out,
            sloped_roof_area_m2,
            t_roof_outer_c,
            t_outdoor_air_c,
            hc_roof_out,
        ),
    }
}

fn reference_records() -> Vec<PreparedSurfaceConvectionRecord> {
    vec![
        // 1. Winter night (11 Jan 2015 02:00 UTC): floor cooler than heated MainAir (T_s <= T_a), wind 3.2 m/s (< 4)
        build_record(
            "winter_night",
            "2015-01-11T02:00:00Z",
            1_420_941_600,
            3_376_800.0,
            3.2,
            17.2,
            4.5,
            15.6,
            13.4,
            16.1,
            6.2,
        ),
        // 2. Winter noon (11 Jan 2015 12:00 UTC): solar-warmed floor (T_s > T_a), wind 4.1 m/s (>= 4)
        build_record(
            "winter_noon",
            "2015-01-11T12:00:00Z",
            1_420_977_600,
            3_412_800.0,
            4.1,
            20.4,
            9.2,
            21.8,
            16.5,
            18.8,
            13.1,
        ),
        // 3. Spring equinox noon (20 Mar 2015 12:00 UTC): solar-warmed floor (T_s > T_a), wind 3.5 m/s (< 4)
        build_record(
            "equinox_noon",
            "2015-03-20T12:00:00Z",
            1_426_852_800,
            9_288_000.0,
            3.5,
            23.6,
            14.8,
            25.9,
            21.2,
            21.9,
            19.4,
        ),
        // 4. Summer afternoon (16 Jul 2015 13:00 UTC): shaded floor cooler than MainAir (T_s <= T_a), wind 2.6 m/s (< 4)
        build_record(
            "summer_afternoon",
            "2015-07-16T13:00:00Z",
            1_437_051_600,
            19_486_800.0,
            2.6,
            31.2,
            28.4,
            29.8,
            30.1,
            28.9,
            33.5,
        ),
    ]
}

pub fn prepare_surface_convection_service(
    payload: PrepareSurfaceConvectionInput,
) -> Result<PrepareSurfaceConvectionOutput, AppError> {
    let preset = payload
        .preset_id
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .unwrap_or(DEFAULT_SURFACE_CONVECTION_PRESET_ID);

    if preset != DEFAULT_SURFACE_CONVECTION_PRESET_ID {
        return Err(AppError::BadRequest(format!(
            "unknown simulation surface convection preset '{preset}'; \
             supported preset is '{DEFAULT_SURFACE_CONVECTION_PRESET_ID}'"
        )));
    }

    let filter = payload
        .period_filter
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .unwrap_or("all");

    let all_records = reference_records();

    let records: Vec<PreparedSurfaceConvectionRecord> = if filter == "all" {
        all_records
    } else {
        let filtered: Vec<PreparedSurfaceConvectionRecord> = all_records
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

    let sloped_roof_area_m2 = FLOOR_AREA_M2 / ROOF_SLOPE_RAD.cos();

    Ok(PrepareSurfaceConvectionOutput {
        preset_id: DEFAULT_SURFACE_CONVECTION_PRESET_ID,
        execution_mode: "reference_dataset_direct",
        source_reference: "Sourisseau et al. (2026) PLOS One 21(2): e0340619, \
                           Section 3.5.1, Bot (1983) & TiGwerLib 0.7.0 HeatTransfer/Convection",
        parameters: ConvectionGeometryParametersOutput {
            floor_area_m2: FLOOR_AREA_M2,
            sloped_roof_area_m2,
            roof_to_floor_area_ratio: 1.0 / ROOF_SLOPE_RAD.cos(),
            south_wall_area_m2: SOUTH_GABLE_WALL_AREA_M2,
            north_corridor_wall_area_m2: NORTH_CORRIDOR_WALL_AREA_M2,
            floor_warm_Formula: "1.86 * |T_s - T_a|^0.33 (T_s > T_a)",
            floor_cool_formula: "1.30 * |T_s - T_a|^0.25 (T_s <= T_a)",
            vertical_wall_formula: "1.31 * |T_s - T_a|^0.33",
            external_roof_low_wind_formula: "2.8 + 1.2 * u (u < 4 m/s)",
            external_roof_high_wind_formula: "2.5 * u^0.8 (u >= 4 m/s)",
        },
        records,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn floor_wall_and_roof_correlations_and_heat_flows_hold_across_all_records() {
        let output = prepare_surface_convection_service(
            PrepareSurfaceConvectionInput::default(),
        )
            .expect("default surface convection preset should succeed");

        assert_eq!(output.records.len(), 4);

        for record in &output.records {
            // Floor branch check
            let fl = &record.floor_to_main_air;
            let expected_hc_fl = if fl.surface_temperature_c > fl.air_temperature_c {
                1.86 * (fl.surface_temperature_c - fl.air_temperature_c).abs().powf(0.33)
            } else {
                1.3 * (fl.surface_temperature_c - fl.air_temperature_c).abs().powf(0.25)
            };
            assert!((fl.hc_surface_basis_w_m2_k - expected_hc_fl).abs() < 1e-12);

            // Vertical wall check
            for wall in [
                &record.south_wall_inner_to_main_air,
                &record.north_wall_inner_to_main_air,
            ] {
                let expected_hc_w = 1.31
                    * (wall.surface_temperature_c - wall.air_temperature_c)
                    .abs()
                    .powf(0.33);
                assert!(
                    (wall.hc_surface_basis_w_m2_k - expected_hc_w).abs() < 1e-12
                );
            }

            // External roof wind branch check (winter_noon has u = 4.1 >= 4.0)
            let rf = &record.roof_outer_to_outdoor_air;
            let u = record.outdoor_wind_speed_m_s;
            let expected_hc_rf = if u < 4.0 {
                2.8 + 1.2 * u
            } else {
                2.5 * u.powf(0.8)
            };
            assert!((rf.hc_surface_basis_w_m2_k - expected_hc_rf).abs() < 1e-12);

            // General equation Q_dot = A * h_c * (T_s - T_a) check
            for elem in [
                &record.floor_to_main_air,
                &record.south_wall_inner_to_main_air,
                &record.north_wall_inner_to_main_air,
                &record.roof_outer_to_outdoor_air,
            ] {
                let expected_q = elem.surface_area_m2
                    * elem.hc_surface_basis_w_m2_k
                    * (elem.surface_temperature_c - elem.air_temperature_c);
                assert!(
                    (elem.total_convective_heat_flow_w - expected_q).abs()
                        < 1e-9
                );
                assert!(
                    (elem.equivalent_flux_per_floor_m2_w_m2 * FLOOR_AREA_M2
                        - expected_q)
                        .abs()
                        < 1e-9
                );
            }
        }
    }
}