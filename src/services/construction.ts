const DEFAULT_PRESET_ID = 'ctifl_nantes_2014_2015';
const FLOOR_AREA_M2 = 1036.8;
const CULTIVATED_AREA_M2 = 943.488;

export interface EstimateConstructionInput {
  preset_id?: string;
  cost_book_id?: string;
}

const BOQ_SPECS = [
  {
    code: 'STR-01',
    description: 'Hot-dip galvanized Venlo steel frame, gutters (6.97 m), posts & foundations',
    unit: 'm2_floor',
    quantity: 1036.8,
    price_kes: 6200.0,
    price_eur: 45.0,
    source: 'Section 2 geometry: L=43.2 m, W=24.0 m, A_floor=1,036.8 m2',
  },
  {
    code: 'COV-02',
    description: '4 mm horticultural glass cladding & aluminum glazing bars (roof + walls)',
    unit: 'm2_envelope',
    quantity: 2196.4,
    price_kes: 2950.0,
    price_eur: 21.5,
    source: 'Section 2 & 13 roof (1,118.4 m2) + perimeter wall glazing',
  },
  {
    code: 'SCR-03',
    description: 'Dual motorized screens (aluminized thermal screen + shading screen)',
    unit: 'm2_screen',
    quantity: 2073.6,
    price_kes: 1450.0,
    price_eur: 10.5,
    source: 'Section 6 & 10 dual horizontal screen layers (2 x 1,036.8 m2)',
  },
  {
    code: 'VNT-04',
    description: 'Continuous leeward/windward roof vents with rack-and-pinion motors',
    unit: 'm2_vent',
    quantity: 186.624,
    price_kes: 4800.0,
    price_eur: 35.0,
    source: 'Section 8 natural ventilation opening area (18% of floor area)',
  },
  {
    code: 'HTG-05',
    description: '51 mm rail & crop heating pipe network, mixing valves & circulation pumps',
    unit: 'm_pipe',
    quantity: 1152.0,
    price_kes: 1650.0,
    price_eur: 12.0,
    source: 'Section 16 hot-water heating pipe loops',
  },
  {
    code: 'IRR-06',
    description: 'Suspended growing gutters, closed-loop drip fertigation & UV drain recovery skid',
    unit: 'm2_cultivated',
    quantity: 943.488,
    price_kes: 3100.0,
    price_eur: 22.5,
    source: 'Section 2 & 25 cultivated area (r_cul = 0.91) with closed-loop recirculation',
  },
  {
    code: 'CTL-07',
    description: 'Climate computer, PAR/temp/RH/CO2 sensors & CO2 dosing manifold',
    unit: 'lot',
    quantity: 1.0,
    price_kes: 1450000.0,
    price_eur: 10500.0,
    source: 'Section 6, 17, 23 & 24 feedback control loop instrumentation',
  },
];

export function estimateConstruction(input: EstimateConstructionInput = {}) {
  const preset_id = input.preset_id || DEFAULT_PRESET_ID;
  const cost_book_id = input.cost_book_id || 'kenya_witeithie_2026_kes';

  const is_kes = cost_book_id !== 'europe_venlo_reference_eur';
  const currency_code = is_kes ? 'KES' : 'EUR';
  const c_labour = is_kes ? 2450000.0 : 24000.0;
  const c_transport = is_kes ? 1680000.0 : 9500.0;
  const c_installation = is_kes ? 1920000.0 : 18500.0;
  const c_other_specified = is_kes ? 650000.0 : 4800.0;

  let sum_qi_pi = 0.0;
  const bill_of_quantities_items = BOQ_SPECS.map((spec) => {
    const unit_price_p_i = is_kes ? spec.price_kes : spec.price_eur;
    const line_total_q_i_p_i = spec.quantity * unit_price_p_i;
    sum_qi_pi += line_total_q_i_p_i;
    return {
      item_code: spec.code,
      description: spec.description,
      unit: spec.unit,
      quantity_q_i: spec.quantity,
      unit_price_p_i,
      line_total_q_i_p_i,
      geometry_or_model_source: spec.source,
    };
  });

  const c_construction_total = sum_qi_pi + c_labour + c_transport + c_installation + c_other_specified;

  return {
    preset_id,
    cost_book_id,
    currency_code,
    floor_area_m2: FLOOR_AREA_M2,
    cultivated_area_m2: CULTIVATED_AREA_M2,
    bill_of_quantities_items,
    subtotal_materials_and_equipment_sum_qi_pi: sum_qi_pi,
    c_labour,
    c_transport,
    c_installation,
    c_other_specified,
    c_construction_total,
    capex_per_m2_floor: c_construction_total / FLOOR_AREA_M2,
    capex_per_m2_cultivated: c_construction_total / CULTIVATED_AREA_M2,
    engineering_scope_disclaimer:
      'Indicative Bill of Quantities derived from greenhouse geometry and climate-equipment sizing; does not constitute a stamped structural or geotechnical engineering design.',
  };
}
