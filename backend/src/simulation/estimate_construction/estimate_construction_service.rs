use serde::{Deserialize, Serialize};

const DEFAULT_PRESET_ID: &str = "ctifl_nantes_2014_2015";
const FLOOR_AREA_M2: f64 = 1_036.8;
const CULTIVATED_AREA_M2: f64 = 943.488;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EstimateConstructionInput {
    pub preset_id: Option<String>,
    /// Supported cost books: "kenya_witeithie_2026_kes" (default) or "europe_venlo_reference_eur"
    pub cost_book_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EstimateConstructionOutput {
    pub preset_id: String,
    pub cost_book_id: String,
    pub currency_code: String,
    pub floor_area_m2: f64,
    pub cultivated_area_m2: f64,
    pub bill_of_quantities_items: Vec<BoqLineItem>,
    pub subtotal_materials_and_equipment_sum_qi_pi: f64,
    pub c_labour: f64,
    pub c_transport: f64,
    pub c_installation: f64,
    pub c_other_specified: f64,
    pub c_construction_total: f64,
    pub capex_per_m2_floor: f64,
    pub capex_per_m2_cultivated: f64,
    pub engineering_scope_disclaimer: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoqLineItem {
    pub item_code: String,
    pub description: String,
    pub unit: String,
    pub quantity_q_i: f64,
    pub unit_price_p_i: f64,
    pub line_total_q_i_p_i: f64,
    pub geometry_or_model_source: String,
}

struct BoqSpec {
    code: &'static str,
    description: &'static str,
    unit: &'static str,
    quantity: f64,
    price_kes: f64,
    price_eur: f64,
    source: &'static str,
}

const BOQ_SPECS: [BoqSpec; 7] = [
    BoqSpec {
        code: "STR-01",
        description: "Hot-dip galvanized Venlo steel frame, gutters (6.97 m), posts & foundations",
        unit: "m2_floor",
        quantity: 1_036.8,
        price_kes: 6_200.0,
        price_eur: 45.0,
        source: "Section 2 geometry: L=43.2 m, W=24.0 m, A_floor=1,036.8 m2",
    },
    BoqSpec {
        code: "COV-02",
        description: "4 mm horticultural glass cladding & aluminum glazing bars (roof + walls)",
        unit: "m2_envelope",
        quantity: 2_196.4,
        price_kes: 2_950.0,
        price_eur: 21.5,
        source: "Section 2 & 13 roof (1,118.4 m2) + perimeter wall glazing",
    },
    BoqSpec {
        code: "SCR-03",
        description: "Dual motorized screens (aluminized thermal screen + shading screen)",
        unit: "m2_screen",
        quantity: 2_073.6,
        price_kes: 1_450.0,
        price_eur: 10.5,
        source: "Section 6 & 10 dual horizontal screen layers (2 x 1,036.8 m2)",
    },
    BoqSpec {
        code: "VNT-04",
        description: "Continuous leeward/windward roof vents with rack-and-pinion motors",
        unit: "m2_vent",
        quantity: 186.624,
        price_kes: 4_800.0,
        price_eur: 35.0,
        source: "Section 8 natural ventilation opening area (18% of floor area)",
    },
    BoqSpec {
        code: "HTG-05",
        description: "51 mm rail & crop heating pipe network, mixing valves & circulation pumps",
        unit: "m_pipe",
        quantity: 1_152.0,
        price_kes: 1_650.0,
        price_eur: 12.0,
        source: "Section 16 hot-water heating pipe loops",
    },
    BoqSpec {
        code: "IRR-06",
        description: "Suspended growing gutters, closed-loop drip fertigation & UV drain recovery skid",
        unit: "m2_cultivated",
        quantity: 943.488,
        price_kes: 3_100.0,
        price_eur: 22.5,
        source: "Section 2 & 25 cultivated area (r_cul = 0.91) with closed-loop recirculation",
    },
    BoqSpec {
        code: "CTL-07",
        description: "Climate computer, PAR/temp/RH/CO2 sensors & CO2 dosing manifold",
        unit: "lot",
        quantity: 1.0,
        price_kes: 1_450_000.0,
        price_eur: 10_500.0,
        source: "Section 6, 17, 23 & 24 feedback control loop instrumentation",
    },
];

pub fn estimate_construction_service(
    input: EstimateConstructionInput,
) -> Result<EstimateConstructionOutput, String> {
    let preset_id = input
        .preset_id
        .unwrap_or_else(|| DEFAULT_PRESET_ID.to_string());
    if preset_id != DEFAULT_PRESET_ID {
        return Err(format!(
            "Unsupported preset_id '{}'. Expected '{}'.",
            preset_id, DEFAULT_PRESET_ID
        ));
    }

    let cost_book_id = input
        .cost_book_id
        .unwrap_or_else(|| "kenya_witeithie_2026_kes".to_string());

    let (currency_code, is_kes, c_labour, c_transport, c_installation, c_other) =
        match cost_book_id.as_str() {
            "kenya_witeithie_2026_kes" => (
                "KES",
                true,
                2_450_000.0,
                1_680_000.0,
                1_920_000.0,
                650_000.0,
            ),
            "europe_venlo_reference_eur" => (
                "EUR",
                false,
                24_000.0,
                9_500.0,
                18_500.0,
                4_800.0,
            ),
            other => {
                return Err(format!(
                    "Unsupported cost_book_id '{}'. Valid values: kenya_witeithie_2026_kes, europe_venlo_reference_eur.",
                    other
                ))
            }
        };

    let mut items = Vec::with_capacity(BOQ_SPECS.len());
    let mut sum_qi_pi = 0.0;
    for spec in BOQ_SPECS {
        let p_i = if is_kes { spec.price_kes } else { spec.price_eur };
        let line_total = spec.quantity * p_i;
        sum_qi_pi += line_total;
        items.push(BoqLineItem {
            item_code: spec.code.to_string(),
            description: spec.description.to_string(),
            unit: spec.unit.to_string(),
            quantity_q_i: spec.quantity,
            unit_price_p_i: p_i,
            line_total_q_i_p_i: line_total,
            geometry_or_model_source: spec.source.to_string(),
        });
    }

    // C_construction = sum_i q_i * p_i + C_labour + C_transport + C_installation + C_other_specified
    let c_construction_total =
        sum_qi_pi + c_labour + c_transport + c_installation + c_other;

    Ok(EstimateConstructionOutput {
        preset_id,
        cost_book_id,
        currency_code: currency_code.to_string(),
        floor_area_m2: FLOOR_AREA_M2,
        cultivated_area_m2: CULTIVATED_AREA_M2,
        bill_of_quantities_items: items,
        subtotal_materials_and_equipment_sum_qi_pi: sum_qi_pi,
        c_labour,
        c_transport,
        c_installation,
        c_other_specified: c_other,
        c_construction_total,
        capex_per_m2_floor: c_construction_total / FLOOR_AREA_M2,
        capex_per_m2_cultivated: c_construction_total / CULTIVATED_AREA_M2,
        engineering_scope_disclaimer: "Indicative Bill of Quantities derived from greenhouse geometry and climate-equipment sizing; does not constitute a stamped structural or geotechnical engineering design.".to_string(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_boq_construction_cost_equation() {
        let out = estimate_construction_service(EstimateConstructionInput::default()).unwrap();
        let sum_lines: f64 = out
            .bill_of_quantities_items
            .iter()
            .map(|i| i.quantity_q_i * i.unit_price_p_i)
            .sum();
        assert!((out.subtotal_materials_and_equipment_sum_qi_pi - sum_lines).abs() < 1e-9);

        let expected_total = sum_lines
            + out.c_labour
            + out.c_transport
            + out.c_installation
            + out.c_other_specified;
        assert!((out.c_construction_total - expected_total).abs() < 1e-9);
    }
}