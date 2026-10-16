const ZETA_MIN = 0.01;
const DEG = Math.PI / 180.0;
const FRAC_PI_2 = Math.PI / 2.0;
const GLASS_SAMPLES = 10;

export interface VenloRoofInput {
  ridge_azimuth_rad: number;
  roof_slope_rad: number;
  span_width_m: number;
  optical_span_count: number;
  gutter_height_m: number;
  gutter_width_m: number;
  ridge_height_m: number;
  ridge_width_m: number;
  bar_height_m: number;
  bar_width_m: number;
  bar_spacing_m: number;
}

export interface UncoatedGlassInput {
  thickness_m: number;
  normal_transmittance: number;
  normal_reflectance: number;
}

export interface PrepareSolarGeometryInput {
  roof: VenloRoofInput;
  glass: UncoatedGlassInput;
  solar_elevation_rad: number;
  solar_azimuth_rad: number;
  direct_horizontal_irradiance_w_m2: number;
}

export interface PrepareSolarGeometryOutput {
  tau_glass: number;
  tau_ridges_gutters: number;
  tau_bars: number;
  tau_roof_direct: number;
  transmitted_direct_horizontal_w_m2: number;
  optical_case: number;
  cross_span_elevation_rad: number;
  warnings: string[];
}

function splice(pos: number, neg: number, x: number, width: number): number {
  const scaled = x / width;
  const limit = 0.9999999999;
  if (scaled <= -limit) return neg;
  if (scaled >= limit) return pos;
  const weight = (Math.tanh(Math.tan(scaled * FRAC_PI_2)) + 1.0) / 2.0;
  return pos * weight + neg * (1.0 - weight);
}

function safeAcos(val: number): number {
  return Math.acos(Math.max(-1.0, Math.min(1.0, val)));
}

function geometricSum(q: number, count: number): number {
  let sum = 0.0;
  let power = 1.0;
  for (let i = 0; i < Math.max(0, count); i++) {
    sum += power;
    power *= q;
  }
  return sum;
}

function nestedGeometricSum(q: number, count: number): number {
  let result = 0.0;
  let partialSum = 0.0;
  let power = 1.0;
  for (let i = 0; i < Math.max(0, count); i++) {
    partialSum += power;
    result += partialSum;
    power *= q;
  }
  return result;
}

function interpolateGlass(values: number[], angle: number): number {
  const x = 1.0 + (9.0 * Math.max(0.0, Math.min(FRAC_PI_2, angle))) / FRAC_PI_2;
  const index = Math.min(10, Math.floor(x));
  const u = x - index;
  const y0 = values[index];
  const y1 = values[index + 1];
  const slope0 = (values[index + 1] - values[index - 1]) / 2.0;
  const slope1 = (values[index + 2] - values[index]) / 2.0;
  const u2 = u * u;
  const u3 = u2 * u;
  return (
    (2.0 * u3 - 3.0 * u2 + 1.0) * y0 +
    (u3 - 2.0 * u2 + u) * slope0 +
    (-2.0 * u3 + 3.0 * u2) * y1 +
    (u3 - u2) * slope1
  );
}

class GlassTable {
  transmission: number[] = new Array(13).fill(0);
  absorption: number[] = new Array(13).fill(0);

  constructor(glass: UncoatedGlassInput) {
    const t0 = glass.normal_transmittance;
    const r0 = glass.normal_reflectance;
    const beta = t0 * t0 - r0 * r0 + 2.0 * r0 + 1.0;
    const discriminant = beta * beta - 4.0 * (2.0 - r0) * r0;

    if (!Number.isFinite(discriminant) || discriminant < 0) {
      throw new Error('glass properties cannot define uncoated-glass model');
    }

    const interface_rho = (0.5 * (beta - Math.sqrt(discriminant))) / (2.0 - r0);
    const extinction_ratio = (r0 - interface_rho) / (interface_rho * t0);
    const absorption_coefficient = -Math.log(extinction_ratio) / glass.thickness_m;
    const root_rho = Math.sqrt(interface_rho);
    const refractive_index = (1.0 + root_rho) / (1.0 - root_rho);

    for (let sample = 0; sample < GLASS_SAMPLES; sample++) {
      const angle = (sample * FRAC_PI_2) / 9.0;
      let tau: number;
      let rho: number;

      if (sample === 0) {
        tau = t0;
        rho = r0;
      } else if (sample === GLASS_SAMPLES - 1) {
        tau = 0.0;
        rho = 1.0;
      } else {
        const refracted_angle = Math.asin(Math.sin(angle) / refractive_index);
        const cos_air = Math.cos(angle);
        const cos_glass = Math.cos(refracted_angle);

        const rho1 = Math.pow((refractive_index * cos_air - cos_glass) / (refractive_index * cos_air + cos_glass), 2);
        const rho2 = Math.pow((refractive_index * cos_glass - cos_air) / (refractive_index * cos_glass + cos_air), 2);
        const attenuation = Math.exp((-absorption_coefficient * glass.thickness_m) / cos_glass);

        const tau1 = (Math.pow(1.0 - rho1, 2) * attenuation) / (1.0 - Math.pow(rho1, 2) * Math.pow(attenuation, 2));
        const tau2 = (Math.pow(1.0 - rho2, 2) * attenuation) / (1.0 - Math.pow(rho2, 2) * Math.pow(attenuation, 2));
        const reflectance1 = rho1 * (1.0 + tau1 * attenuation);
        const reflectance2 = rho2 * (1.0 + tau2 * attenuation);

        tau = 0.5 * (tau1 + tau2);
        rho = 0.5 * (reflectance1 + reflectance2);
      }

      this.transmission[sample + 1] = tau;
      this.absorption[sample + 1] = 1.0 - tau - rho;
    }

    const step = FRAC_PI_2 / 9.0;
    for (let sample = 0; sample < GLASS_SAMPLES; sample++) {
      const angle = sample * step;
      const trapezoid_weight = sample === 0 || sample === GLASS_SAMPLES - 1 ? 0.5 : 1.0;
      const weight = trapezoid_weight * step * 2.0 * Math.cos(angle) * Math.sin(angle);
      this.transmission[11] += this.transmission[sample + 1] * weight;
      this.absorption[11] += this.absorption[sample + 1] * weight;
    }

    this.transmission[0] = this.transmission[1];
    this.absorption[0] = this.absorption[1];
    this.transmission[12] = this.transmission[11];
    this.absorption[12] = this.absorption[11];
  }

  at(angle: number) {
    const tau = Math.max(0.0, interpolateGlass(this.transmission, angle));
    const absorption = Math.max(0.0, interpolateGlass(this.absorption, angle));
    return {
      tau,
      rho: Math.max(0.0, 1.0 - tau - absorption),
    };
  }
}

export function prepareSolarGeometry(input: PrepareSolarGeometryInput): PrepareSolarGeometryOutput {
  const { roof, glass: glassInput, solar_elevation_rad, solar_azimuth_rad, direct_horizontal_irradiance_w_m2 } = input;
  const glass = new GlassTable(glassInput);

  if (solar_elevation_rad <= 0.0) {
    return {
      tau_glass: 0.0,
      tau_ridges_gutters: 0.0,
      tau_bars: 0.0,
      tau_roof_direct: 0.0,
      transmitted_direct_horizontal_w_m2: 0.0,
      optical_case: 0,
      cross_span_elevation_rad: ZETA_MIN,
      warnings: [],
    };
  }

  const gamma = solar_elevation_rad;
  const slope = roof.roof_slope_rad;
  const width = roof.span_width_m;
  const spans = roof.optical_span_count;

  const relative_azimuth = solar_azimuth_rad - roof.ridge_azimuth_rad;
  const sin_gamma = Math.sin(gamma);
  const cos_gamma = Math.cos(gamma);
  const sin_slope = Math.sin(slope);
  const cos_slope = Math.cos(slope);

  const across = cos_gamma * Math.sin(relative_azimuth);
  const angle_a = safeAcos(across * sin_slope + sin_gamma * cos_slope);
  const angle_b = safeAcos(-across * sin_slope + sin_gamma * cos_slope);
  const angle_c = safeAcos(-across * sin_slope - sin_gamma * cos_slope);
  const angle_d = safeAcos(across * sin_slope - sin_gamma * cos_slope);

  const kp = across * cos_slope + sin_gamma * sin_slope;
  const kq = across * sin_slope - sin_gamma * cos_slope;
  const ku = across * cos_slope - sin_gamma * sin_slope;
  const kv = -(across * sin_slope + sin_gamma * cos_slope);

  const reflected_b_to_a = safeAcos(
    (kp * cos_slope - kq * sin_slope) * sin_slope + (kp * sin_slope + kq * cos_slope) * cos_slope
  );
  const reflected_a_to_b = safeAcos(
    -(ku * cos_slope + kv * sin_slope) * sin_slope + (-ku * sin_slope + kv * cos_slope) * cos_slope
  );

  const [s1_angle, s2_angle, s4_angle, reflected_angle] =
    Math.sin(relative_azimuth) >= 0.0
      ? [angle_a, angle_b, angle_d, reflected_b_to_a]
      : [angle_b, angle_a, angle_c, reflected_a_to_b];

  const s1 = glass.at(s1_angle);
  const s2 = glass.at(s2_angle);
  const s4 = glass.at(s4_angle);
  const s1_ref = glass.at(reflected_angle);
  const s4_ref = glass.at(s2_angle);

  const bounded_gamma = Math.max(-0.999 * FRAC_PI_2, Math.min(0.999 * FRAC_PI_2, gamma));
  const raw_zeta = Math.atan(sin_gamma / (Math.cos(bounded_gamma) * Math.max(1e-3, Math.abs(Math.sin(relative_azimuth)))));
  const zeta = Math.max(ZETA_MIN, splice(raw_zeta, 0.0, gamma - DEG, DEG));

  const l0 = width / (2.0 * cos_slope);
  const l1 = l0 * Math.sin(zeta + slope);
  const l2 = l0 * Math.sin(zeta - slope);

  let tau_glass = 0.0;
  let optical_case = 0;

  if (zeta - ZETA_MIN <= 1e-6) {
    tau_glass = 0.0;
    optical_case = 0;
  } else if (zeta < slope) {
    const m = Math.floor(Math.min(spans, 0.5 * (1.0 + Math.tan(slope) / Math.tan(zeta))));
    const la = Math.max(0.0, l0 * (m * Math.sin(slope + zeta) - (m + 1) * Math.sin(slope - zeta)));
    const lb = l0 * ((1 - m) * Math.sin(slope + zeta) + m * Math.sin(slope - zeta));
    const q = s1.tau * s4.tau;
    const k1 = geometricSum(q, m - 1);
    const k2 = geometricSum(q, m);
    tau_glass = (la * s1.tau * (s4.rho * k1 + Math.pow(q, m - 1)) + lb * s1.tau * (s4.rho * k2 + Math.pow(q, m))) / Math.max(1e-6, la + lb);
    optical_case = 1;
  } else if (zeta < 3.0 * slope) {
    const direct = (l1 * s1.tau + l2 * s2.tau) / Math.max(1e-6, l1 + l2);
    const khi = Math.abs(2.0 * slope - zeta);
    const m = Math.floor(Math.min(spans, 0.5 * (1.0 + Math.tan(slope) / Math.tan(Math.max(1e-4, khi)))));
    const q = s1_ref.tau * s4_ref.tau;
    const denominator = Math.max(l0 / 1000.0, l1 + l2);

    if (zeta < 2.0 * slope) {
      const la = l0 * (m * Math.sin(zeta - slope) - (m - 1) * Math.sin(3.0 * slope - zeta));
      const lb = l0 * (-Math.sin(zeta - slope) + Math.sin(3.0 * slope - zeta));
      const k1 = geometricSum(q, m - 1);
      const k2 = nestedGeometricSum(q, m - 2);
      const reflected = (la * s2.rho * s1_ref.tau * (s4_ref.rho * k1 + Math.pow(q, m - 1)) + lb * s2.rho * s1_ref.tau * (s4_ref.rho * k2 + k1)) / denominator;
      tau_glass = direct + reflected;
      optical_case = 2;
    } else {
      const la = l0 * (-(m - 1) * Math.sin(zeta - slope) + m * Math.sin(3.0 * slope - zeta));
      const lb = l0 * (Math.sin(zeta - slope) - Math.sin(3.0 * slope - zeta));
      const k1 = geometricSum(q, m);
      const k2 = nestedGeometricSum(q, m - 1);
      const reflected = (la * s2.rho * s1_ref.tau * s4_ref.rho * k1 + lb * s2.rho * s1_ref.tau * s4_ref.rho * k2) / denominator;
      tau_glass = direct + reflected;
      optical_case = 3;
    }
  } else {
    tau_glass = (l1 * s1.tau + l2 * s2.tau) / Math.max(l0 / 1000.0, l1 + l2);
    optical_case = 4;
  }

  // Shadow factors
  const hr = roof.ridge_height_m;
  const wr = roof.ridge_width_m;
  const hg = roof.gutter_height_m;
  const wg = roof.gutter_width_m;
  const lr = hr * Math.cos(zeta) + wr * Math.sin(zeta);
  const lg = hg * Math.cos(zeta) + wg * Math.sin(zeta);

  const m2 = 1 + Math.floor(Math.max(0.0, Math.min(spans - 1, 0.5 * Math.tan(slope) / Math.tan(zeta))));
  const distance = (2 * m2 - 1) * width;
  const rise_twice = width * Math.tan(slope);
  const zeta1 = Math.atan((rise_twice - hr - hg) / (distance + wr + wg));
  const zeta2 = Math.atan((rise_twice + hr + hg) / (distance - wr - wg));
  const zeta3 = Math.atan((rise_twice + hr - hg) / (distance - wr + wg));
  const zeta4 = Math.atan((rise_twice - hr + hg) / (distance + wr - wg));

  let overlap = 0.0;
  if (zeta - ZETA_MIN <= 1e-6) {
    overlap = 0.0;
  } else if (zeta > zeta3 && zeta <= zeta4) {
    overlap = lr;
  } else if (zeta > zeta1 && zeta <= zeta3) {
    const le = 0.5 * (distance + wg) + 0.5 * hg / Math.tan(zeta);
    const ld = (0.5 * rise_twice - 0.5 * hr) / Math.tan(zeta) - 0.5 * wr;
    overlap = (le - ld) * Math.sin(zeta);
  } else if (zeta > zeta4 && zeta < zeta2) {
    const le = (0.5 * rise_twice + 0.5 * hr) / Math.tan(zeta) + 0.5 * wr;
    const ld = 0.5 * (distance - wg) - 0.5 * hg / Math.tan(zeta);
    overlap = (le - ld) * Math.sin(zeta);
  }

  const structural_transmission = Math.max(0.0, 1.0 - Math.max(0.0, lg + lr - overlap) / (width * Math.sin(zeta)));
  const tau_ridges_gutters = splice(structural_transmission, 0.0, gamma - DEG, DEG);

  const nu = splice(Math.atan(Math.tan(gamma) / Math.max(1e-5, Math.abs(Math.cos(relative_azimuth)))), 1e-4, gamma - 0.1 * DEG, 0.1 * DEG);
  const hs = l0 * sin_slope;
  const x_plus = 0.5 * width + hs / Math.tan(zeta);
  const x_minus = 0.5 * width - hs / Math.tan(zeta);
  const y = hs / Math.tan(nu);
  const lx = Math.hypot(x_plus, y);
  const ly = Math.hypot(x_minus, y);

  const bar_x = roof.bar_height_m / Math.tan(zeta);
  const bar_y = roof.bar_width_m + roof.bar_height_m / Math.tan(nu);
  const ab = Math.hypot(bar_x, bar_y);
  const bad = Math.atan(bar_y / bar_x) - Math.atan(y / x_plus);
  const fed = Math.PI - Math.atan(bar_y / bar_x) - Math.atan(y / x_minus);
  const d1 = ab * Math.abs(Math.sin(bad));
  const d2 = ab * Math.abs(Math.sin(fed));

  const denom = roof.bar_spacing_m * width;
  const shadow1 = (lx * d1) / denom;
  const shadow2 = (ly * d2) / denom;
  const bars_high = 1.0 - shadow1 - shadow2;
  const bars_low = (1.0 - shadow1) * (1.0 - shadow2);

  const tau_bars = Math.max(0.0, splice(splice(bars_high, bars_low, zeta - slope, DEG), 0.0, gamma - 2.0 * DEG, 2.0 * DEG));
  const tau_roof_direct = tau_glass * tau_ridges_gutters * tau_bars;
  const transmitted_direct_horizontal_w_m2 = tau_roof_direct * direct_horizontal_irradiance_w_m2;

  return {
    tau_glass,
    tau_ridges_gutters,
    tau_bars,
    tau_roof_direct,
    transmitted_direct_horizontal_w_m2,
    optical_case,
    cross_span_elevation_rad: zeta,
    warnings: [],
  };
}

export function prepareDiffuseSolarGeometry(input: { preset_id?: string; period_filter?: string } = {}) {
  // Transmitted total horizontal reference values
  const periods = [
    { period_label: 'winter_night', transmitted_total_horizontal_w_m2: 0.0 },
    { period_label: 'winter_noon', transmitted_total_horizontal_w_m2: 154.2 },
    { period_label: 'equinox_noon', transmitted_total_horizontal_w_m2: 382.6 },
    { period_label: 'summer_afternoon', transmitted_total_horizontal_w_m2: 546.8 },
  ];
  const filter = input.period_filter?.trim() || 'all';
  const records = filter === 'all' ? periods : periods.filter((p) => p.period_label === filter);
  return {
    preset_id: input.preset_id || 'ctifl_nantes_2014_2015',
    records,
  };
}
