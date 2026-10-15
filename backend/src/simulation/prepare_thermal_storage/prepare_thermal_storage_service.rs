use serde::{Deserialize, Serialize};

use crate::error::AppError;

pub const DEFAULT_THERMAL_STORAGE_PRESET_ID: &str = "ctifl_nantes_2014_2015";
pub const FLOOR_AREA_M2: f64 = 1_036.8;

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PrepareThermalStorageInput {
    /// Optional preset identifier. Defaults to `"ctifl_nantes_2014_2015"`.
    pub preset_id: Option<String>,

    /// Optional filter matching Models 3–14 period labels:
    /// `"winter_night"`, `"winter_noon"`, `"equinox_noon"`, `"summer_afternoon"`, or `"all"`.
    pub period_filter: Option<String>,
}

#[derive(Debug, Clone, Copy, Serialize)]
pub struct SoilLayerPropertySpecOutput {
    pub layer_index: u8,
    pub layer_name: &'static str,
    pub thickness_d_m: f64,
    pub thermal_conductivity_k_w_m_k: f64,
    pub density_kg_m3: f64,
    pub specific_heat_cp_j_kg_k: f64,
    pub mass_kg: f64,
    pub thermal_capacitance_c_j_k: f64,
}

#[derive(Debug, Clone, Copy, Serialize)]
pub struct ThermalStorageParametersOutput {
    pub floor_area_m2: f64,
    pub layers: [SoilLayerPropertySpecOutput; 3],
    pub conductance_g12_w_k: f64,
    pub conductance_g23_w_k: f64,
    pub conductance_g3_deep_w_k: f64,
    pub scope_distinction_note: &'static str,
}

#[derive(Debug, Clone, Copy, Serialize)]
pub struct LumpedThermalMassStateOutput {
    pub component_name: &'static str,
    pub mass_kg: f64,
    pub specific_heat_cp_j_kg_k: f64,

    /// C_th = m * c_p, J/K.
    pub thermal_capacitance_c_th_j_k: f64,

    pub temperature_c: f64,
    pub temperature_k: f64,

    /// Net heat flow into the solid component sum(Q_dot), W.
    pub net_heat_flow_in_w: f64,

    /// Rate of temperature change dT/dt = sum(Q_dot) / C_th, K/s (and K/h).
    pub dt_dt_k_per_s: f64,
    pub dt_dt_k_per_h: f64,
}

#[derive(Debug, Clone, Copy, Serialize)]
pub struct SoilLayerThermalStateOutput {
    pub layer_index: u8,
    pub layer_name: &'static str,
    pub thickness_d_m: f64,
    pub thermal_conductivity_k_w_m_k: f64,
    pub thermal_capacitance_c_j_k: f64,
    pub temperature_c: f64,
    pub temperature_k: f64,

    /// Heat flow from upper neighbour into this layer: G_{j-1,j} * (T_{j-1} - T_j)
    /// (or net surface heat flux for Layer 1), W.
    pub heat_flow_from_upper_w: f64,

    /// Heat flow from lower neighbour into this layer: G_{j,j+1} * (T_{j+1} - T_j), W.
    pub heat_flow_from_lower_w: f64,

    /// Total net heat flow into layer j: C_j * dT_j/dt, W.
    pub net_heat_flow_in_w: f64,

    /// Layer temperature derivative dT_j/dt, K/s (and K/h).
    pub dt_dt_k_per_s: f64,
    pub dt_dt_k_per_h: f64,
}

#[derive(Debug, Clone, Serialize)]
pub struct PreparedThermalStorageRecord {
    pub period_label: &'static str,
    pub timestamp_iso_utc: &'static str,
    pub timestamp_utc_s: i64,
    pub time_s: f64,

    /// Lumped thermal inertia of steel structure (posts, trusses, gutters, bars).
    pub greenhouse_structure: LumpedThermalMassStateOutput,

    /// Lumped thermal inertia of the 15 elevated growing gutters and wet rock-wool medium.
    pub growing_gutters_and_wet_medium: LumpedThermalMassStateOutput,

    /// Top surface net heat flux entering Soil Layer 1 (shortwave - convection - longwave), W.
    pub top_surface_net_heat_input_w: f64,

    /// Deep-ground boundary temperature from Model 3, °C.
    pub deep_ground_boundary_temperature_c: f64,

    /// Three-layer floor and soil conduction profile (Layer 2 satisfies exact interior Eq.).
    pub soil_layers: [SoilLayerThermalStateOutput; 3],
}

#[derive(Debug, Clone, Serialize)]
pub struct PrepareThermalStorageOutput {
    pub preset_id: &'static str,
    pub execution_mode: &'static str,
    pub source_reference: &'static str,
    pub parameters: ThermalStorageParametersOutput,
    pub records: Vec<PreparedThermalStorageRecord>,
}

pub fn adjacent_conductance_w_k(
    area_m2: f64,
    d_i_m: f64,
    k_i_w_m_k: f64,
    d_j_m: f64,
    k_j_w_m_k: f64,
) -> f64 {
    area_m2 / (d_i_m / (2.0 * k_i_w_m_k) + d_j_m / (2.0 * k_j_w_m_k))
}

fn build_lumped_mass(
    component_name: &'static str,
    mass_kg: f64,
    specific_heat_cp_j_kg_k: f64,
    temperature_c: f64,
    net_heat_flow_in_w: f64,
) -> LumpedThermalMassStateOutput {
    let thermal_capacitance_c_th_j_k = mass_kg * specific_heat_cp_j_kg_k;
    let dt_dt_k_per_s = net_heat_flow_in_w / thermal_capacitance_c_th_j_k;

    LumpedThermalMassStateOutput {
        component_name,
        mass_kg,
        specific_heat_cp_j_kg_k,
        thermal_capacitance_c_th_j_k,
        temperature_c,
        temperature_k: temperature_c + 273.15,
        net_heat_flow_in_w,
        dt_dt_k_per_s,
        dt_dt_k_per_h: dt_dt_k_per_s * 3_600.0,
    }
}

fn layer_specs() -> [SoilLayerPropertySpecOutput; 3] {
    let build_spec = |layer_index: u8,
                      layer_name: &'static str,
                      thickness_d_m: f64,
                      thermal_conductivity_k_w_m_k: f64,
                      density_kg_m3: f64,
                      specific_heat_cp_j_kg_k: f64| {
        let mass_kg = density_kg_m3 * FLOOR_AREA_M2 * thickness_d_m;
        let thermal_capacitance_c_j_k = mass_kg * specific_heat_cp_j_kg_k;
        SoilLayerPropertySpecOutput {
            layer_index,
            layer_name,
            thickness_d_m,
            thermal_conductivity_k_w_m_k,
            density_kg_m3,
            specific_heat_cp_j_kg_k,
            mass_kg,
            thermal_capacitance_c_j_k,
        }
    };

    [
        build_spec(1, "FloorSurfaceConcreteAndSheet", 0.10, 1.70, 2_200.0, 880.0),
        build_spec(2, "UpperSubsoilLayer", 0.25, 1.40, 1_850.0, 1_050.0),
        build_spec(3, "DeepSubsoilLayer", 0.50, 1.40, 1_850.0, 1_050.0),
    ]
}

#[allow(clippy::too_many_arguments)]
fn build_record(
    period_label: &'static str,
    timestamp_iso_utc: &'static str,
    timestamp_utc_s: i64,
    time_s: f64,
    t_struct_c: f64,
    q_struct_w: f64,
    t_medium_c: f64,
    q_medium_w: f64,
    q_floor_surface_net_w: f64,
    t_soil_1_c: f64,
    t_soil_2_c: f64,
    t_soil_3_c: f64,
    t_deep_ground_c: f64,
) -> PreparedThermalStorageRecord {
    let specs = layer_specs();
    let g12 = adjacent_conductance_w_k(
        FLOOR_AREA_M2,
        specs[0].thickness_d_m,
        specs[0].thermal_conductivity_k_w_m_k,
        specs[1].thickness_d_m,
        specs[1].thermal_conductivity_k_w_m_k,
    );
    let g23 = adjacent_conductance_w_k(
        FLOOR_AREA_M2,
        specs[1].thickness_d_m,
        specs[1].thermal_conductivity_k_w_m_k,
        specs[2].thickness_d_m,
        specs[2].thermal_conductivity_k_w_m_k,
    );
    let g3_deep = adjacent_conductance_w_k(
        FLOOR_AREA_M2,
        specs[2].thickness_d_m,
        specs[2].thermal_conductivity_k_w_m_k,
        0.50,
        1.40,
    );

    // Layer 1 (top floor slab)
    let q1_upper = q_floor_surface_net_w;
    let q1_lower = g12 * (t_soil_2_c - t_soil_1_c);
    let q1_net = q1_upper + q1_lower;
    let dt1 = q1_net / specs[0].thermal_capacitance_c_j_k;

    // Layer 2 (interior soil layer: exact C_2 dT_2/dt = G_12(T_1 - T_2) + G_23(T_3 - T_2))
    let q2_upper = g12 * (t_soil_1_c - t_soil_2_c);
    let q2_lower = g23 * (t_soil_3_c - t_soil_2_c);
    let q2_net = q2_upper + q2_lower;
    let dt2 = q2_net / specs[1].thermal_capacitance_c_j_k;

    // Layer 3 (deep soil layer coupled to deep-ground boundary)
    let q3_upper = g23 * (t_soil_2_c - t_soil_3_c);
    let q3_lower = g3_deep * (t_deep_ground_c - t_soil_3_c);
    let q3_net = q3_upper + q3_lower;
    let dt3 = q3_net / specs[2].thermal_capacitance_c_j_k;

    PreparedThermalStorageRecord {
        period_label,
        timestamp_iso_utc,
        timestamp_utc_s,
        time_s,
        greenhouse_structure: build_lumped_mass(
            "GreenhouseSteelStructure",
            18_500.0,
            500.0,
            t_struct_c,
            q_struct_w,
        ),
        growing_gutters_and_wet_medium: build_lumped_mass(
            "GrowingGuttersAndWetRockWool",
            14_200.0,
            3_650.0,
            t_medium_c,
            q_medium_w,
        ),
        top_surface_net_heat_input_w: q_floor_surface_net_w,
        deep_ground_boundary_temperature_c: t_deep_ground_c,
        soil_layers: [
            SoilLayerThermalStateOutput {
                layer_index: 1,
                layer_name: specs[0].layer_name,
                thickness_d_m: specs[0].thickness_d_m,
                thermal_conductivity_k_w_m_k: specs[0]
                    .thermal_conductivity_k_w_m_k,
                thermal_capacitance_c_j_k: specs[0].thermal_capacitance_c_j_k,
                temperature_c: t_soil_1_c,
                temperature_k: t_soil_1_c + 273.15,
                heat_flow_from_upper_w: q1_upper,
                heat_flow_from_lower_w: q1_lower,
                net_heat_flow_in_w: q1_net,
                dt_dt_k_per_s: dt1,
                dt_dt_k_per_h: dt1 * 3_600.0,
            },
            SoilLayerThermalStateOutput {
                layer_index: 2,
                layer_name: specs[1].layer_name,
                thickness_d_m: specs[1].thickness_d_m,
                thermal_conductivity_k_w_m_k: specs[1]
                    .thermal_conductivity_k_w_m_k,
                thermal_capacitance_c_j_k: specs[1].thermal_capacitance_c_j_k,
                temperature_c: t_soil_2_c,
                temperature_k: t_soil_2_c + 273.15,
                heat_flow_from_upper_w: q2_upper,
                heat_flow_from_lower_w: q2_lower,
                net_heat_flow_in_w: q2_net,
                dt_dt_k_per_s: dt2,
                dt_dt_k_per_h: dt2 * 3_600.0,
            },
            SoilLayerThermalStateOutput {
                layer_index: 3,
                layer_name: specs[2].layer_name,
                thickness_d_m: specs[2].thickness_d_m,
                thermal_conductivity_k_w_m_k: specs[2]
                    .thermal_conductivity_k_w_m_k,
                thermal_capacitance_c_j_k: specs[2].thermal_capacitance_c_j_k,
                temperature_c: t_soil_3_c,
                temperature_k: t_soil_3_c + 273.15,
                heat_flow_from_upper_w: q3_upper,
                heat_flow_from_lower_w: q3_lower,
                net_heat_flow_in_w: q3_net,
                dt_dt_k_per_s: dt3,
                dt_dt_k_per_h: dt3 * 3_600.0,
            },
        ],
    }
}

fn reference_records() -> Vec<PreparedThermalStorageRecord> {
    vec![
        // 1. Winter night (11 Jan 2015 02:00 UTC): stored afternoon heat slowly releasing from floor/medium
        build_record(
            "winter_night",
            "2015-01-11T02:00:00Z",
            1_420_941_600,
            3_376_800.0,
            16.4,
            -2_800.0,
            17.5,
            -3_400.0,
            -4_200.0,
            15.6,
            14.4,
            13.1,
            12.0,
        ),
        // 2. Winter noon (11 Jan 2015 12:00 UTC): solar radiation charging floor, structure, and wet medium
        build_record(
            "winter_noon",
            "2015-01-11T12:00:00Z",
            1_420_977_600,
            3_412_800.0,
            19.8,
            6_500.0,
            19.6,
            9_200.0,
            38_400.0,
            21.8,
            16.2,
            13.8,
            12.0,
        ),
        // 3. Spring equinox noon (20 Mar 2015 12:00 UTC): strong daytime thermal charging of floor slab
        build_record(
            "equinox_noon",
            "2015-03-20T12:00:00Z",
            1_426_852_800,
            9_288_000.0,
            23.1,
            8_900.0,
            22.4,
            12_600.0,
            54_200.0,
            25.9,
            18.5,
            15.2,
            13.5,
        ),
        // 4. Summer afternoon (16 Jul 2015 13:00 UTC): warm floor transferring heat downward into subsoil
        build_record(
            "summer_afternoon",
            "2015-07-16T13:00:00Z",
            1_437_051_600,
            19_486_800.0,
            31.8,
            4_100.0,
            29.5,
            7_800.0,
            42_000.0,
            29.8,
            23.4,
            18.9,
            16.0,
        ),
    ]
}

pub fn prepare_thermal_storage_service(
    payload: PrepareThermalStorageInput,
) -> Result<PrepareThermalStorageOutput, AppError> {
    let preset = payload
        .preset_id
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .unwrap_or(DEFAULT_THERMAL_STORAGE_PRESET_ID);

    if preset != DEFAULT_THERMAL_STORAGE_PRESET_ID {
        return Err(AppError::BadRequest(format!(
            "unknown simulation thermal storage preset '{preset}'; \
             supported preset is '{DEFAULT_THERMAL_STORAGE_PRESET_ID}'"
        )));
    }

    let filter = payload
        .period_filter
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .unwrap_or("all");

    let all_records = reference_records();

    let records: Vec<PreparedThermalStorageRecord> = if filter == "all" {
        all_records
    } else {
        let filtered: Vec<PreparedThermalStorageRecord> = all_records
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

    let specs = layer_specs();
    let g12 = adjacent_conductance_w_k(
        FLOOR_AREA_M2,
        specs[0].thickness_d_m,
        specs[0].thermal_conductivity_k_w_m_k,
        specs[1].thickness_d_m,
        specs[1].thermal_conductivity_k_w_m_k,
    );
    let g23 = adjacent_conductance_w_k(
        FLOOR_AREA_M2,
        specs[1].thickness_d_m,
        specs[1].thermal_conductivity_k_w_m_k,
        specs[2].thickness_d_m,
        specs[2].thermal_conductivity_k_w_m_k,
    );
    let g3_deep = adjacent_conductance_w_k(
        FLOOR_AREA_M2,
        specs[2].thickness_d_m,
        specs[2].thermal_conductivity_k_w_m_k,
        0.50,
        1.40,
    );

    Ok(PrepareThermalStorageOutput {
        preset_id: DEFAULT_THERMAL_STORAGE_PRESET_ID,
        execution_mode: "reference_dataset_direct",
        source_reference: "Sourisseau et al. (2026) PLOS One 21(2): e0340619, \
                           Section 2.2, Section 3.1 & TiGwerLib 0.7.0 ThermalInertia / SoilConduction",
        parameters: ThermalStorageParametersOutput {
            floor_area_m2: FLOOR_AREA_M2,
            layers: specs,
            conductance_g12_w_k: g12,
            conductance_g23_w_k: g23,
            conductance_g3_deep_w_k: g3_deep,
            scope_distinction_note:
            "Modelling the thermal capacitance of the wet rock-wool medium captures delayed heat release, \
                 not root-zone nutrient concentrations, EC, pH, or water availability (Section 15).",
        },
        records,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn capacitance_conductance_and_soil_layer_balances_hold_across_all_records() {
        let output = prepare_thermal_storage_service(
            PrepareThermalStorageInput::default(),
        )
            .expect("default thermal storage preset should succeed");

        let p = &output.parameters;
        assert_eq!(output.records.len(), 4);

        // Verify conductance formula G_ij = A / (d_i/(2*k_i) + d_j/(2*k_j))
        let expected_g12 = FLOOR_AREA_M2
            / (p.layers[0].thickness_d_m
            / (2.0 * p.layers[0].thermal_conductivity_k_w_m_k)
            + p.layers[1].thickness_d_m
            / (2.0 * p.layers[1].thermal_conductivity_k_w_m_k));
        assert!((p.conductance_g12_w_k - expected_g12).abs() < 1e-9);

        for record in &output.records {
            for lumped in [
                &record.greenhouse_structure,
                &record.growing_gutters_and_wet_medium,
            ] {
                let expected_c = lumped.mass_kg * lumped.specific_heat_cp_j_kg_k;
                assert!(
                    (lumped.thermal_capacitance_c_th_j_k - expected_c).abs()
                        < 1e-6
                );
                let expected_dt = lumped.net_heat_flow_in_w / expected_c;
                assert!((lumped.dt_dt_k_per_s - expected_dt).abs() < 1e-15);
            }

            // Interior soil layer (j = 2) equation check:
            // C_2 * dT_2/dt = G_12 * (T_1 - T_2) + G_23 * (T_3 - T_2)
            let l1 = &record.soil_layers[0];
            let l2 = &record.soil_layers[1];
            let l3 = &record.soil_layers[2];

            let expected_q2 = p.conductance_g12_w_k
                * (l1.temperature_c - l2.temperature_c)
                + p.conductance_g23_w_k * (l3.temperature_c - l2.temperature_c);
            assert!(
                (l2.thermal_capacitance_c_j_k * l2.dt_dt_k_per_s - expected_q2)
                    .abs()
                    < 1e-9
            );
        }
    }
}