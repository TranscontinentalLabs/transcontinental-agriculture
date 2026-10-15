use serde::{Deserialize, Serialize};

use crate::error::AppError;

pub const DEFAULT_AIRFLOW_TRANSPORT_PRESET_ID: &str = "ctifl_nantes_2014_2015";

/// Universal gas constant R, J/(mol·K).
pub const UNIVERSAL_GAS_CONSTANT_J_MOL_K: f64 = 8.314_462_618;

/// Species molar masses M_i, kg/mol.
pub const DRY_AIR_MOLAR_MASS_KG_MOL: f64 = 0.028_965_4;
pub const WATER_VAPOUR_MOLAR_MASS_KG_MOL: f64 = 0.018_015_28;
pub const CO2_MOLAR_MASS_KG_MOL: f64 = 0.044_009_5;

/// Species specific heat capacities c_p,i, J/(kg·K).
pub const CP_DRY_AIR_J_KG_K: f64 = 1_006.0;
pub const CP_WATER_VAPOUR_J_KG_K: f64 = 1_860.0;
pub const CP_CO2_J_KG_K: f64 = 844.0;

/// Reference latent heat of vaporization at 0 °C, J/kg.
pub const LATENT_HEAT_VAPORIZATION_J_KG: f64 = 2_501_000.0;

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PrepareAirflowTransportInput {
    /// Optional preset identifier. Defaults to `"ctifl_nantes_2014_2015"`.
    pub preset_id: Option<String>,

    /// Optional filter matching Models 3–10 period labels:
    /// `"winter_night"`, `"winter_noon"`, `"equinox_noon"`, `"summer_afternoon"`, or `"all"`.
    pub period_filter: Option<String>,
}

#[derive(Debug, Clone, Copy, Serialize)]
pub struct TransportPhysicalConstantsOutput {
    pub universal_gas_constant_j_mol_k: f64,
    pub dry_air_molar_mass_kg_mol: f64,
    pub water_vapour_molar_mass_kg_mol: f64,
    pub co2_molar_mass_kg_mol: f64,
    pub cp_dry_air_j_kg_k: f64,
    pub cp_water_vapour_j_kg_k: f64,
    pub cp_co2_j_kg_k: f64,
    pub latent_heat_vaporization_j_kg: f64,
    pub connector_accounting_warning: &'static str,
}

#[derive(Debug, Clone, Copy, Serialize)]
pub struct ZoneGasBoundaryStateOutput {
    pub temperature_c: f64,
    pub temperature_k: f64,
    pub dry_air_partial_pressure_pa: f64,
    pub vapour_partial_pressure_pa: f64,
    pub co2_partial_pressure_pa: f64,
}

#[derive(Debug, Clone, Copy, Serialize)]
pub struct AirflowPathTransportOutput {
    pub volume_1_name: &'static str,
    pub volume_2_name: &'static str,
    pub exchange_mode: &'static str,

    /// Volumetric flow magnitude F (m³/s) from Models 8, 9, or 10.
    pub volumetric_flow_m3_s: f64,

    /// Direction coefficients (k1, k2) in Eq. (12):
    /// Bilateral: (1.0, 1.0); Unilateral: (1.0, 0.0) if p1 > p2 else (0.0, 1.0).
    pub k1: f64,
    pub k2: f64,

    pub state_1: ZoneGasBoundaryStateOutput,
    pub state_2: ZoneGasBoundaryStateOutput,

    /// Net species mass flows from Volume 1 to Volume 2 (Eq. 12), kg/s.
    /// Positive means net mass leaves Volume 1 and enters Volume 2.
    pub net_dry_air_mass_flow_kg_s: f64,
    pub net_water_vapour_mass_flow_kg_s: f64,
    pub net_co2_mass_flow_kg_s: f64,

    /// Net sensible heat flow from Volume 1 to Volume 2 (Eq. 13), W.
    pub sensible_heat_flow_w: f64,

    /// Associated vapour latent energy flow (m_dot_vapour * lambda_0), W.
    /// Reported separately for diagnostics; do not add to sensible_heat_flow_w
    /// if the receiving air-volume mass port already carries vapour enthalpy.
    pub diagnostic_vapour_latent_flow_w: f64,
}

#[derive(Debug, Clone, Serialize)]
pub struct PreparedAirflowTransportRecord {
    pub period_label: &'static str,
    pub timestamp_iso_utc: &'static str,
    pub timestamp_utc_s: i64,
    pub time_s: f64,

    /// Bilateral transport through roof vents (Model 8: TopAir <-> Outdoor).
    pub roof_vents_top_to_outdoor: AirflowPathTransportOutput,

    /// Unilateral pressure-crack transport through roof cover (Model 9: TopAir <-> Outdoor).
    pub roof_crack_top_to_outdoor: AirflowPathTransportOutput,

    /// Bilateral transport across horizontal screens (Model 10: MainAir <-> TopAir).
    pub screens_main_to_top: AirflowPathTransportOutput,
}

#[derive(Debug, Clone, Serialize)]
pub struct PrepareAirflowTransportOutput {
    pub preset_id: &'static str,
    pub execution_mode: &'static str,
    pub source_reference: &'static str,
    pub constants: TransportPhysicalConstantsOutput,
    pub records: Vec<PreparedAirflowTransportRecord>,
}

fn build_transport_path(
    volume_1_name: &'static str,
    volume_2_name: &'static str,
    exchange_mode: &'static str,
    volumetric_flow_m3_s: f64,
    k1: f64,
    k2: f64,
    state_1: ZoneGasBoundaryStateOutput,
    state_2: ZoneGasBoundaryStateOutput,
) -> AirflowPathTransportOutput {
    let m_da1 = volumetric_flow_m3_s
        * (DRY_AIR_MOLAR_MASS_KG_MOL / UNIVERSAL_GAS_CONSTANT_J_MOL_K)
        * k1
        * (state_1.dry_air_partial_pressure_pa / state_1.temperature_k);
    let m_da2 = volumetric_flow_m3_s
        * (DRY_AIR_MOLAR_MASS_KG_MOL / UNIVERSAL_GAS_CONSTANT_J_MOL_K)
        * k2
        * (state_2.dry_air_partial_pressure_pa / state_2.temperature_k);
    let net_dry_air_mass_flow_kg_s = m_da1 - m_da2;

    let m_w1 = volumetric_flow_m3_s
        * (WATER_VAPOUR_MOLAR_MASS_KG_MOL / UNIVERSAL_GAS_CONSTANT_J_MOL_K)
        * k1
        * (state_1.vapour_partial_pressure_pa / state_1.temperature_k);
    let m_w2 = volumetric_flow_m3_s
        * (WATER_VAPOUR_MOLAR_MASS_KG_MOL / UNIVERSAL_GAS_CONSTANT_J_MOL_K)
        * k2
        * (state_2.vapour_partial_pressure_pa / state_2.temperature_k);
    let net_water_vapour_mass_flow_kg_s = m_w1 - m_w2;

    let m_co2_1 = volumetric_flow_m3_s
        * (CO2_MOLAR_MASS_KG_MOL / UNIVERSAL_GAS_CONSTANT_J_MOL_K)
        * k1
        * (state_1.co2_partial_pressure_pa / state_1.temperature_k);
    let m_co2_2 = volumetric_flow_m3_s
        * (CO2_MOLAR_MASS_KG_MOL / UNIVERSAL_GAS_CONSTANT_J_MOL_K)
        * k2
        * (state_2.co2_partial_pressure_pa / state_2.temperature_k);
    let net_co2_mass_flow_kg_s = m_co2_1 - m_co2_2;

    let sensible_heat_flow_w = (m_da1 * CP_DRY_AIR_J_KG_K * state_1.temperature_c
        - m_da2 * CP_DRY_AIR_J_KG_K * state_2.temperature_c)
        + (m_w1 * CP_WATER_VAPOUR_J_KG_K * state_1.temperature_c
        - m_w2 * CP_WATER_VAPOUR_J_KG_K * state_2.temperature_c)
        + (m_co2_1 * CP_CO2_J_KG_K * state_1.temperature_c
        - m_co2_2 * CP_CO2_J_KG_K * state_2.temperature_c);

    let diagnostic_vapour_latent_flow_w =
        net_water_vapour_mass_flow_kg_s * LATENT_HEAT_VAPORIZATION_J_KG;

    AirflowPathTransportOutput {
        volume_1_name,
        volume_2_name,
        exchange_mode,
        volumetric_flow_m3_s,
        k1,
        k2,
        state_1,
        state_2,
        net_dry_air_mass_flow_kg_s,
        net_water_vapour_mass_flow_kg_s,
        net_co2_mass_flow_kg_s,
        sensible_heat_flow_w,
        diagnostic_vapour_latent_flow_w,
    }
}

fn reference_records() -> Vec<PreparedAirflowTransportRecord> {
    // 1. Winter night states (11 Jan 2015 02:00 UTC)
    let wn_main = ZoneGasBoundaryStateOutput {
        temperature_c: 17.2,
        temperature_k: 290.35,
        dry_air_partial_pressure_pa: 100_267.358_107_979_5,
        vapour_partial_pressure_pa: 1_530.641_892_020_5,
        co2_partial_pressure_pa: 44.791_120,
    };
    let wn_top = ZoneGasBoundaryStateOutput {
        temperature_c: 11.4,
        temperature_k: 284.55,
        dry_air_partial_pressure_pa: 100_635.733_904_984_6,
        vapour_partial_pressure_pa: 1_159.266_095_015_4,
        co2_partial_pressure_pa: 42.244_925,
    };
    let wn_out = ZoneGasBoundaryStateOutput {
        temperature_c: 4.5,
        temperature_k: 277.65,
        dry_air_partial_pressure_pa: 101_058.774_096_047_8,
        vapour_partial_pressure_pa: 741.225_903_952_2,
        co2_partial_pressure_pa: 40.720_000,
    };

    // 2. Winter noon states (11 Jan 2015 12:00 UTC)
    let wd_main = ZoneGasBoundaryStateOutput {
        temperature_c: 20.4,
        temperature_k: 293.55,
        dry_air_partial_pressure_pa: 99_950.451_052_932_4,
        vapour_partial_pressure_pa: 1_797.548_947_067_6,
        co2_partial_pressure_pa: 69.188_640,
    };
    let wd_top = ZoneGasBoundaryStateOutput {
        temperature_c: 18.9,
        temperature_k: 292.05,
        dry_air_partial_pressure_pa: 100_064.589_239_891_6,
        vapour_partial_pressure_pa: 1_681.410_760_108_4,
        co2_partial_pressure_pa: 52.907_920,
    };
    let wd_out = ZoneGasBoundaryStateOutput {
        temperature_c: 9.2,
        temperature_k: 282.35,
        dry_air_partial_pressure_pa: 100_865.643_889_994_4,
        vapour_partial_pressure_pa: 884.356_110_005_6,
        co2_partial_pressure_pa: 40.700_000,
    };

    // 3. Spring equinox noon states (20 Mar 2015 12:00 UTC)
    let eq_main = ZoneGasBoundaryStateOutput {
        temperature_c: 23.6,
        temperature_k: 296.75,
        dry_air_partial_pressure_pa: 99_400.692_116_132_4,
        vapour_partial_pressure_pa: 2_097.307_883_867_6,
        co2_partial_pressure_pa: 73.078_560,
    };
    let eq_top = ZoneGasBoundaryStateOutput {
        temperature_c: 22.8,
        temperature_k: 295.95,
        dry_air_partial_pressure_pa: 99_553.121_756_125_0,
        vapour_partial_pressure_pa: 1_942.878_243_875_0,
        co2_partial_pressure_pa: 54.807_840,
    };
    let eq_out = ZoneGasBoundaryStateOutput {
        temperature_c: 14.8,
        temperature_k: 287.95,
        dry_air_partial_pressure_pa: 100_422.587_901_933_7,
        vapour_partial_pressure_pa: 1_077.412_098_066_3,
        co2_partial_pressure_pa: 40.600_000,
    };

    // 4. Summer afternoon states (16 Jul 2015 13:00 UTC)
    let su_main = ZoneGasBoundaryStateOutput {
        temperature_c: 31.2,
        temperature_k: 304.35,
        dry_air_partial_pressure_pa: 98_505.812_505_125_4,
        vapour_partial_pressure_pa: 2_817.187_494_874_6,
        co2_partial_pressure_pa: 41.542_430,
    };
    let su_top = ZoneGasBoundaryStateOutput {
        temperature_c: 34.6,
        temperature_k: 307.75,
        dry_air_partial_pressure_pa: 98_351.335_571_010_2,
        vapour_partial_pressure_pa: 2_969.664_428_989_8,
        co2_partial_pressure_pa: 41.035_005,
    };
    let su_out = ZoneGasBoundaryStateOutput {
        temperature_c: 28.4,
        temperature_k: 301.55,
        dry_air_partial_pressure_pa: 99_468.006_223_354_5,
        vapour_partial_pressure_pa: 1_856.993_776_645_5,
        co2_partial_pressure_pa: 40.530_000,
    };

    vec![
        PreparedAirflowTransportRecord {
            period_label: "winter_night",
            timestamp_iso_utc: "2015-01-11T02:00:00Z",
            timestamp_utc_s: 1_420_941_600,
            time_s: 3_376_800.0,
            roof_vents_top_to_outdoor: build_transport_path(
                "TopAir",
                "Outdoor",
                "bilateral",
                0.432_922_213_651,
                1.0,
                1.0,
                wn_top,
                wn_out,
            ),
            roof_crack_top_to_outdoor: build_transport_path(
                "TopAir",
                "Outdoor",
                "unilateral_infiltration",
                0.128_098_196_871,
                0.0,
                1.0,
                wn_top,
                wn_out,
            ),
            screens_main_to_top: build_transport_path(
                "MainAir",
                "TopAir",
                "bilateral",
                7.684_875_334_778,
                1.0,
                1.0,
                wn_main,
                wn_top,
            ),
        },
        PreparedAirflowTransportRecord {
            period_label: "winter_noon",
            timestamp_iso_utc: "2015-01-11T12:00:00Z",
            timestamp_utc_s: 1_420_977_600,
            time_s: 3_412_800.0,
            roof_vents_top_to_outdoor: build_transport_path(
                "TopAir",
                "Outdoor",
                "bilateral",
                2.944_734_423_792,
                1.0,
                1.0,
                wd_top,
                wd_out,
            ),
            roof_crack_top_to_outdoor: build_transport_path(
                "TopAir",
                "Outdoor",
                "unilateral_infiltration",
                0.110_802_997_201,
                0.0,
                1.0,
                wd_top,
                wd_out,
            ),
            screens_main_to_top: build_transport_path(
                "MainAir",
                "TopAir",
                "bilateral",
                837.097_974_618_460,
                1.0,
                1.0,
                wd_main,
                wd_top,
            ),
        },
        PreparedAirflowTransportRecord {
            period_label: "equinox_noon",
            timestamp_iso_utc: "2015-03-20T12:00:00Z",
            timestamp_utc_s: 1_426_852_800,
            time_s: 9_288_000.0,
            roof_vents_top_to_outdoor: build_transport_path(
                "TopAir",
                "Outdoor",
                "bilateral",
                11.698_611_405_590,
                1.0,
                1.0,
                eq_top,
                eq_out,
            ),
            roof_crack_top_to_outdoor: build_transport_path(
                "TopAir",
                "Outdoor",
                "unilateral_infiltration",
                0.110_802_997_201,
                0.0,
                1.0,
                eq_top,
                eq_out,
            ),
            screens_main_to_top: build_transport_path(
                "MainAir",
                "TopAir",
                "bilateral",
                642.209_154_581_549,
                1.0,
                1.0,
                eq_main,
                eq_top,
            ),
        },
        PreparedAirflowTransportRecord {
            period_label: "summer_afternoon",
            timestamp_iso_utc: "2015-07-16T13:00:00Z",
            timestamp_utc_s: 1_437_051_600,
            time_s: 19_486_800.0,
            roof_vents_top_to_outdoor: build_transport_path(
                "TopAir",
                "Outdoor",
                "bilateral",
                25.518_517_481_493,
                1.0,
                1.0,
                su_top,
                su_out,
            ),
            roof_crack_top_to_outdoor: build_transport_path(
                "TopAir",
                "Outdoor",
                "unilateral_infiltration",
                0.110_802_997_201,
                0.0,
                1.0,
                su_top,
                su_out,
            ),
            screens_main_to_top: build_transport_path(
                "MainAir",
                "TopAir",
                "bilateral",
                13.596_720_165_133,
                1.0,
                1.0,
                su_main,
                su_top,
            ),
        },
    ]
}

pub fn prepare_airflow_transport_service(
    payload: PrepareAirflowTransportInput,
) -> Result<PrepareAirflowTransportOutput, AppError> {
    let preset = payload
        .preset_id
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .unwrap_or(DEFAULT_AIRFLOW_TRANSPORT_PRESET_ID);

    if preset != DEFAULT_AIRFLOW_TRANSPORT_PRESET_ID {
        return Err(AppError::BadRequest(format!(
            "unknown simulation airflow transport preset '{preset}'; \
             supported preset is '{DEFAULT_AIRFLOW_TRANSPORT_PRESET_ID}'"
        )));
    }

    let filter = payload
        .period_filter
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .unwrap_or("all");

    let all_records = reference_records();

    let records: Vec<PreparedAirflowTransportRecord> = if filter == "all" {
        all_records
    } else {
        let filtered: Vec<PreparedAirflowTransportRecord> = all_records
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

    Ok(PrepareAirflowTransportOutput {
        preset_id: DEFAULT_AIRFLOW_TRANSPORT_PRESET_ID,
        execution_mode: "reference_dataset_direct",
        source_reference: "Sourisseau et al. (2026) PLOS One 21(2): e0340619, \
                           Section 3.4.4 (Eqs. 12-13) & TiGwerLib 0.7.0 Airflow transport base class",
        constants: TransportPhysicalConstantsOutput {
            universal_gas_constant_j_mol_k: UNIVERSAL_GAS_CONSTANT_J_MOL_K,
            dry_air_molar_mass_kg_mol: DRY_AIR_MOLAR_MASS_KG_MOL,
            water_vapour_molar_mass_kg_mol: WATER_VAPOUR_MOLAR_MASS_KG_MOL,
            co2_molar_mass_kg_mol: CO2_MOLAR_MASS_KG_MOL,
            cp_dry_air_j_kg_k: CP_DRY_AIR_J_KG_K,
            cp_water_vapour_j_kg_k: CP_WATER_VAPOUR_J_KG_K,
            cp_co2_j_kg_k: CP_CO2_J_KG_K,
            latent_heat_vaporization_j_kg: LATENT_HEAT_VAPORIZATION_J_KG,
            connector_accounting_warning:
            "Sensible heat (Eq. 13) and vapour latent energy are kept separate; \
                 never add diagnostic_vapour_latent_flow_w to a heat port if the air-volume \
                 mass port already carries water-vapour enthalpy.",
        },
        records,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn equations_12_and_13_hold_for_all_paths_and_records() {
        let output = prepare_airflow_transport_service(
            PrepareAirflowTransportInput::default(),
        )
            .expect("default airflow transport preset should succeed");

        assert_eq!(output.records.len(), 4);

        for record in &output.records {
            for path in [
                &record.roof_vents_top_to_outdoor,
                &record.roof_crack_top_to_outdoor,
                &record.screens_main_to_top,
            ] {
                let f = path.volumetric_flow_m3_s;
                let s1 = &path.state_1;
                let s2 = &path.state_2;

                let m_da1 = f
                    * (DRY_AIR_MOLAR_MASS_KG_MOL / UNIVERSAL_GAS_CONSTANT_J_MOL_K)
                    * path.k1
                    * (s1.dry_air_partial_pressure_pa / s1.temperature_k);
                let m_da2 = f
                    * (DRY_AIR_MOLAR_MASS_KG_MOL / UNIVERSAL_GAS_CONSTANT_J_MOL_K)
                    * path.k2
                    * (s2.dry_air_partial_pressure_pa / s2.temperature_k);
                assert!(
                    (path.net_dry_air_mass_flow_kg_s - (m_da1 - m_da2)).abs()
                        < 1e-12
                );

                let m_w1 = f
                    * (WATER_VAPOUR_MOLAR_MASS_KG_MOL
                    / UNIVERSAL_GAS_CONSTANT_J_MOL_K)
                    * path.k1
                    * (s1.vapour_partial_pressure_pa / s1.temperature_k);
                let m_w2 = f
                    * (WATER_VAPOUR_MOLAR_MASS_KG_MOL
                    / UNIVERSAL_GAS_CONSTANT_J_MOL_K)
                    * path.k2
                    * (s2.vapour_partial_pressure_pa / s2.temperature_k);
                assert!(
                    (path.net_water_vapour_mass_flow_kg_s - (m_w1 - m_w2))
                        .abs()
                        < 1e-12
                );

                let m_co2_1 = f
                    * (CO2_MOLAR_MASS_KG_MOL / UNIVERSAL_GAS_CONSTANT_J_MOL_K)
                    * path.k1
                    * (s1.co2_partial_pressure_pa / s1.temperature_k);
                let m_co2_2 = f
                    * (CO2_MOLAR_MASS_KG_MOL / UNIVERSAL_GAS_CONSTANT_J_MOL_K)
                    * k2_factor(path.k2)
                    * (s2.co2_partial_pressure_pa / s2.temperature_k);
                assert!(
                    (path.net_co2_mass_flow_kg_s - (m_co2_1 - m_co2_2)).abs()
                        < 1e-12
                );

                let expected_q_sens = (m_da1 * CP_DRY_AIR_J_KG_K * s1.temperature_c
                    - m_da2 * CP_DRY_AIR_J_KG_K * s2.temperature_c)
                    + (m_w1 * CP_WATER_VAPOUR_J_KG_K * s1.temperature_c
                    - m_w2 * CP_WATER_VAPOUR_J_KG_K * s2.temperature_c)
                    + (m_co2_1 * CP_CO2_J_KG_K * s1.temperature_c
                    - m_co2_2 * CP_CO2_J_KG_K * s2.temperature_c);
                assert!(
                    (path.sensible_heat_flow_w - expected_q_sens).abs() < 1e-9
                );
            }
        }
    }

    fn k2_factor(k2: f64) -> f64 {
        k2
    }
}