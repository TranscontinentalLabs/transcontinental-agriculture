V1
interpret_user_message
resolve_location
get_historical_weather
prepare_weather_dataset
calculate_climate_profile
get_crop_requirements
get_greenhouse_designs
prepare_greenhouse_candidates
configure_greenhouse_systems
run_greenhouse_simulation
evaluate_simulation_results
compare_greenhouse_candidates
select_greenhouse_configuration
generate_bill_of_materials
estimate_greenhouse_costs
generate_build_package
generate_greenhouse_layout
assemble_greenhouse_proposal

Start
interpret_user_message
resolve_location
get_historical_weather
calculate_climate_profile
get_greenhouse_designs_based_on_climate_profile
get_crop_requirements
get_greenhouse_designs_based_on_crop_requirements
prepare_greenhouse_candidates
evaluate_best_candidate (the greenhouse designs will have placement of equipment and ventilation we will use predefined ones as of now then we will create dynamic as the system grows, this process wil be done by checking which greenhouse matches the 2 results, every greenhouse will have metadata of the cost to run and construct)
get_build_package
send_3d_layout

nb we will create greenhouse designs for different conditions

| Step | Tool | Output used by the next step |
|---|---|---|
| 1 | `interpret_user_message` | Location text, crops and intent |
| 2 | `resolve_location` | Confirmed coordinates |
| 3 | `get_historical_weather` | Weather dataset, source and quality information |
| 4 | `calculate_climate_profile` | Climate characteristics used to match designs |
| 5 | `get_greenhouse_designs_based_on_climate_profile` | Design IDs with climate-match results |
| 6 | `get_crop_requirements` | Requirements for the requested crops |
| 7 | `get_greenhouse_designs_based_on_crop_requirements` | Design IDs with crop-match results |
| 8 | `prepare_greenhouse_candidates` | Designs that satisfy both sets of requirements, with their metadata |
| 9 | `evaluate_best_candidate` | Ranked candidates, recommendation and reasons |
| 10 | `get_build_package` | Selected design’s materials, equipment, placement and assembly information |
| 11 | `get_3d_layout` | Selected design’s scene data or model reference for the frontend to render |