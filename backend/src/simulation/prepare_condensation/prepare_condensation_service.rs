use serde::{Deserialize, Serialize};

use crate::error::AppError;

pub const DEFAULT_CONDENSATION_PRESET_ID: &str = "ctifl_nantes_2014_2015";

pub const UNIVERSAL_GAS_CONSTANT_J_MOL_K: f64 = 8.314_462_618;
pub const WATER_VAPOUR_MOLAR_MASS_KG_MOL: f64 = 0.018_015_28;
pub const CP_AIR_J_KG_K: f64 = 1_006.0;
pub const LEWIS_NUMBER: f64 = 0.89;
pub const LEWIS_EXPONENT_N: f64 = 1.0 / 3.0;
pub const LATENT_HEAT_CONDENSATION_J_KG: f64 = 2_450_000.0;

pub const FLOOR_AREA_M2: f64 = 1_036.8;
pub const ROOF_SLOPE_RAD: f64 = 0.383_972_435_438_75;
pub const SOUTH_WALL_AREA_M2: f64 = 177.0;
pub const NORTH_WALL_AREA_M2: f64 = 301.104;

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PrepareCondensationInput {
    /// Optional preset identifier. Defaults to `"ctifl_nantes_2014_2015"`.
    pub preset_id: Option<String>,

    /// Optional filter matching Models 3–13 period labels:
    /// `"winter_night"`, `"winter_noon"`, `"equinox_noon"`, `"summer_afternoon"`, or `"all"`.
    pub period_filter: Option<String>,
}

#[derive(Debug, Clone, Copy, Serialize)]
pub struct PublishedSeasonCondensationSummaryOutput {
    pub condensed_share_of_transpired_water_pct: f64,
    pub roof_condensation_333d_kg_per_m2_roof: f64,
    pub roof_condensation_annualized_kg_per_m2_roof_yr: f64,
    pub cover_condensation_333d_kg_per_m2_cover: f64,
    pub cover_condensation_annualized_kg_per_m2_cover_yr: f64,
    pub south_wall_share_of_roof_condensation_pct: f64,
    pub south_wall_glass_share_pct: f64,
    pub south_wall_curtain_share_pct: f64,
    pub east_wall_total_condensation_kg: f64,
    pub north_wall_total_condensation_kg: f64,
    pub horizontal_screens_total_condensation_kg: f64,
    pub agronomic_limit_note: &'static str,
}

#[derive(Debug, Clone, Copy, Serialize)]
pub struct SurfaceCondensationElementOutput {
    pub surface_name: &'static str,
    pub adjacent_air_zone: &'static str,
    pub surface_area_m2: f64,

    pub air_temperature_c: f64,
    pub surface_temperature_c: f64,
    pub mean_temperature_k: f64,
    pub air_density_kg_m3: f64,

    pub air_vapour_pressure_pa: f64,
    pub surface_saturation_vapour_pressure_pa: f64,

    /// p_v,air - p_sat(T_s), Pa. Positive drives condensation; negative drives potential evaporation.
    pub vapour_pressure_excess_pa: f64,

    pub hc_w_m2_k: f64,

    /// Lewis-analogy factor a = Le^(n-1) * M_v / (R * rho * c_p * T_m), s·K·m / J.
    pub lewis_factor_a: f64,

    /// Mass-transfer coefficient k_m = a * h_c, kg/(s·m²·Pa).
    pub mass_transfer_coeff_km: f64,

    /// Positive condensation rate m_dot_cond = A * k_m * max(0, p_v,air - p_sat(T_s)), kg/s.
    pub condensation_rate_kg_s: f64,

    /// Potential evaporation rate A * k_m * max(0, p_sat(T_s) - p_v,air), kg/s.
    pub potential_evaporation_rate_kg_s: f64,

    /// Available liquid water mass rate on the surface for re-evaporation, kg/s.
    pub available_water_rate_kg_s: f64,

    /// Actual evaporation rate max(0, min(m_dot_potential, m_dot_available)), kg/s.
    pub evaporation_rate_kg_s: f64,

    /// Latent heat released to the surface by condensation (+lambda * m_dot_cond), W.
    pub condensation_latent_heat_release_w: f64,

    /// Latent heat absorbed from the surface by re-evaporation (+lambda * m_dot_evap), W.
    pub evaporation_latent_heat_absorption_w: f64,
}

#[derive(Debug, Clone, Serialize)]
pub struct PreparedCondensationRecord {
    pub period_label: &'static str,
    pub timestamp_iso_utc: &'static str,
    pub timestamp_utc_s: i64,
    pub time_s: f64,

    pub roof_inner_from_top_air: SurfaceCondensationElementOutput,
    pub south_wall_inner_from_main_air: SurfaceCondensationElementOutput,
    pub north_wall_inner_from_main_air: SurfaceCondensationElementOutput,
    pub lower_thermal_screen_from_main_air: SurfaceCondensationElementOutput,

    /// Whole-compartment instantaneous condensation rate across the tracked surfaces, kg/s.
    pub total_condensation_rate_kg_s: f64,

    /// Whole-compartment instantaneous latent heat released by condensation, W.
    pub total_condensation_latent_heat_w: f64,
}

#[derive(Debug, Clone, Serialize)]
pub struct PrepareCondensationOutput {
    pub preset_id: &'static str,
    pub execution_mode: &'static str,
    pub source_reference: &'static str,
    pub published_season_summary: PublishedSeasonCondensationSummaryOutput,
    pub records: Vec<PreparedCondensationRecord>,
}

fn saturation_vapour_pressure_tetens_pa(temperature_c: f64) -> f64 {
    610.78 * ((17.27 * temperature_c) / (temperature_c + 237.3)).exp()
}

#[allow(clippy::too_many_arguments)]
fn build_surface_condensation(
    surface_name: &'static str,
    adjacent_air_zone: &'static str,
    surface_area_m2: f64,
    air_temperature_c: f64,
    surface_temperature_c: f64,
    air_density_kg_m3: f64,
    air_vapour_pressure_pa: f64,
    hc_w_m2_k: f64,
    available_water_rate_kg_s: f64,
) -> SurfaceCondensationElementOutput {
    let air_temperature_k = air_temperature_c + 273.15;
    let surface_temperature_k = surface_temperature_c + 273.15;
    let mean_temperature_k = 0.5 * (air_temperature_k + surface_temperature_k);

    let surface_saturation_vapour_pressure_pa =
        saturation_vapour_pressure_tetens_pa(surface_temperature_c);
    let vapour_pressure_excess_pa =
        air_vapour_pressure_pa - surface_saturation_vapour_pressure_pa;

    let lewis_factor_a = (LEWIS_NUMBER.powf(LEWIS_EXPONENT_N - 1.0)
        * WATER_VAPOUR_MOLAR_MASS_KG_MOL)
        / (UNIVERSAL_GAS_CONSTANT_J_MOL_K
        * air_density_kg_m3
        * CP_AIR_J_KG_K
        * mean_temperature_k);

    let mass_transfer_coeff_km = lewis_factor_a * hc_w_m2_k;

    let condensation_rate_kg_s = surface_area_m2
        * mass_transfer_coeff_km
        * vapour_pressure_excess_pa.max(0.0);

    let potential_evaporation_rate_kg_s = surface_area_m2
        * mass_transfer_coeff_km
        * (-vapour_pressure_excess_pa).max(0.0);

    let evaporation_rate_kg_s = potential_evaporation_rate_kg_s
        .min(available_water_rate_kg_s)
        .max(0.0);

    let condensation_latent_heat_release_w =
        LATENT_HEAT_CONDENSATION_J_KG * condensation_rate_kg_s;
    let evaporation_latent_heat_absorption_w =
        LATENT_HEAT_CONDENSATION_J_KG * evaporation_rate_kg_s;

    SurfaceCondensationElementOutput {
        surface_name,
        adjacent_air_zone,
        surface_area_m2,
        air_temperature_c,
        surface_temperature_c,
        mean_temperature_k,
        air_density_kg_m3,
        air_vapour_pressure_pa,
        surface_saturation_vapour_pressure_pa,
        vapour_pressure_excess_pa,
        hc_w_m2_k,
        lewis_factor_a,
        mass_transfer_coeff_km,
        condensation_rate_kg_s,
        potential_evaporation_rate_kg_s,
        available_water_rate_kg_s,
        evaporation_rate_kg_s,
        condensation_latent_heat_release_w,
        evaporation_latent_heat_absorption_w,
    }
}

#[allow(clippy::too_many_arguments)]
fn build_record(
    period_label: &'static str,
    timestamp_iso_utc: &'static str,
    timestamp_utc_s: i64,
    time_s: f64,
    t_main_air_c: f64,
    rho_main_air: f64,
    pv_main_air_pa: f64,
    t_top_air_c: f64,
    rho_top_air: f64,
    pv_top_air_pa: f64,
    t_roof_inner_c: f64,
    hc_roof_inner: f64,
    t_south_wall_c: f64,
    hc_south_wall: f64,
    t_north_wall_c: f64,
    hc_north_wall: f64,
    t_lower_screen_c: f64,
    hc_lower_screen: f64,
    screen_available_water_kg_s: f64,
) -> PreparedCondensationRecord {
    let sloped_roof_area_m2 = FLOOR_AREA_M2 / ROOF_SLOPE_RAD.cos();

    let roof_inner_from_top_air = build_surface_condensation(
        "RoofInner",
        "TopAir",
        sloped_roof_area_m2,
        t_top_air_c,
        t_roof_inner_c,
        rho_top_air,
        pv_top_air_pa,
        hc_roof_inner,
        0.0,
    );

    let south_wall_inner_from_main_air = build_surface_condensation(
        "SouthWallInner",
        "MainAir",
        SOUTH_WALL_AREA_M2,
        t_main_air_c,
        t_south_wall_c,
        rho_main_air,
        pv_main_air_pa,
        hc_south_wall,
        0.0,
    );

    let north_wall_inner_from_main_air = build_surface_condensation(
        "NorthCorridorWallInner",
        "MainAir",
        NORTH_WALL_AREA_M2,
        t_main_air_c,
        t_north_wall_c,
        rho_main_air,
        pv_main_air_pa,
        hc_north_wall,
        0.0,
    );

    let lower_thermal_screen_from_main_air = build_surface_condensation(
        "LowerThermalScreen",
        "MainAir",
        FLOOR_AREA_M2,
        t_main_air_c,
        t_lower_screen_c,
        rho_main_air,
        pv_main_air_pa,
        hc_lower_screen,
        screen_available_water_kg_s,
    );

    let total_condensation_rate_kg_s =
        roof_inner_from_top_air.condensation_rate_kg_s
            + south_wall_inner_from_main_air.condensation_rate_kg_s
            + north_wall_inner_from_main_air.condensation_rate_kg_s
            + lower_thermal_screen_from_main_air.condensation_rate_kg_s;

    let total_condensation_latent_heat_w =
        LATENT_HEAT_CONDENSATION_J_KG * total_condensation_rate_kg_s;

    PreparedCondensationRecord {
        period_label,
        timestamp_iso_utc,
        timestamp_utc_s,
        time_s,
        roof_inner_from_top_air,
        south_wall_inner_from_main_air,
        north_wall_inner_from_main_air,
        lower_thermal_screen_from_main_air,
        total_condensation_rate_kg_s,
        total_condensation_latent_heat_w,
    }
}

fn reference_records() -> Vec<PreparedCondensationRecord> {
    vec![
        // 1. Winter night (11 Jan 2015 02:00 UTC): heavy condensation on cold roof (7.4 °C, p_sat=1029 Pa < p_v,TopAir=1159 Pa)
        //    and cold south wall (10.5 °C, p_sat=1270 Pa < p_v,MainAir=1531 Pa); warm north wall & thermal screen stay above dew point.
        build_record(
            "winter_night",
            "2015-01-11T02:00:00Z",
            1_420_941_600,
            3_376_800.0,
            17.2,
            1.214_470_796_7,
            1_530.641_892_020_5,
            11.4,
            1.240_907_556_9,
            1_159.266_095_015_4,
            7.4,
            2.45,
            10.5,
            2.12,
            16.1,
            1.35,
            15.8,
            1.65,
            0.000_05,
        ),
        // 2. Winter noon (11 Jan 2015 12:00 UTC): condensation continues on cool roof (14.2 °C, p_sat=1619 Pa < p_v,TopAir=1681 Pa);
        //    any morning moisture on the lower screen re-evaporates up to available water limit.
        build_record(
            "winter_noon",
            "2015-01-11T12:00:00Z",
            1_420_977_600,
            3_412_800.0,
            20.4,
            1.199_440_958_2,
            1_797.548_947_067_6,
            18.9,
            1.206_101_282_0,
            1_681.410_760_108_4,
            14.2,
            2.58,
            16.5,
            2.05,
            18.8,
            1.53,
            20.2,
            1.20,
            0.000_12,
        ),
        // 3. Spring equinox noon (20 Mar 2015 12:00 UTC): all surfaces warmer than indoor dew point -> zero condensation
        build_record(
            "equinox_noon",
            "2015-03-20T12:00:00Z",
            1_426_852_800,
            9_288_000.0,
            23.6,
            1.182_241_541_7,
            2_097.307_883_867_6,
            22.8,
            1.186_101_008_2,
            1_942.878_243_875_0,
            20.1,
            2.15,
            21.2,
            1.75,
            21.9,
            1.56,
            23.4,
            1.10,
            0.0,
        ),
        // 4. Summer afternoon (16 Jul 2015 13:00 UTC): warm dry surfaces well above dew point -> zero condensation
        build_record(
            "summer_afternoon",
            "2015-07-16T13:00:00Z",
            1_437_051_600,
            19_486_800.0,
            31.2,
            1.147_601_263_3,
            2_817.187_494_874_6,
            34.6,
            1.134_247_491_1,
            2_969.664_428_989_8,
            34.1,
            1.60,
            30.1,
            1.35,
            28.9,
            1.72,
            32.5,
            1.50,
            0.0,
        ),
    ]
}

pub fn prepare_condensation_service(
    payload: PrepareCondensationInput,
) -> Result<PrepareCondensationOutput, AppError> {
    let preset = payload
        .preset_id
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .unwrap_or(DEFAULT_CONDENSATION_PRESET_ID);

    if preset != DEFAULT_CONDENSATION_PRESET_ID {
        return Err(AppError::BadRequest(format!(
            "unknown simulation condensation preset '{preset}'; \
             supported preset is '{DEFAULT_CONDENSATION_PRESET_ID}'"
        )));
    }

    let filter = payload
        .period_filter
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .unwrap_or("all");

    let all_records = reference_records();

    let records: Vec<PreparedCondensationRecord> = if filter == "all" {
        all_records
    } else {
        let filtered: Vec<PreparedCondensationRecord> = all_records
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

    Ok(PrepareCondensationOutput {
        preset_id: DEFAULT_CONDENSATION_PRESET_ID,
        execution_mode: "reference_dataset_direct",
        source_reference: "Sourisseau et al. (2026) PLOS One 21(2): e0340619, \
                           Section 3.5.1, Section 4.4.1 (Fig. 13) & De Zwart (1996) Lewis analogy",
        published_season_summary: PublishedSeasonCondensationSummaryOutput {
            condensed_share_of_transpired_water_pct: 10.4,
            roof_condensation_333d_kg_per_m2_roof: 80.4,
            roof_condensation_annualized_kg_per_m2_roof_yr: 88.0,
            cover_condensation_333d_kg_per_m2_cover: 75.5,
            cover_condensation_annualized_kg_per_m2_cover_yr: 82.8,
            south_wall_share_of_roof_condensation_pct: 8.9,
            south_wall_glass_share_pct: 90.0,
            south_wall_curtain_share_pct: 10.0,
            east_wall_total_condensation_kg: 920.0,
            north_wall_total_condensation_kg: 829.0,
            horizontal_screens_total_condensation_kg: 34.0,
            agronomic_limit_note:
            "Condensation conditions provide surface moisture removal and risk exposure indicators, \
                 not a calibrated fungal disease probability model (Section 14).",
        },
        records,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lewis_analogy_condensation_and_bounded_evaporation_hold_across_all_records() {
        let output =
            prepare_condensation_service(PrepareCondensationInput::default())
                .expect("default condensation preset should succeed");

        assert_eq!(output.records.len(), 4);

        for record in &output.records {
            for elem in [
                &record.roof_inner_from_top_air,
                &record.south_wall_inner_from_main_air,
                &record.north_wall_inner_from_main_air,
                &record.lower_thermal_screen_from_main_air,
            ] {
                let expected_a = (LEWIS_NUMBER.powf(LEWIS_EXPONENT_N - 1.0)
                    * WATER_VAPOUR_MOLAR_MASS_KG_MOL)
                    / (UNIVERSAL_GAS_CONSTANT_J_MOL_K
                    * elem.air_density_kg_m3
                    * CP_AIR_J_KG_K
                    * elem.mean_temperature_k);
                assert!((elem.lewis_factor_a - expected_a).abs() < 1e-18);

                let expected_km = expected_a * elem.hc_w_m2_k;
                assert!(
                    (elem.mass_transfer_coeff_km - expected_km).abs() < 1e-16
                );

                let excess = elem.air_vapour_pressure_pa
                    - elem.surface_saturation_vapour_pressure_pa;
                let expected_cond = elem.surface_area_m2
                    * expected_km
                    * excess.max(0.0);
                assert!(
                    (elem.condensation_rate_kg_s - expected_cond).abs() < 1e-12
                );

                let expected_pot_evap = elem.surface_area_m2
                    * expected_km
                    * (-excess).max(0.0);
                let expected_evap = expected_pot_evap
                    .min(elem.available_water_rate_kg_s)
                    .max(0.0);
                assert!(
                    (elem.evaporation_rate_kg_s - expected_evap).abs() < 1e-12
                );

                let expected_q_cond =
                    LATENT_HEAT_CONDENSATION_J_KG * expected_cond;
                assert!(
                    (elem.condensation_latent_heat_release_w - expected_q_cond)
                        .abs()
                        < 1e-6
                );
            }
        }

        // Winter night has active condensation on roof and south wall; summer afternoon has zero.
        assert!(output.records[0].total_condensation_rate_kg_s > 0.001);
        assert_eq!(output.records[3].total_condensation_rate_kg_s, 0.0);
    }
}