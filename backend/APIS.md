**Health**
curl -X GET "http://localhost:8000/health" \
| jq  

curl -X POST "http://localhost:8000/messaging/user-message" \
-H "Content-Type: application/json" \
-d '{
"user_message": ""
}' 
| jq

curl -X POST "http://localhost:8000/tools/start-simulation" \
-H "Content-Type: application/json" \
| jq  

curl -X POST "http://localhost:8000/tools/calculate-vdp" \
-H "Content-Type: application/json" \
-d '{
"air_temperature": 28.0,
"relative_humidity": 70.0
}' 
| jq

curl -X POST "http://localhost:8000/simulation/get-greenhouse-configuration" \
-H "Content-Type: application/json" \
| jq

curl -sS -X POST \
"http://localhost:8000/tools/prepare-solar-geometry" \
-H "Content-Type: application/json" \
-d '{
"roof": {
"ridge_azimuth_rad": 5.777039824101231,
"roof_slope_rad": 0.38397243543875,
"span_width_m": 4.0,
"optical_span_count": 10,
"gutter_height_m": 0.155,
"gutter_width_m": 0.105,
"ridge_height_m": 0.035,
"ridge_width_m": 0.040,
"bar_height_m": 0.052,
"bar_width_m": 0.022,
"bar_spacing_m": 1.35
},
"glass": {
"thickness_m": 0.004,
"normal_transmittance": 0.90,
"normal_reflectance": 0.08
},
"solar_elevation_rad": 0.7853981633974483,
"solar_azimuth_rad": 3.141592653589793,
"direct_horizontal_irradiance_w_m2": 600.0
}' | jq