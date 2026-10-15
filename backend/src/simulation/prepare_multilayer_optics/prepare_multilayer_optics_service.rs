use serde::{Deserialize, Serialize};

use crate::error::AppError;

pub const DEFAULT_MULTILAYER_OPTICS_PRESET_ID: &str = "ctifl_nantes_2014_2015";

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PrepareMultilayerOpticsInput {
    /// Optional preset identifier. Defaults to `"ctifl_nantes_2014_2015"`.
    pub preset_id: Option<String>,

    /// Optional filter matching Models 3–5 period labels:
    /// `"winter_night"`, `"winter_noon"`, `"equinox_noon"`, `"summer_afternoon"`, or `"all"`.
    pub period_filter: Option<String>,
}

#[derive(Debug, Clone, Copy, Serialize)]
pub struct FiveLayerValuesOutput {
    pub roof: f64,
    pub shading_screen: f64,
    pub thermal_screen: f64,
    pub canopy: f64,
    pub floor: f64,
}

#[derive(Debug, Clone, Copy, Serialize)]
pub struct WavebandStackSolutionOutput {
    pub incident_horizontal_w_m2: f64,
    pub stack_tau_downward: f64,
    pub stack_rho_front: f64,
    pub absorbed_fraction: FiveLayerValuesOutput,
    pub absorbed_irradiance_floor_w_m2: FiveLayerValuesOutput,
    pub downward_incident_canopy_floor_w_m2: f64,
    pub upward_incident_canopy_floor_w_m2: f64,
    pub downward_incident_floor_w_m2: f64,
    pub conservation_residual: f64,
}

#[derive(Debug, Clone, Copy, Serialize)]
pub struct PublishedMultilayerVariantComparisonOutput {
    pub variant_id: &'static str,
    pub description: &'static str,
    pub max_temperature_rmse_shift_k: Option<f64>,
    pub march_lai_shift_m2_m2: Option<f64>,
    pub season_yield_shift_kg_m2: Option<f64>,
    pub season_fruit_count_shift_m2: Option<i32>,
    pub physical_interpretation: &'static str,
}

#[derive(Debug, Clone, Serialize)]
pub struct PreparedMultilayerOpticsRecord {
    pub period_label: &'static str,
    pub timestamp_iso_utc: &'static str,
    pub timestamp_utc_s: i64,
    pub time_s: f64,

    pub shading_screen_deployment: f64,
    pub thermal_screen_deployment: f64,
    pub leaf_area_index: f64,
    pub canopy_tau_par: f64,
    pub canopy_tau_nir: f64,

    pub par: WavebandStackSolutionOutput,
    pub nir: WavebandStackSolutionOutput,

    /// Combined (PAR + NIR) shortwave heat absorbed per layer,
    /// W/m² of total greenhouse floor area.
    pub total_shortwave_absorbed_floor_w_m2: FiveLayerValuesOutput,

    /// Absorbed canopy PAR per m² of total greenhouse floor area, W/m².
    pub absorbed_canopy_par_floor_w_m2: f64,

    /// Absorbed canopy PAR per m² of cultivated area, W/m².
    pub absorbed_canopy_par_cultivated_w_m2: f64,

    /// Downward PAR photon flux incident at the top of the canopy,
    /// umol/(m² floor · s).
    pub incident_canopy_ppfd_umol_m2_floor_s: f64,

    /// PAR photon flux absorbed by the canopy per m² of total greenhouse floor,
    /// umol/(m² floor · s).
    pub absorbed_canopy_ppfd_umol_m2_floor_s: f64,

    /// PAR photon flux absorbed by the canopy per m² of cultivated area,
    /// umol/(m² cultivated · s).
    pub absorbed_canopy_ppfd_umol_m2_cultivated_s: f64,

    /// Counterfactual absorbed canopy PPFD if both screens were retracted (u = 0),
    /// umol/(m² floor · s).
    pub unscreened_absorbed_canopy_ppfd_umol_m2_floor_s: f64,

    /// Instantaneous canopy absorbed PPFD reduction due to active screen deployment,
    /// umol/(m² floor · s).
    pub screen_loss_absorbed_canopy_ppfd_umol_m2_floor_s: f64,
}

#[derive(Debug, Clone, Serialize)]
pub struct PrepareMultilayerOpticsOutput {
    pub preset_id: &'static str,
    pub execution_mode: &'static str,
    pub source_reference: &'static str,
    pub r_cul: f64,
    pub par_fraction_of_solar: f64,
    pub nir_fraction_of_solar: f64,
    pub par_joule_to_umol: f64,
    pub roof_upward_diffuse_transmittance: f64,
    pub shading_screen_daylight_deployment_share_summer_pct: f64,
    pub published_variant_comparisons: [PublishedMultilayerVariantComparisonOutput; 2],
    pub records: Vec<PreparedMultilayerOpticsRecord>,
}

fn reference_records() -> Vec<PreparedMultilayerOpticsRecord> {
    vec![
        // 1. Winter night (11 Jan 2015 02:00 UTC) — thermal screen deployed (u_thermal = 1.0), LAI = 1.1
        PreparedMultilayerOpticsRecord {
            period_label: "winter_night",
            timestamp_iso_utc: "2015-01-11T02:00:00Z",
            timestamp_utc_s: 1_420_941_600,
            time_s: 3_376_800.0,
            shading_screen_deployment: 0.0,
            thermal_screen_deployment: 1.0,
            leaf_area_index: 1.1,
            canopy_tau_par: 0.511_341_892_163,
            canopy_tau_nir: 0.766_180_801_631,
            par: WavebandStackSolutionOutput {
                incident_horizontal_w_m2: 0.0,
                stack_tau_downward: 0.0,
                stack_rho_front: 0.08,
                absorbed_fraction: FiveLayerValuesOutput {
                    roof: 0.92,
                    shading_screen: 0.0,
                    thermal_screen: 0.0,
                    canopy: 0.0,
                    floor: 0.0,
                },
                absorbed_irradiance_floor_w_m2: FiveLayerValuesOutput {
                    roof: 0.0,
                    shading_screen: 0.0,
                    thermal_screen: 0.0,
                    canopy: 0.0,
                    floor: 0.0,
                },
                downward_incident_canopy_floor_w_m2: 0.0,
                upward_incident_canopy_floor_w_m2: 0.0,
                downward_incident_floor_w_m2: 0.0,
                conservation_residual: 0.0,
            },
            nir: WavebandStackSolutionOutput {
                incident_horizontal_w_m2: 0.0,
                stack_tau_downward: 0.0,
                stack_rho_front: 0.08,
                absorbed_fraction: FiveLayerValuesOutput {
                    roof: 0.92,
                    shading_screen: 0.0,
                    thermal_screen: 0.0,
                    canopy: 0.0,
                    floor: 0.0,
                },
                absorbed_irradiance_floor_w_m2: FiveLayerValuesOutput {
                    roof: 0.0,
                    shading_screen: 0.0,
                    thermal_screen: 0.0,
                    canopy: 0.0,
                    floor: 0.0,
                },
                downward_incident_canopy_floor_w_m2: 0.0,
                upward_incident_canopy_floor_w_m2: 0.0,
                downward_incident_floor_w_m2: 0.0,
                conservation_residual: 0.0,
            },
            total_shortwave_absorbed_floor_w_m2: FiveLayerValuesOutput {
                roof: 0.0,
                shading_screen: 0.0,
                thermal_screen: 0.0,
                canopy: 0.0,
                floor: 0.0,
            },
            absorbed_canopy_par_floor_w_m2: 0.0,
            absorbed_canopy_par_cultivated_w_m2: 0.0,
            incident_canopy_ppfd_umol_m2_floor_s: 0.0,
            absorbed_canopy_ppfd_umol_m2_floor_s: 0.0,
            absorbed_canopy_ppfd_umol_m2_cultivated_s: 0.0,
            unscreened_absorbed_canopy_ppfd_umol_m2_floor_s: 0.0,
            screen_loss_absorbed_canopy_ppfd_umol_m2_floor_s: 0.0,
        },
        // 2. Winter noon (11 Jan 2015 12:00 UTC) — screens retracted for winter light, LAI = 1.1
        PreparedMultilayerOpticsRecord {
            period_label: "winter_noon",
            timestamp_iso_utc: "2015-01-11T12:00:00Z",
            timestamp_utc_s: 1_420_977_600,
            time_s: 3_412_800.0,
            shading_screen_deployment: 0.0,
            thermal_screen_deployment: 0.0,
            leaf_area_index: 1.1,
            canopy_tau_par: 0.511_341_892_163,
            canopy_tau_nir: 0.766_180_801_631,
            par: WavebandStackSolutionOutput {
                incident_horizontal_w_m2: 120.0,
                stack_tau_downward: 0.0,
                stack_rho_front: 0.189_671_845_198,
                absorbed_fraction: FiveLayerValuesOutput {
                    roof: 0.254_165_022_486,
                    shading_screen: 0.0,
                    thermal_screen: 0.0,
                    canopy: 0.427_623_788_258,
                    floor: 0.128_539_344_058,
                },
                absorbed_irradiance_floor_w_m2: FiveLayerValuesOutput {
                    roof: 30.499_802_698_307,
                    shading_screen: 0.0,
                    thermal_screen: 0.0,
                    canopy: 51.314_854_590_991,
                    floor: 15.424_721_286_985,
                },
                downward_incident_canopy_floor_w_m2: 84.269_974_636_524,
                upward_incident_canopy_floor_w_m2: 28.645_910_961_543,
                downward_incident_floor_w_m2: 44.070_632_248_527,
                conservation_residual: 0.0,
            },
            nir: WavebandStackSolutionOutput {
                incident_horizontal_w_m2: 120.0,
                stack_tau_downward: 0.0,
                stack_rho_front: 0.287_522_586_901,
                absorbed_fraction: FiveLayerValuesOutput {
                    roof: 0.276_227_578_825,
                    shading_screen: 0.0,
                    thermal_screen: 0.0,
                    canopy: 0.151_585_397_879,
                    floor: 0.284_664_436_395,
                },
                absorbed_irradiance_floor_w_m2: FiveLayerValuesOutput {
                    roof: 33.147_309_459_014,
                    shading_screen: 0.0,
                    thermal_screen: 0.0,
                    canopy: 18.190_247_745_469,
                    floor: 34.159_732_367_448,
                },
                downward_incident_canopy_floor_w_m2: 85.521_243_833_485,
                upward_incident_canopy_floor_w_m2: 34.159_732_367_448,
                downward_incident_floor_w_m2: 68.319_464_734_897,
                conservation_residual: 0.0,
            },
            total_shortwave_absorbed_floor_w_m2: FiveLayerValuesOutput {
                roof: 63.647_112_157_321,
                shading_screen: 0.0,
                thermal_screen: 0.0,
                canopy: 69.505_102_336_460,
                floor: 49.584_453_654_433,
            },
            absorbed_canopy_par_floor_w_m2: 51.314_854_590_991,
            absorbed_canopy_par_cultivated_w_m2: 56.389_950_099_990,
            incident_canopy_ppfd_umol_m2_floor_s: 385.113_784_088_915,
            absorbed_canopy_ppfd_umol_m2_floor_s: 234.508_885_480_829,
            absorbed_canopy_ppfd_umol_m2_cultivated_s: 257.702_071_956_955,
            unscreened_absorbed_canopy_ppfd_umol_m2_floor_s: 234.508_885_480_829,
            screen_loss_absorbed_canopy_ppfd_umol_m2_floor_s: 0.0,
        },
        // 3. Spring equinox noon (20 Mar 2015 12:00 UTC) — screens retracted, LAI = 3.2
        PreparedMultilayerOpticsRecord {
            period_label: "equinox_noon",
            timestamp_iso_utc: "2015-03-20T12:00:00Z",
            timestamp_utc_s: 1_426_852_800,
            time_s: 9_288_000.0,
            shading_screen_deployment: 0.0,
            thermal_screen_deployment: 0.0,
            leaf_area_index: 3.2,
            canopy_tau_par: 0.186_877_238_985,
            canopy_tau_nir: 0.473_547_244_848,
            par: WavebandStackSolutionOutput {
                incident_horizontal_w_m2: 290.0,
                stack_tau_downward: 0.0,
                stack_rho_front: 0.126_056_000_460,
                absorbed_fraction: FiveLayerValuesOutput {
                    roof: 0.173_115_126_904,
                    shading_screen: 0.0,
                    thermal_screen: 0.0,
                    canopy: 0.649_061_906_211,
                    floor: 0.051_766_966_424,
                },
                absorbed_irradiance_floor_w_m2: FiveLayerValuesOutput {
                    roof: 50.203_386_802_265,
                    shading_screen: 0.0,
                    thermal_screen: 0.0,
                    canopy: 188.227_952_801_274,
                    floor: 15.012_420_263_070,
                },
                downward_incident_canopy_floor_w_m2: 221.031_342_648_144,
                upward_incident_canopy_floor_w_m2: 27.880_209_059_986,
                downward_incident_floor_w_m2: 42.892_629_323_056,
                conservation_residual: 0.0,
            },
            nir: WavebandStackSolutionOutput {
                incident_horizontal_w_m2: 290.0,
                stack_tau_downward: 0.0,
                stack_rho_front: 0.259_379_239_640,
                absorbed_fraction: FiveLayerValuesOutput {
                    roof: 0.203_175_721_485,
                    shading_screen: 0.0,
                    thermal_screen: 0.0,
                    canopy: 0.334_965_926_105,
                    floor: 0.202_479_112_771,
                },
                absorbed_irradiance_floor_w_m2: FiveLayerValuesOutput {
                    roof: 58.920_959_230_611,
                    shading_screen: 0.0,
                    thermal_screen: 0.0,
                    canopy: 97.140_118_570_337,
                    floor: 58.718_942_703_514,
                },
                downward_incident_canopy_floor_w_m2: 225.151_456_716_882,
                upward_incident_canopy_floor_w_m2: 58.718_942_703_514,
                downward_incident_floor_w_m2: 117.437_885_407_028,
                conservation_residual: 0.0,
            },
            total_shortwave_absorbed_floor_w_m2: FiveLayerValuesOutput {
                roof: 109.124_346_032_876,
                shading_screen: 0.0,
                thermal_screen: 0.0,
                canopy: 285.368_071_371_611,
                floor: 73.731_362_966_584,
            },
            absorbed_canopy_par_floor_w_m2: 188.227_952_801_274,
            absorbed_canopy_par_cultivated_w_m2: 206.843_904_177_224,
            incident_canopy_ppfd_umol_m2_floor_s: 1_010.113_235_902_018,
            absorbed_canopy_ppfd_umol_m2_floor_s: 860.201_744_301_822,
            absorbed_canopy_ppfd_umol_m2_cultivated_s: 945.276_642_089_914,
            unscreened_absorbed_canopy_ppfd_umol_m2_floor_s: 860.201_744_301_822,
            screen_loss_absorbed_canopy_ppfd_umol_m2_floor_s: 0.0,
        },
        // 4. Summer afternoon (16 Jul 2015 13:00 UTC) — shading screen unfolded (Section 4.2, Fig. 9), LAI = 4.6
        PreparedMultilayerOpticsRecord {
            period_label: "summer_afternoon",
            timestamp_iso_utc: "2015-07-16T13:00:00Z",
            timestamp_utc_s: 1_437_051_600,
            time_s: 19_486_800.0,
            shading_screen_deployment: 1.0,
            thermal_screen_deployment: 0.0,
            leaf_area_index: 4.6,
            canopy_tau_par: 0.126_359_103_017,
            canopy_tau_nir: 0.352_897_295_362,
            par: WavebandStackSolutionOutput {
                incident_horizontal_w_m2: 410.0,
                stack_tau_downward: 0.0,
                stack_rho_front: 0.253_808_101_954,
                absorbed_fraction: FiveLayerValuesOutput {
                    roof: 0.153_688_029_643,
                    shading_screen: 0.086_327_266_922,
                    thermal_screen: 0.0,
                    canopy: 0.481_056_161_745,
                    floor: 0.025_120_439_736,
                },
                absorbed_irradiance_floor_w_m2: FiveLayerValuesOutput {
                    roof: 63.012_092_153_634,
                    shading_screen: 35.394_179_437_970,
                    thermal_screen: 0.0,
                    canopy: 197.233_026_315_550,
                    floor: 10.299_380_291_583,
                },
                downward_incident_canopy_floor_w_m2: 223.625_092_135_468,
                upward_incident_canopy_floor_w_m2: 19.127_420_541_512,
                downward_incident_floor_w_m2: 29.426_800_833_095,
                conservation_residual: 0.0,
            },
            nir: WavebandStackSolutionOutput {
                incident_horizontal_w_m2: 410.0,
                stack_tau_downward: 0.0,
                stack_rho_front: 0.341_030_219_612,
                absorbed_fraction: FiveLayerValuesOutput {
                    roof: 0.173_354_133_868,
                    shading_screen: 0.099_616_041_932,
                    thermal_screen: 0.0,
                    canopy: 0.276_813_845_720,
                    floor: 0.109_185_758_868,
                },
                absorbed_irradiance_floor_w_m2: FiveLayerValuesOutput {
                    roof: 71.075_194_886_026,
                    shading_screen: 40.842_577_192_075,
                    thermal_screen: 0.0,
                    canopy: 113.493_676_745_079,
                    floor: 44.766_161_135_761,
                },
                downward_incident_canopy_floor_w_m2: 225.025_703_474_385,
                upward_incident_canopy_floor_w_m2: 44.766_161_135_761,
                downward_incident_floor_w_m2: 89.532_322_271_522,
                conservation_residual: 0.0,
            },
            total_shortwave_absorbed_floor_w_m2: FiveLayerValuesOutput {
                roof: 134.087_287_039_660,
                shading_screen: 76.236_756_630_045,
                thermal_screen: 0.0,
                canopy: 310.726_703_060_629,
                floor: 55.065_541_427_344,
            },
            absorbed_canopy_par_floor_w_m2: 197.233_026_315_550,
            absorbed_canopy_par_cultivated_w_m2: 216.739_589_357_747,
            incident_canopy_ppfd_umol_m2_floor_s: 1_021.966_671_059_089,
            absorbed_canopy_ppfd_umol_m2_floor_s: 901.354_930_262_064,
            absorbed_canopy_ppfd_umol_m2_cultivated_s: 990.499_923_364_905,
            unscreened_absorbed_canopy_ppfd_umol_m2_floor_s: 1_338.852_023_902_900,
            screen_loss_absorbed_canopy_ppfd_umol_m2_floor_s: 437.497_093_640_836,
        },
    ]
}

pub fn prepare_multilayer_optics_service(
    payload: PrepareMultilayerOpticsInput,
) -> Result<PrepareMultilayerOpticsOutput, AppError> {
    let preset = payload
        .preset_id
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .unwrap_or(DEFAULT_MULTILAYER_OPTICS_PRESET_ID);

    if preset != DEFAULT_MULTILAYER_OPTICS_PRESET_ID {
        return Err(AppError::BadRequest(format!(
            "unknown simulation multilayer optics preset '{preset}'; \
             supported preset is '{DEFAULT_MULTILAYER_OPTICS_PRESET_ID}'"
        )));
    }

    let filter = payload
        .period_filter
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .unwrap_or("all");

    let all_records = reference_records();

    let records: Vec<PreparedMultilayerOpticsRecord> = if filter == "all" {
        all_records
    } else {
        let filtered: Vec<PreparedMultilayerOpticsRecord> = all_records
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

    Ok(PrepareMultilayerOpticsOutput {
        preset_id: DEFAULT_MULTILAYER_OPTICS_PRESET_ID,
        execution_mode: "reference_dataset_direct",
        source_reference: "Sourisseau et al. (2026) PLOS One 21(2): e0340619, \
                           Section 3.2.3, Section 4.2, Section 4.5 (Fig. 14) & TiGwerLib 0.7.0 ML_PAR/ML_NIR",
        r_cul: 0.91,
        par_fraction_of_solar: 0.50,
        nir_fraction_of_solar: 0.50,
        par_joule_to_umol: 4.57,
        roof_upward_diffuse_transmittance: 0.750_731_435_433,
        shading_screen_daylight_deployment_share_summer_pct: 15.5,
        published_variant_comparisons: [
            PublishedMultilayerVariantComparisonOutput {
                variant_id: "C.1 (no south wall shortwave)",
                description:
                "South gable boundary removed (missing 17% south-wall cover solar contribution)",
                max_temperature_rmse_shift_k: Some(0.42),
                march_lai_shift_m2_m2: Some(-1.2),
                season_yield_shift_kg_m2: Some(-3.8),
                season_fruit_count_shift_m2: Some(-35),
                physical_interpretation:
                "Reduced crop light interception slows spring LAI expansion enough to miss the first pruning event and cuts cumulative harvest by 3.8 kg/m² (Section 4.5, Fig. 14)",
            },
            PublishedMultilayerVariantComparisonOutput {
                variant_id: "C.2 (quasi-infinite greenhouse: constant tau=0.78, r_cul=1.0, no walls)",
                description:
                "Constant roof transmittance (0.78), alleyways ignored (r_cul=1.0 instead of 0.91), and all walls removed",
                max_temperature_rmse_shift_k: None,
                march_lai_shift_m2_m2: None,
                season_yield_shift_kg_m2: None,
                season_fruit_count_shift_m2: None,
                physical_interpretation:
                "Overestimating cultivated area (r_cul=1.0 vs 0.91) artificially compensates for missing south-wall light, showing why explicit r_cul and directional optics are required (Section 4.5)",
            },
        ],
        records,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_reference_multilayer_records_conserving_energy() {
        let output = prepare_multilayer_optics_service(
            PrepareMultilayerOpticsInput::default(),
        )
            .expect("default multilayer optics preset should succeed");

        assert_eq!(output.preset_id, DEFAULT_MULTILAYER_OPTICS_PRESET_ID);
        assert_eq!(output.records.len(), 4);

        for record in &output.records {
            let par_sum = record.par.stack_tau_downward
                + record.par.stack_rho_front
                + record.par.absorbed_fraction.roof
                + record.par.absorbed_fraction.shading_screen
                + record.par.absorbed_fraction.thermal_screen
                + record.par.absorbed_fraction.canopy
                + record.par.absorbed_fraction.floor;
            assert!((par_sum - 1.0).abs() < 1e-11);

            let nir_sum = record.nir.stack_tau_downward
                + record.nir.stack_rho_front
                + record.nir.absorbed_fraction.roof
                + record.nir.absorbed_fraction.shading_screen
                + record.nir.absorbed_fraction.thermal_screen
                + record.nir.absorbed_fraction.canopy
                + record.nir.absorbed_fraction.floor;
            assert!((nir_sum - 1.0).abs() < 1e-11);

            assert!(
                (record.absorbed_canopy_par_cultivated_w_m2 * output.r_cul
                    - record.absorbed_canopy_par_floor_w_m2)
                    .abs()
                    < 1e-9
            );
        }
    }

    #[test]
    fn summer_afternoon_shows_shading_screen_light_reduction() {
        let output = prepare_multilayer_optics_service(
            PrepareMultilayerOpticsInput {
                preset_id: None,
                period_filter: Some("summer_afternoon".into()),
            },
        )
            .expect("summer_afternoon filter should succeed");

        let record = &output.records[0];
        assert_eq!(record.shading_screen_deployment, 1.0);
        assert!(record.screen_loss_absorbed_canopy_ppfd_umol_m2_floor_s > 400.0);
        assert!(record.total_shortwave_absorbed_floor_w_m2.shading_screen > 70.0);
    }
}