use serde::{Deserialize, Serialize};

use crate::error::AppError;

pub const DEFAULT_PRESET_ID: &str = "ctifl_nantes_2014_2015";

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GreenhouseConfigurationInput {
    /// Optional research preset identifier.
    /// Defaults to `"ctifl_nantes_2014_2015"` when omitted.
    pub preset_id: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct BasisQuantity<T> {
    pub value: T,
    pub unit: &'static str,
    pub area_basis: &'static str,
    pub provenance: &'static str,
}

#[derive(Debug, Clone, Serialize)]
pub struct ReferenceDimensionsOutput {
    pub length_m: f64,
    pub width_m: f64,
    pub r_cul: f64,
    pub h_gutter_m: f64,
    pub h_ridge_m: f64,
    pub mean_height_m: f64,
    pub north_alleyway_width_m: f64,
    pub south_alleyway_width_m: f64,
}

#[derive(Debug, Clone, Serialize)]
pub struct ReferenceRoofCoverOutput {
    pub greenhouse_family: &'static str,
    pub span_count: u8,
    pub optical_span_count_reference: u16,
    pub span_width_m: f64,
    pub roof_slope_deg: f64,
    pub roof_slope_rad: f64,
    pub ridge_azimuth_deg_from_north: f64,
    pub orientation_offset_from_north_deg: f64,
    pub glazing_material: &'static str,
    pub glass_thickness_m: f64,
    pub normal_transmittance: f64,
    pub normal_reflectance: f64,
}

#[derive(Debug, Clone, Serialize)]
pub struct ReferenceVentilationOutput {
    pub arrangement: &'static str,
    pub max_opening_angle_deg: f64,
    pub closed_crack_virtual_height_mm: f64,
}

#[derive(Debug, Clone, Serialize)]
pub struct ReferenceScreensOutput {
    pub upper_shading_screen: &'static str,
    pub lower_thermal_screen: &'static str,
    pub south_wall_curtain: &'static str,
    pub installation_level: &'static str,
}

#[derive(Debug, Clone, Serialize)]
pub struct ReferenceHeatingAndMixingOutput {
    pub primary_heat_source: &'static str,
    pub pipe_networks: [&'static str; 3],
    pub air_mixing_ducts: &'static str,
}

#[derive(Debug, Clone, Serialize)]
pub struct ReferenceCropLayoutOutput {
    pub cultivar: &'static str,
    pub row_count: u8,
    pub substrate: &'static str,
    pub growing_gutter_height_m: f64,
    pub mature_plant_apex_height_m: f64,
    pub floor_covering_under_crop: &'static str,
}

#[derive(Debug, Clone, Serialize)]
pub struct ReferenceWallBoundariesOutput {
    pub south_wall: &'static str,
    pub north_wall: &'static str,
    pub east_wall: &'static str,
    pub west_wall: &'static str,
    pub south_wall_share_of_cover_pct: f64,
}

#[derive(Debug, Clone, Serialize)]
pub struct GreenhouseConfigurationOutput {
    pub preset_id: &'static str,
    pub execution_mode: &'static str,
    pub source_reference: &'static str,

    /// Exact values used by the reference Modelica geometry equations.
    pub floor_area_m2: f64,
    pub cultivated_area_m2: f64,
    pub volume_m3: f64,

    /// Rounded figures reported in the published article text (Section 2.2).
    pub published_floor_area_m2_rounded: f64,
    pub published_cultivated_area_m2_rounded: f64,

    /// Explicit unit and area-basis metadata for downstream tools.
    pub floor_area_detail: BasisQuantity<f64>,
    pub cultivated_area_detail: BasisQuantity<f64>,
    pub volume_detail: BasisQuantity<f64>,

    pub dimensions: ReferenceDimensionsOutput,
    pub roof_and_cover: ReferenceRoofCoverOutput,
    pub ventilation: ReferenceVentilationOutput,
    pub screens: ReferenceScreensOutput,
    pub heating_and_mixing: ReferenceHeatingAndMixingOutput,
    pub crop_layout: ReferenceCropLayoutOutput,
    pub boundary_exposure: ReferenceWallBoundariesOutput,
}

pub fn get_greenhouse_configuration_service(
    payload: GreenhouseConfigurationInput,
) -> Result<GreenhouseConfigurationOutput, AppError> {
    let preset = payload
        .preset_id
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .unwrap_or(DEFAULT_PRESET_ID);

    if preset != DEFAULT_PRESET_ID {
        return Err(AppError::BadRequest(format!(
            "unknown simulation greenhouse preset '{preset}'; \
             supported preset is '{DEFAULT_PRESET_ID}'"
        )));
    }

    Ok(GreenhouseConfigurationOutput {
        preset_id: DEFAULT_PRESET_ID,
        execution_mode: "reference_dataset_direct",
        source_reference: "Sourisseau et al. (2026) PLOS One 21(2): e0340619, \
                           Section 2.2 & TiGwerLib 0.7.0 CTIFLGreenhouseBaseModel",

        floor_area_m2: 1036.8,
        cultivated_area_m2: 943.488,
        volume_m3: 7646.4,

        published_floor_area_m2_rounded: 1037.0,
        published_cultivated_area_m2_rounded: 944.0,

        floor_area_detail: BasisQuantity {
            value: 1036.8,
            unit: "m2",
            area_basis: "whole_greenhouse_floor",
            provenance: "reference_parameter (43.2 m * 24.0 m)",
        },
        cultivated_area_detail: BasisQuantity {
            value: 943.488,
            unit: "m2",
            area_basis: "cultivated_floor",
            provenance: "reference_parameter (r_cul=0.91 * 1036.8 m2; reported as ~944 m2)",
        },
        volume_detail: BasisQuantity {
            value: 7646.4,
            unit: "m3",
            area_basis: "whole_greenhouse_enclosed_air",
            provenance: "reference_parameter (1036.8 m2 * 7.375 m mean height)",
        },

        dimensions: ReferenceDimensionsOutput {
            length_m: 43.2,
            width_m: 24.0,
            r_cul: 0.91,
            h_gutter_m: 6.97,
            h_ridge_m: 7.78,
            mean_height_m: 7.375,
            north_alleyway_width_m: 3.0,
            south_alleyway_width_m: 1.2,
        },
        roof_and_cover: ReferenceRoofCoverOutput {
            greenhouse_family: "Venlo multispan compartment",
            span_count: 6,
            optical_span_count_reference: 10,
            span_width_m: 4.0,
            roof_slope_deg: 22.0,
            roof_slope_rad: 0.383_972_435_438_75,
            ridge_azimuth_deg_from_north: 331.0,
            orientation_offset_from_north_deg: -29.0,
            glazing_material: "Uncoated single glass pane",
            glass_thickness_m: 0.004,
            normal_transmittance: 0.90,
            normal_reflectance: 0.08,
        },
        ventilation: ReferenceVentilationOutput {
            arrangement: "Actuated alternate roof vents (leeward and windward)",
            max_opening_angle_deg: 44.0,
            closed_crack_virtual_height_mm: 6.0,
        },
        screens: ReferenceScreensOutput {
            upper_shading_screen: "Svensson Harmony 3315 O FR",
            lower_thermal_screen: "Svensson Luxous 1547 D FR",
            south_wall_curtain: "Svensson Luxous 1547 D FR",
            installation_level: "Horizontal screens unfolded at truss/gutter level (6.97 m)",
        },
        heating_and_mixing: ReferenceHeatingAndMixingOutput {
            primary_heat_source: "Natural gas boiler supplying CTIFL facility hot-water loops",
            pipe_networks: [
                "Rails 51 loops close to floor (between tomato rows)",
                "Forcas loops inside crop canopy (1 per row)",
                "PE anti-condensation loops near stems (1 per row)",
            ],
            air_mixing_ducts: "4 axial-fan perforated plastic ducts (0.8 m diameter) below growing gutters",
        },
        crop_layout: ReferenceCropLayoutOutput {
            cultivar: "Clodano F1 Hybrid (Syngenta)",
            row_count: 15,
            substrate: "Soilless rock wool medium on elevated growing gutters",
            growing_gutter_height_m: 1.1,
            mature_plant_apex_height_m: 3.9,
            floor_covering_under_crop: "White plastic sheeting",
        },
        boundary_exposure: ReferenceWallBoundariesOutput {
            south_wall: "Outdoor exposed (fitted with internal thermal curtain)",
            north_wall: "Adjoining heated corridor (concrete alleyway side)",
            east_wall: "Adjoining heated corridor",
            west_wall: "Adjoining similar greenhouse compartment (neutral exchange assumed)",
            south_wall_share_of_cover_pct: 17.0,
        },
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_reference_greenhouse_configuration_by_default() {
        let result = get_greenhouse_configuration_service(
            GreenhouseConfigurationInput::default(),
        )
            .expect("default reference preset should succeed");

        assert_eq!(result.preset_id, DEFAULT_PRESET_ID);
        assert!((result.floor_area_m2 - 1036.8).abs() < 1e-9);
        assert!((result.cultivated_area_m2 - 943.488).abs() < 1e-9);
        assert!((result.volume_m3 - 7646.4).abs() < 1e-9);
        assert_eq!(result.published_floor_area_m2_rounded, 1037.0);
        assert_eq!(result.published_cultivated_area_m2_rounded, 944.0);
    }

    #[test]
    fn rejects_unknown_preset() {
        let result = get_greenhouse_configuration_service(
            GreenhouseConfigurationInput {
                preset_id: Some("unknown_preset".into()),
            },
        );

        assert!(matches!(result, Err(AppError::BadRequest(_))));
    }
}