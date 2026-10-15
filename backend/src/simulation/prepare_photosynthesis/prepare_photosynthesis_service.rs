use serde::{Deserialize, Serialize};

use crate::error::AppError;

pub const DEFAULT_PHOTOSYNTHESIS_PRESET_ID: &str = "ctifl_nantes_2014_2015";

/// Air-to-stomatal CO2 ratio eta_{air->stom} (-).
pub const ETA_AIR_TO_STOM: f64 = 0.67;

/// Potential leaf electron transport at 25 °C, umol electrons/(m²_leaf·s).
pub const J_25_LEAF_UMOL_M2_S: f64 = 210.0;

/// Arrhenius / Eyring temperature-response constants for J_pot.
pub const T_25_K: f64 = 298.15;
pub const E_J_J_MOL: f64 = 37_000.0;
pub const H_DEACTIVATION_J_MOL: f64 = 220_000.0;
pub const S_ENTROPY_J_MOL_K: f64 = 710.0;
pub const R_GAS_J_MOL_K: f64 = 8.314_462_618;

/// Light-response curvature parameters for electron transport J.
pub const ALPHA_ELECTRON_YIELD: f64 = 0.385;
pub const THETA_CURVATURE: f64 = 0.70;

/// CO2 compensation point temperature coefficient c_Gamma, umol/(mol·K).
pub const C_GAMMA_UMOL_MOL_K: f64 = 1.70;

/// Carbohydrate (CH2O) molar mass conversion: 30 g/mol = 0.030 mg/umol.
pub const M_CH2O_MG_PER_UMOL: f64 = 0.030;

/// Maximum carbohydrate buffer capacity C_max, mg/m².
pub const C_BUFFER_MAX_MG_M2: f64 = 20_000.0;

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PreparePhotosynthesisInput {
    /// Optional preset identifier. Defaults to `"ctifl_nantes_2014_2015"`.
    pub preset_id: Option<String>,

    /// Optional filter matching Models 3–18 period labels:
    /// `"winter_night"`, `"winter_noon"`, `"equinox_noon"`, `"summer_afternoon"`, or `"all"`.
    pub period_filter: Option<String>,
}

#[derive(Debug, Clone, Copy, Serialize)]
pub struct PhotosynthesisParametersOutput {
    pub cultivar: &'static str,
    pub eta_air_to_stom: f64,
    pub j_25_leaf_umol_m2_s: f64,
    pub t_25_k: f64,
    pub e_j_j_mol: f64,
    pub h_deactivation_j_mol: f64,
    pub s_entropy_j_mol_k: f64,
    pub r_gas_j_mol_k: f64,
    pub alpha_electron_yield: f64,
    pub theta_curvature: f64,
    pub c_gamma_umol_mol_k: f64,
    pub m_ch2o_mg_per_umol: f64,
    pub c_buffer_max_mg_m2: f64,
}

#[derive(Debug, Clone, Serialize)]
pub struct PreparedPhotosynthesisRecord {
    pub period_label: &'static str,
    pub timestamp_iso_utc: &'static str,
    pub timestamp_utc_s: i64,
    pub time_s: f64,

    pub leaf_area_index: f64,
    pub leaf_temperature_c: f64,
    pub leaf_temperature_k: f64,

    /// Absorbed PAR photon flux density from Model 6, umol/(m²·s).
    pub absorbed_par_umol_m2_s: f64,

    /// Indoor CO2 in MainAir and stomatal CO2 = eta * CO2_air, ppm (umol/mol).
    pub co2_air_ppm: f64,
    pub co2_stomatal_ppm: f64,

    /// Stored carbohydrate buffer state C_buffer, mg/m².
    pub carbohydrate_buffer_mg_m2: f64,

    /// J_{25,can} = LAI * J_{25,leaf}, umol/(m²·s).
    pub j_25_canopy_umol_m2_s: f64,

    /// Temperature-dependent potential electron transport J_pot, umol/(m²·s).
    pub j_pot_umol_m2_s: f64,

    /// Actual electron transport rate J, umol/(m²·s).
    pub electron_transport_j_umol_m2_s: f64,

    /// CO2 compensation point Gamma, ppm (umol/mol).
    pub gamma_compensation_ppm: f64,

    /// Gross photosynthesis P and photorespiration R, umol CO2/(m²·s).
    pub gross_photosynthesis_p_umol_m2_s: f64,
    pub photorespiration_r_umol_m2_s: f64,
    pub net_assimilation_p_minus_r_umol_m2_s: f64,

    /// Buffer saturation factor h_buffer = 1 / (1 + exp(0.005 * (C_buffer - C_max))).
    pub h_buffer: f64,

    /// Net carbohydrate supply to buffer F_{air->buffer} = M_CH2O * h_buffer * (P - R), mg/(m²·s).
    pub f_air_to_buffer_mg_m2_s: f64,

    /// Counterfactual carbohydrate supply if screens were retracted (u = 0), mg/(m²·s).
    pub unscreened_f_air_to_buffer_mg_m2_s: f64,

    /// Carbohydrate production reduction due to active screen deployment, mg/(m²·s).
    pub screen_photosynthesis_loss_mg_m2_s: f64,
}

#[derive(Debug, Clone, Serialize)]
pub struct PreparePhotosynthesisOutput {
    pub preset_id: &'static str,
    pub execution_mode: &'static str,
    pub source_reference: &'static str,
    pub parameters: PhotosynthesisParametersOutput,
    pub records: Vec<PreparedPhotosynthesisRecord>,
}

struct RecordSpec {
    period_label: &'static str,
    timestamp_iso_utc: &'static str,
    timestamp_utc_s: i64,
    time_s: f64,
    leaf_area_index: f64,
    leaf_temperature_c: f64,
    absorbed_par_umol_m2_s: f64,
    unscreened_absorbed_par_umol_m2_s: f64,
    co2_air_ppm: f64,
    carbohydrate_buffer_mg_m2: f64,
}

pub fn electron_transport_j(j_pot: f64, par_umol_m2_s: f64) -> f64 {
    let sum = j_pot + ALPHA_ELECTRON_YIELD * par_umol_m2_s;
    let discriminant = (sum.powi(2)
        - 4.0 * THETA_CURVATURE * j_pot * ALPHA_ELECTRON_YIELD * par_umol_m2_s)
        .max(0.0);
    (sum - discriminant.sqrt()) / (2.0 * THETA_CURVATURE)
}

fn evaluate_record(spec: RecordSpec) -> PreparedPhotosynthesisRecord {
    let leaf_temperature_k = spec.leaf_temperature_c + 273.15;
    let co2_stomatal_ppm = ETA_AIR_TO_STOM * spec.co2_air_ppm;

    let j_25_canopy_umol_m2_s = spec.leaf_area_index * J_25_LEAF_UMOL_M2_S;

    let arrhenius = ((E_J_J_MOL * (leaf_temperature_k - T_25_K))
        / (R_GAS_J_MOL_K * leaf_temperature_k * T_25_K))
        .exp();

    let deact_num = 1.0
        + ((S_ENTROPY_J_MOL_K * T_25_K - H_DEACTIVATION_J_MOL)
        / (R_GAS_J_MOL_K * T_25_K))
        .exp();
    let deact_den = 1.0
        + ((S_ENTROPY_J_MOL_K * leaf_temperature_k - H_DEACTIVATION_J_MOL)
        / (R_GAS_J_MOL_K * leaf_temperature_k))
        .exp();

    let j_pot_umol_m2_s = j_25_canopy_umol_m2_s * arrhenius * (deact_num / deact_den);

    let electron_transport_j_umol_m2_s =
        electron_transport_j(j_pot_umol_m2_s, spec.absorbed_par_umol_m2_s);

    let ratio_j25_jpot = j_25_canopy_umol_m2_s / j_pot_umol_m2_s;
    let gamma_compensation_ppm = C_GAMMA_UMOL_MOL_K
        * spec.leaf_temperature_c
        * ratio_j25_jpot
        + 20.0 * C_GAMMA_UMOL_MOL_K * (1.0 - ratio_j25_jpot);

    // Vanthoor implementation guard against negative assimilation when CO2_stom < Gamma.
    let gross_photosynthesis_p_umol_m2_s = (electron_transport_j_umol_m2_s / 4.0)
        * ((co2_stomatal_ppm - gamma_compensation_ppm).max(0.0)
        / (co2_stomatal_ppm + 2.0 * gamma_compensation_ppm));

    let photorespiration_r_umol_m2_s = gross_photosynthesis_p_umol_m2_s
        * (gamma_compensation_ppm / co2_stomatal_ppm);

    let net_assimilation_p_minus_r_umol_m2_s =
        gross_photosynthesis_p_umol_m2_s - photorespiration_r_umol_m2_s;

    let h_buffer = 1.0
        / (1.0
        + (0.005 * (spec.carbohydrate_buffer_mg_m2 - C_BUFFER_MAX_MG_M2))
        .exp());

    let f_air_to_buffer_mg_m2_s =
        M_CH2O_MG_PER_UMOL * h_buffer * net_assimilation_p_minus_r_umol_m2_s;

    // Counterfactual unscreened evaluation
    let j_unscreened = electron_transport_j(
        j_pot_umol_m2_s,
        spec.unscreened_absorbed_par_umol_m2_s,
    );
    let p_unscreened = (j_unscreened / 4.0)
        * ((co2_stomatal_ppm - gamma_compensation_ppm).max(0.0)
        / (co2_stomatal_ppm + 2.0 * gamma_compensation_ppm));
    let r_unscreened = p_unscreened * (gamma_compensation_ppm / co2_stomatal_ppm);
    let unscreened_f_air_to_buffer_mg_m2_s =
        M_CH2O_MG_PER_UMOL * h_buffer * (p_unscreened - r_unscreened);

    let screen_photosynthesis_loss_mg_m2_s =
        unscreened_f_air_to_buffer_mg_m2_s - f_air_to_buffer_mg_m2_s;

    PreparedPhotosynthesisRecord {
        period_label: spec.period_label,
        timestamp_iso_utc: spec.timestamp_iso_utc,
        timestamp_utc_s: spec.timestamp_utc_s,
        time_s: spec.time_s,
        leaf_area_index: spec.leaf_area_index,
        leaf_temperature_c: spec.leaf_temperature_c,
        leaf_temperature_k,
        absorbed_par_umol_m2_s: spec.absorbed_par_umol_m2_s,
        co2_air_ppm: spec.co2_air_ppm,
        co2_stomatal_ppm,
        carbohydrate_buffer_mg_m2: spec.carbohydrate_buffer_mg_m2,
        j_25_canopy_umol_m2_s,
        j_pot_umol_m2_s,
        electron_transport_j_umol_m2_s,
        gamma_compensation_ppm,
        gross_photosynthesis_p_umol_m2_s,
        photorespiration_r_umol_m2_s,
        net_assimilation_p_minus_r_umol_m2_s,
        h_buffer,
        f_air_to_buffer_mg_m2_s,
        unscreened_f_air_to_buffer_mg_m2_s,
        screen_photosynthesis_loss_mg_m2_s,
    }
}

fn reference_records() -> Vec<PreparedPhotosynthesisRecord> {
    vec![
        // 1. Winter night (11 Jan 2015 02:00 UTC): PAR = 0 -> J = 0, P = 0, R = 0, F_{air->buffer} = 0
        evaluate_record(RecordSpec {
            period_label: "winter_night",
            timestamp_iso_utc: "2015-01-11T02:00:00Z",
            timestamp_utc_s: 1_420_941_600,
            time_s: 3_376_800.0,
            leaf_area_index: 1.1,
            leaf_temperature_c: 16.8,
            absorbed_par_umol_m2_s: 0.0,
            unscreened_absorbed_par_umol_m2_s: 0.0,
            co2_air_ppm: 440.0,
            carbohydrate_buffer_mg_m2: 4_200.0,
        }),
        // 2. Winter noon (11 Jan 2015 12:00 UTC): PAR = 234.51 umol/(m²·s), CO2 enriched to 680 ppm
        evaluate_record(RecordSpec {
            period_label: "winter_noon",
            timestamp_iso_utc: "2015-01-11T12:00:00Z",
            timestamp_utc_s: 1_420_977_600,
            time_s: 3_412_800.0,
            leaf_area_index: 1.1,
            leaf_temperature_c: 21.1,
            absorbed_par_umol_m2_s: 234.508_885_480_829,
            unscreened_absorbed_par_umol_m2_s: 234.508_885_480_829,
            co2_air_ppm: 680.0,
            carbohydrate_buffer_mg_m2: 7_800.0,
        }),
        // 3. Spring equinox noon (20 Mar 2015 12:00 UTC): PAR = 860.20 umol/(m²·s), LAI = 3.2, CO2 = 720 ppm
        evaluate_record(RecordSpec {
            period_label: "equinox_noon",
            timestamp_iso_utc: "2015-03-20T12:00:00Z",
            timestamp_utc_s: 1_426_852_800,
            time_s: 9_288_000.0,
            leaf_area_index: 3.2,
            leaf_temperature_c: 24.5,
            absorbed_par_umol_m2_s: 860.201_744_301_822,
            unscreened_absorbed_par_umol_m2_s: 860.201_744_301_822,
            co2_air_ppm: 720.0,
            carbohydrate_buffer_mg_m2: 12_500.0,
        }),
        // 4. Summer afternoon (16 Jul 2015 13:00 UTC): shading screen reduces absorbed PAR from 1338.85 to 901.35 umol/(m²·s)
        evaluate_record(RecordSpec {
            period_label: "summer_afternoon",
            timestamp_iso_utc: "2015-07-16T13:00:00Z",
            timestamp_utc_s: 1_437_051_600,
            time_s: 19_486_800.0,
            leaf_area_index: 4.6,
            leaf_temperature_c: 30.4,
            absorbed_par_umol_m2_s: 901.354_930_262_064,
            unscreened_absorbed_par_umol_m2_s: 1_338.852_023_902_900,
            co2_air_ppm: 410.0,
            carbohydrate_buffer_mg_m2: 15_200.0,
        }),
    ]
}

pub fn prepare_photosynthesis_service(
    payload: PreparePhotosynthesisInput,
) -> Result<PreparePhotosynthesisOutput, AppError> {
    let preset = payload
        .preset_id
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .unwrap_or(DEFAULT_PHOTOSYNTHESIS_PRESET_ID);

    if preset != DEFAULT_PHOTOSYNTHESIS_PRESET_ID {
        return Err(AppError::BadRequest(format!(
            "unknown simulation photosynthesis preset '{preset}'; \
             supported preset is '{DEFAULT_PHOTOSYNTHESIS_PRESET_ID}'"
        )));
    }

    let filter = payload
        .period_filter
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .unwrap_or("all");

    let all_records = reference_records();

    let records: Vec<PreparedPhotosynthesisRecord> = if filter == "all" {
        all_records
    } else {
        let filtered: Vec<PreparedPhotosynthesisRecord> = all_records
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

    Ok(PreparePhotosynthesisOutput {
        preset_id: DEFAULT_PHOTOSYNTHESIS_PRESET_ID,
        execution_mode: "reference_dataset_direct",
        source_reference: "Sourisseau et al. (2026) PLOS One 21(2): e0340619, \
                           Section 3.6, Section 3.7.1, Vanthoor et al. (2011) & TiGwerLib 0.7.0 TYM",
        parameters: PhotosynthesisParametersOutput {
            cultivar: "Clodano F1 Hybrid (Syngenta) — photosynthesis parameters retained from Vanthoor (2011)",
            eta_air_to_stom: ETA_AIR_TO_STOM,
            j_25_leaf_umol_m2_s: J_25_LEAF_UMOL_M2_S,
            t_25_k: T_25_K,
            e_j_j_mol: E_J_J_MOL,
            h_deactivation_j_mol: H_DEACTIVATION_J_MOL,
            s_entropy_j_mol_k: S_ENTROPY_J_MOL_K,
            r_gas_j_mol_k: R_GAS_J_MOL_K,
            alpha_electron_yield: ALPHA_ELECTRON_YIELD,
            theta_curvature: THETA_CURVATURE,
            c_gamma_umol_mol_k: C_GAMMA_UMOL_MOL_K,
            m_ch2o_mg_per_umol: M_CH2O_MG_PER_UMOL,
            c_buffer_max_mg_m2: C_BUFFER_MAX_MG_M2,
        },
        records,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn electron_transport_assimilation_and_buffer_saturation_hold_across_all_records() {
        let output =
            prepare_photosynthesis_service(PreparePhotosynthesisInput::default())
                .expect("default photosynthesis preset should succeed");

        assert_eq!(output.records.len(), 4);

        for record in &output.records {
            // CO2_stom = eta_{air->stom} * CO2_air
            assert!(
                (record.co2_stomatal_ppm - ETA_AIR_TO_STOM * record.co2_air_ppm)
                    .abs()
                    < 1e-12
            );

            // J_{25,can} = LAI * J_{25,leaf}
            assert!(
                (record.j_25_canopy_umol_m2_s
                    - record.leaf_area_index * J_25_LEAF_UMOL_M2_S)
                    .abs()
                    < 1e-12
            );

            // Quadratic root check: theta * J^2 - (J_pot + alpha*PAR)*J + J_pot*alpha*PAR = 0
            let j = record.electron_transport_j_umol_m2_s;
            let j_pot = record.j_pot_umol_m2_s;
            let a_par = ALPHA_ELECTRON_YIELD * record.absorbed_par_umol_m2_s;
            let quadratic_residual =
                THETA_CURVATURE * j.powi(2) - (j_pot + a_par) * j + j_pot * a_par;
            assert!(quadratic_residual.abs() < 1e-9);

            // P, R, h_buffer, and F_{air->buffer}
            let expected_p = (j / 4.0)
                * ((record.co2_stomatal_ppm - record.gamma_compensation_ppm)
                .max(0.0)
                / (record.co2_stomatal_ppm
                + 2.0 * record.gamma_compensation_ppm));
            let expected_r = expected_p
                * (record.gamma_compensation_ppm / record.co2_stomatal_ppm);
            assert!(
                (record.gross_photosynthesis_p_umol_m2_s - expected_p).abs()
                    < 1e-12
            );
            assert!(
                (record.photorespiration_r_umol_m2_s - expected_r).abs() < 1e-12
            );

            let expected_h_buf = 1.0
                / (1.0
                + (0.005
                * (record.carbohydrate_buffer_mg_m2
                - C_BUFFER_MAX_MG_M2))
                .exp());
            assert!((record.h_buffer - expected_h_buf).abs() < 1e-12);

            let expected_f = M_CH2O_MG_PER_UMOL
                * expected_h_buf
                * (expected_p - expected_r);
            assert!(
                (record.f_air_to_buffer_mg_m2_s - expected_f).abs() < 1e-12
            );
        }

        // Nighttime has zero photosynthesis; summer afternoon shows shading-screen photosynthesis trade-off.
        assert_eq!(output.records[0].f_air_to_buffer_mg_m2_s, 0.0);
        assert!(output.records[3].screen_photosynthesis_loss_mg_m2_s > 0.05);
    }
}