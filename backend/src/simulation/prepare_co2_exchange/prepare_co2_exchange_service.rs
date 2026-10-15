use serde::{Deserialize, Serialize};

use crate::error::AppError;

pub const DEFAULT_CO2_EXCHANGE_PRESET_ID: &str = "ctifl_nantes_2014_2015";

pub const CULTIVATED_AREA_M2: f64 = 943.488;
pub const M_CO2_G_MOL: f64 = 44.009_5;
pub const M_CH2O_G_MOL: f64 = 30.026_0;
pub const MG_TO_KG: f64 = 1e-6;

/// Practical finite-injector capacity for planning comparison (18 kg/h = 0.005 kg/s).
pub const FINITE_INJECTOR_MAX_DELIVERY_KG_S: f64 = 0.005_0;

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PrepareCo2ExchangeInput {
    /// Optional preset identifier. Defaults to `"ctifl_nantes_2014_2015"`.
    pub preset_id: Option<String>,

    /// Optional filter matching Models 3–22 period labels:
    /// `"winter_night"`, `"winter_noon"`, `"equinox_noon"`, `"summer_afternoon"`, or `"all"`.
    pub period_filter: Option<String>,
}

#[derive(Debug, Clone, Copy, Serialize)]
pub struct Co2ExchangeParametersOutput {
    pub cultivated_area_a_crop_m2: f64,
    pub m_co2_g_mol: f64,
    pub m_ch2o_g_mol: f64,
    pub stoichiometric_co2_per_ch2o: f64,
    pub mg_to_kg_factor: f64,
    pub finite_injector_max_delivery_kg_s: f64,
    pub reference_forcing_mode: &'static str,
    pub planning_mode_recommendation: &'static str,
}

#[derive(Debug, Clone, Copy, Serialize)]
pub struct PublishedCo2ValidationSummaryOutput {
    pub comparison_day_after_start: u16,
    pub comparison_date_iso: &'static str,
    pub cumulative_injection_underestimate_pct: f64,
    pub sensor_nominal_accuracy_ppm: f64,
    pub sensitivity_offset_tested_ppm: f64,
    pub underestimate_with_minus_20ppm_offset_pct: f64,
    pub supply_sources_in_experiment: &'static str,
}

#[derive(Debug, Clone, Copy, Serialize)]
pub struct Co2ModeBalanceEvaluationOutput {
    pub mode_name: &'static str,
    pub injection_rate_kg_s: f64,
    pub injector_saturated: bool,

    /// dM_CO2/dt = m_dot_injection + m_dot_air,in - m_dot_air,out - m_dot_crop, kg/s.
    pub dm_co2_dt_kg_s: f64,
}

#[derive(Debug, Clone, Serialize)]
pub struct PreparedCo2ExchangeRecord {
    pub period_label: &'static str,
    pub timestamp_iso_utc: &'static str,
    pub timestamp_utc_s: i64,
    pub time_s: f64,

    /// Section 27: `climate.co2_ppm` (MainAir.CO2_ppm).
    pub main_air_co2_ppm: f64,

    /// Carbohydrate flows from Models 19 and 20, mg CH2O/(m²_cultivated·s).
    pub f_photosynthesis_mg_m2_s: f64,
    pub r_growth_mg_m2_s: f64,
    pub r_maintenance_mg_m2_s: f64,

    /// F_net,CH2O = F_photosynthesis - R_growth - R_maintenance, mg CH2O/(m²_cultivated·s).
    /// Positive during daytime assimilation; negative during nighttime respiration.
    pub f_net_ch2o_mg_m2_s: f64,

    /// Whole-crop CO2 uptake rate m_dot_CO2,crop = 10^-6 * A_crop * (M_CO2/M_CH2O) * F_net,CH2O, kg/s.
    /// Positive means net CO2 removed from indoor air by the crop; negative means net respiratory release.
    pub m_dot_co2_crop_kg_s: f64,

    /// External airflow CO2 inflows and outflows (vents + pressure cracks from Model 11), kg/s.
    pub m_dot_air_in_kg_s: f64,
    pub m_dot_air_out_kg_s: f64,
    pub net_ventilation_and_leakage_loss_kg_s: f64,

    /// Reference prescribed-concentration injection rate, kg/s.
    pub prescribed_injection_rate_kg_s: f64,

    /// Section 27: `co2.injected_kg` integrated up to this timestamp in the reference run, kg.
    pub cumulative_injected_co2_kg: f64,

    /// Comparison of the three explicit CO2 operating modes on this timestamp.
    pub mode_no_enrichment: Co2ModeBalanceEvaluationOutput,
    pub mode_prescribed_concentration: Co2ModeBalanceEvaluationOutput,
    pub mode_finite_injector: Co2ModeBalanceEvaluationOutput,
}

#[derive(Debug, Clone, Serialize)]
pub struct PrepareCo2ExchangeOutput {
    pub preset_id: &'static str,
    pub execution_mode: &'static str,
    pub source_reference: &'static str,
    pub parameters: Co2ExchangeParametersOutput,
    pub published_co2_validation: PublishedCo2ValidationSummaryOutput,
    pub records: Vec<PreparedCo2ExchangeRecord>,
}

struct RecordSpec {
    period_label: &'static str,
    timestamp_iso_utc: &'static str,
    timestamp_utc_s: i64,
    time_s: f64,
    main_air_co2_ppm: f64,
    f_photosynthesis_mg_m2_s: f64,
    r_growth_mg_m2_s: f64,
    r_maintenance_mg_m2_s: f64,
    m_dot_air_in_kg_s: f64,
    m_dot_air_out_kg_s: f64,
    prescribed_injection_rate_kg_s: f64,
    cumulative_injected_co2_kg: f64,
}

fn evaluate_mode(
    mode_name: &'static str,
    injection_rate_kg_s: f64,
    injector_saturated: bool,
    m_dot_air_in_kg_s: f64,
    m_dot_air_out_kg_s: f64,
    m_dot_co2_crop_kg_s: f64,
) -> Co2ModeBalanceEvaluationOutput {
    let dm_co2_dt_kg_s = injection_rate_kg_s + m_dot_air_in_kg_s
        - m_dot_air_out_kg_s
        - m_dot_co2_crop_kg_s;

    Co2ModeBalanceEvaluationOutput {
        mode_name,
        injection_rate_kg_s,
        injector_saturated,
        dm_co2_dt_kg_s,
    }
}

fn evaluate_record(spec: RecordSpec) -> PreparedCo2ExchangeRecord {
    let f_net_ch2o_mg_m2_s = spec.f_photosynthesis_mg_m2_s
        - spec.r_growth_mg_m2_s
        - spec.r_maintenance_mg_m2_s;

    let m_dot_co2_crop_kg_s = MG_TO_KG
        * CULTIVATED_AREA_M2
        * (M_CO2_G_MOL / M_CH2O_G_MOL)
        * f_net_ch2o_mg_m2_s;

    let net_ventilation_and_leakage_loss_kg_s =
        spec.m_dot_air_out_kg_s - spec.m_dot_air_in_kg_s;

    let mode_no_enrichment = evaluate_mode(
        "no_enrichment",
        0.0,
        false,
        spec.m_dot_air_in_kg_s,
        spec.m_dot_air_out_kg_s,
        m_dot_co2_crop_kg_s,
    );

    let mode_prescribed_concentration = evaluate_mode(
        "prescribed_concentration",
        spec.prescribed_injection_rate_kg_s,
        false,
        spec.m_dot_air_in_kg_s,
        spec.m_dot_air_out_kg_s,
        m_dot_co2_crop_kg_s,
    );

    let finite_inj = spec
        .prescribed_injection_rate_kg_s
        .min(FINITE_INJECTOR_MAX_DELIVERY_KG_S)
        .max(0.0);
    let injector_saturated =
        spec.prescribed_injection_rate_kg_s > FINITE_INJECTOR_MAX_DELIVERY_KG_S;

    let mode_finite_injector = evaluate_mode(
        "finite_injector",
        finite_inj,
        injector_saturated,
        spec.m_dot_air_in_kg_s,
        spec.m_dot_air_out_kg_s,
        m_dot_co2_crop_kg_s,
    );

    PreparedCo2ExchangeRecord {
        period_label: spec.period_label,
        timestamp_iso_utc: spec.timestamp_iso_utc,
        timestamp_utc_s: spec.timestamp_utc_s,
        time_s: spec.time_s,
        main_air_co2_ppm: spec.main_air_co2_ppm,
        f_photosynthesis_mg_m2_s: spec.f_photosynthesis_mg_m2_s,
        r_growth_mg_m2_s: spec.r_growth_mg_m2_s,
        r_maintenance_mg_m2_s: spec.r_maintenance_mg_m2_s,
        f_net_ch2o_mg_m2_s,
        m_dot_co2_crop_kg_s,
        m_dot_air_in_kg_s: spec.m_dot_air_in_kg_s,
        m_dot_air_out_kg_s: spec.m_dot_air_out_kg_s,
        net_ventilation_and_leakage_loss_kg_s,
        prescribed_injection_rate_kg_s: spec.prescribed_injection_rate_kg_s,
        cumulative_injected_co2_kg: spec.cumulative_injected_co2_kg,
        mode_no_enrichment,
        mode_prescribed_concentration,
        mode_finite_injector,
    }
}

fn reference_records() -> Vec<PreparedCo2ExchangeRecord> {
    vec![
        // 1. Winter night (11 Jan 2015 02:00 UTC): F_photosynthesis = 0, dark respiration releases CO2 (m_dot_crop < 0), injection = 0
        evaluate_record(RecordSpec {
            period_label: "winter_night",
            timestamp_iso_utc: "2015-01-11T02:00:00Z",
            timestamp_utc_s: 1_420_941_600,
            time_s: 3_376_800.0,
            main_air_co2_ppm: 440.0,
            f_photosynthesis_mg_m2_s: 0.0,
            r_growth_mg_m2_s: 0.078_085_065_9,
            r_maintenance_mg_m2_s: 0.012_618_924_4,
            m_dot_air_in_kg_s: 0.000_418_500_0,
            m_dot_air_out_kg_s: 0.000_323_190_1,
            prescribed_injection_rate_kg_s: 0.0,
            cumulative_injected_co2_kg: 1_840.0,
        }),
        // 2. Winter noon (11 Jan 2015 12:00 UTC): CO2 enriched to 680 ppm; crop assimilates CO2 and vents lose CO2
        evaluate_record(RecordSpec {
            period_label: "winter_noon",
            timestamp_iso_utc: "2015-01-11T12:00:00Z",
            timestamp_utc_s: 1_420_977_600,
            time_s: 3_412_800.0,
            main_air_co2_ppm: 680.0,
            f_photosynthesis_mg_m2_s: 0.371_413_412_1,
            r_growth_mg_m2_s: 0.091_516_016_9,
            r_maintenance_mg_m2_s: 0.012_947_552_0,
            m_dot_air_in_kg_s: 0.002_251_200_0,
            m_dot_air_out_kg_s: 0.002_743_576_9,
            prescribed_injection_rate_kg_s: 0.000_880_000_0,
            cumulative_injected_co2_kg: 1_865.0,
        }),
        // 3. Spring equinox noon (20 Mar 2015 12:00 UTC): high CO2 target (720 ppm) with open vents exceeds 18 kg/h finite injector
        evaluate_record(RecordSpec {
            period_label: "equinox_noon",
            timestamp_iso_utc: "2015-03-20T12:00:00Z",
            timestamp_utc_s: 1_426_852_800,
            time_s: 9_288_000.0,
            main_air_co2_ppm: 720.0,
            f_photosynthesis_mg_m2_s: 1.298_093_763_0,
            r_growth_mg_m2_s: 0.139_186_215_9,
            r_maintenance_mg_m2_s: 0.050_890_120_0,
            m_dot_air_in_kg_s: 0.008_620_000_0,
            m_dot_air_out_kg_s: 0.011_274_033_6,
            prescribed_injection_rate_kg_s: 0.005_850_000_0,
            cumulative_injected_co2_kg: 7_420.0,
        }),
        // 4. Summer afternoon (16 Jul 2015 13:00 UTC): wide-open vents (25.5 m³/s), indoor CO2 near ambient (410 ppm)
        evaluate_record(RecordSpec {
            period_label: "summer_afternoon",
            timestamp_iso_utc: "2015-07-16T13:00:00Z",
            timestamp_utc_s: 1_437_051_600,
            time_s: 19_486_800.0,
            main_air_co2_ppm: 410.0,
            f_photosynthesis_mg_m2_s: 1.217_484_605_4,
            r_growth_mg_m2_s: 0.146_019_550_2,
            r_maintenance_mg_m2_s: 0.094_210_300_0,
            m_dot_air_in_kg_s: 0.018_250_000_0,
            m_dot_air_out_kg_s: 0.018_027_075_5,
            prescribed_injection_rate_kg_s: 0.001_200_000_0,
            cumulative_injected_co2_kg: 18_950.0,
        }),
    ]
}

pub fn prepare_co2_exchange_service(
    payload: PrepareCo2ExchangeInput,
) -> Result<PrepareCo2ExchangeOutput, AppError> {
    let preset = payload
        .preset_id
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .unwrap_or(DEFAULT_CO2_EXCHANGE_PRESET_ID);

    if preset != DEFAULT_CO2_EXCHANGE_PRESET_ID {
        return Err(AppError::BadRequest(format!(
            "unknown simulation CO2 exchange preset '{preset}'; \
             supported preset is '{DEFAULT_CO2_EXCHANGE_PRESET_ID}'"
        )));
    }

    let filter = payload
        .period_filter
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .unwrap_or("all");

    let all_records = reference_records();

    let records: Vec<PreparedCo2ExchangeRecord> = if filter == "all" {
        all_records
    } else {
        let filtered: Vec<PreparedCo2ExchangeRecord> = all_records
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

    Ok(PrepareCo2ExchangeOutput {
        preset_id: DEFAULT_CO2_EXCHANGE_PRESET_ID,
        execution_mode: "reference_dataset_direct",
        source_reference: "Sourisseau et al. (2026) PLOS One 21(2): e0340619, \
                           Section 2.3.4, Section 3.1, Section 4.4.2 & TiGwerLib 0.7.0 CO2 balance",
        parameters: Co2ExchangeParametersOutput {
            cultivated_area_a_crop_m2: CULTIVATED_AREA_M2,
            m_co2_g_mol: M_CO2_G_MOL,
            m_ch2o_g_mol: M_CH2O_G_MOL,
            stoichiometric_co2_per_ch2o: M_CO2_G_MOL / M_CH2O_G_MOL,
            mg_to_kg_factor: MG_TO_KG,
            finite_injector_max_delivery_kg_s: FINITE_INJECTOR_MAX_DELIVERY_KG_S,
            reference_forcing_mode:
            "prescribed_concentration (infinite CO2 source tracking measured MainAir sensor, Section 3.1)",
            planning_mode_recommendation:
            "Use finite_injector or no_enrichment for Witeithie resource planning rather than assuming an infinite CO2 source (Section 23).",
        },
        published_co2_validation: PublishedCo2ValidationSummaryOutput {
            comparison_day_after_start: 313,
            comparison_date_iso: "2015-10-12",
            cumulative_injection_underestimate_pct: 17.0,
            sensor_nominal_accuracy_ppm: 50.0,
            sensitivity_offset_tested_ppm: -20.0,
            underestimate_with_minus_20ppm_offset_pct: 6.3,
            supply_sources_in_experiment:
            "Natural-gas boiler exhaust fumes (when heating active) and liquid CO2 tank (Section 2.3.4)",
        },
        records,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn net_carbohydrate_co2_stoichiometry_and_mass_balances_hold_across_all_records() {
        let output =
            prepare_co2_exchange_service(PrepareCo2ExchangeInput::default())
                .expect("default CO2 exchange preset should succeed");

        assert_eq!(output.records.len(), 4);

        for record in &output.records {
            // F_net,CH2O = F_photosynthesis - R_growth - R_maintenance
            let expected_f_net = record.f_photosynthesis_mg_m2_s
                - record.r_growth_mg_m2_s
                - record.r_maintenance_mg_m2_s;
            assert!(
                (record.f_net_ch2o_mg_m2_s - expected_f_net).abs() < 1e-12
            );

            // m_dot_CO2,crop = 10^-6 * A_crop * (M_CO2 / M_CH2O) * F_net,CH2O
            let expected_m_crop = MG_TO_KG
                * CULTIVATED_AREA_M2
                * (M_CO2_G_MOL / M_CH2O_G_MOL)
                * expected_f_net;
            assert!(
                (record.m_dot_co2_crop_kg_s - expected_m_crop).abs() < 1e-12
            );

            // dM_CO2/dt = m_dot_injection + m_dot_air,in - m_dot_air,out - m_dot_crop
            for mode in [
                &record.mode_no_enrichment,
                &record.mode_prescribed_concentration,
                &record.mode_finite_injector,
            ] {
                let expected_dm_dt = mode.injection_rate_kg_s
                    + record.m_dot_air_in_kg_s
                    - record.m_dot_air_out_kg_s
                    - expected_m_crop;
                assert!((mode.dm_co2_dt_kg_s - expected_dm_dt).abs() < 1e-12);
            }
        }

        // Winter night has negative net crop uptake (dark respiration releases CO2);
        // Equinox noon saturates the 0.005 kg/s finite injector.
        assert!(output.records[0].m_dot_co2_crop_kg_s < 0.0);
        assert!(output.records[2].mode_finite_injector.injector_saturated);
    }
}