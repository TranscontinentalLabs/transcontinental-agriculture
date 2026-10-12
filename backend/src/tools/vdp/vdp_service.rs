use serde::{Deserialize, Serialize};
use crate::error::AppError;

#[derive(Deserialize)]
pub struct VDPInput{
    pub air_temperature: f64,
    pub relative_humidity: f64,
}

#[derive(Serialize)]
pub struct VDPOutput{
    pub vdp: f64,
}

pub fn calculate_vdp_service(
    payload: VDPInput
) -> Result<VDPOutput, AppError>{

    let svp = saturation_vapor_pressure(payload.air_temperature);
    let vp = vapour_pressure(svp, payload.relative_humidity);

    let vdp_value = svp - vp;

    Ok(VDPOutput {
        vdp: (vdp_value * 100.0).round() / 100.0,
    })
}

fn saturation_vapor_pressure(t: f64) -> f64 {
    0.6108 * ((17.27 * t) / (t + 237.3)).exp()
}

fn vapour_pressure(e: f64, rh: f64) -> f64 {
    e * (rh / 100.0)
}
