# Transcontinental Agriculture — V1 Development Reference

Updated: 7 October 2026  
Status: Working agreement; refine as implementation provides evidence.

## 1. Purpose and current scope

Build the first working path from a user's location and crop choice to a recommended predefined greenhouse, its build package and its browser-rendered 3D layout.

Example request:

> I want to build a greenhouse for capsicum in Witeithie, Juja Sub-county, Kiambu County, Kenya.

This first implementation is a **catalogue-based recommendation workflow**. It matches climate and crop requirements against supported greenhouse designs. Full climate simulation and dynamic design generation are later extensions.

The wider product remains a farm brain that will manage the physical farm after construction. This document covers the initial design-recommendation workflow only; completing it does not yet mean live farm management is implemented.

## 2. Agreed development approach

- Build collaboratively, one working capability at a time; no fixed roadmap in this document.
- Use n8n for orchestration and Gemini for interpretation and review.
- Do not build our own orchestrator for this version.
- Use existing APIs, models and libraries where useful; write our backend integrations and domain logic.
- Start with clearly labelled sample data and a working shell, then replace sample providers with real ones while preserving the tool contracts.
- Do not require the user's budget or exact land size at initial intake.
- Designs have standard dimensions and a required footprint. Confirm physical site fit before construction.
- Use predefined equipment positions, ventilation and growing layouts initially.
- Keep project records and result references outside model conversation memory.
- Tool names below are logical capabilities, not separate services. HTTP routes and detailed schemas remain implementation decisions.

## 3. Component responsibilities

| Component | Responsibility |
| --- | --- |
| Frontend | Capture requests, obtain location confirmation, show progress/results and render the 3D scene |
| n8n | Coordinate calls, preserve workflow progress, handle branches and bounded retries |
| Gemini | Interpret requests and explain/review evidence using structured outputs |
| Backend | Validate inputs, retrieve data, match catalogue entries, calculate comparisons and serve saved results |
| Storage | Preserve requests, datasets, catalogue versions, recommendations and asset references |
| Design catalogue | Supply consistent design metadata, costs, build packages and scene references |

The browser renders the 3D layout. Neither Gemini nor n8n needs to generate executable frontend code.

## 4. Workflow and data handoffs

| # | Tool | Input | Output | Next consumer |
| --- | --- | --- | --- | --- |
| 1 | `interpret_user_message` | User message and relevant project context | Intent, location text, crops, missing information | Location resolver and crop lookup |
| 2 | `resolve_location` | Location text and available geographic context | Candidate locations, coordinates, source and confirmation status | Weather retrieval after confirmation |
| 3 | `get_historical_weather` | Confirmed coordinates, period and required variables | Dataset reference, source, spatial/temporal coverage and quality flags | Climate-profile calculation |
| 4 | `calculate_climate_profile` | Validated weather dataset | Seasonal summaries, variability, relevant extremes and limitations | Climate-based design lookup |
| 5 | `get_greenhouse_designs_based_on_climate_profile` | Climate profile and catalogue version | Design IDs, eligibility/match results and reasons | Candidate preparation |
| 6 | `get_crop_requirements` | Requested crops and supported cultivation assumptions | Versioned crop requirements and unresolved choices | Crop-based design lookup |
| 7 | `get_greenhouse_designs_based_on_crop_requirements` | Crop requirements and the same catalogue version | Design IDs, eligibility/match results and reasons | Candidate preparation |
| 8 | `prepare_greenhouse_candidates` | Climate results, crop results and design metadata | Common eligible designs, costs, assumptions and outstanding checks | Candidate evaluation |
| 9 | `evaluate_best_candidate` | Prepared candidates and explicit evaluation policy | Ranked candidates, selected design/configuration version and selection reasons, or no suitable match | Build-package and layout retrieval |
| 10 | `get_build_package` | Selected design/configuration ID and version | Materials, equipment, placement, assembly information and verification requirements | Proposal display/download |
| 11 | `get_3d_layout` | The same selected design/configuration ID and version | Scene data or 3D asset reference, object IDs and units | Frontend renderer |

### Dependencies

- Step 1 supplies both the location and crop branches.
- Location branch: 2 → 3 → 4 → 5.
- Crop branch: 6 → 7; it can run alongside the location branch.
- Both branches meet at step 8.
- Step 9 selects from the resulting candidates.
- Steps 10 and 11 can run together after selection.
- Location clarification pauses only work requiring the confirmed location.
- A missing or invalid dependency prevents downstream results from being presented as complete.

## 5. Tool behaviour

### `interpret_user_message`

Extract the user's intent, location wording and crops. Do not invent coordinates, a budget, a land size or a selected greenhouse. Distinguish new-build requests from requests concerning an existing greenhouse; route unsupported intents to clarification.

Illustrative output:

```json
{
  "intent": "propose_new_greenhouse",
  "location_query": "Witeithie, Juja Sub-county, Kiambu County, Kenya",
  "crops": ["capsicum"],
  "clarification_needed": false
}
```

This only means the message contains enough information to attempt lookup; it does not confirm geographic resolution.

### `resolve_location`

Return sourced candidate matches. Preserve the user's original wording. If multiple plausible matches exist, request confirmation. Distinguish a locality's representative coordinates from an actual farm pin; record which is being used.

### `get_historical_weather`

Retrieve and retain the full time series. Check units, timestamps, coverage and missing values before downstream use. These checks can be internal functions rather than another agent tool. Label station observations separately from gridded/modelled estimates. Return references and metadata to the workflow instead of unnecessarily passing every reading through Gemini.

### `calculate_climate_profile`

Derive the profile from the dataset. Retain seasonal variation and extremes rather than relying only on annual averages. Do not implement the proposed single climate-signature score as an established method yet; it remains a future research idea.

### Climate and crop design lookups

Query one shared, versioned catalogue. Return common design IDs with explicit match evidence. Rules should distinguish eligible, ineligible and unknown. Unsupported or missing requirements must not silently pass as a match.

For multiple crops, assess suitability for the proposed zones and configuration; separate crop compatibility does not automatically establish that both can share identical conditions.

### `prepare_greenhouse_candidates`

Intersect eligible design IDs from both branches and attach metadata. Preserve exclusions and uncertain matches separately for explanation. Return an explicit no-match result if there is no eligible overlap.

### `evaluate_best_candidate`

Use a documented selection policy. The exact thresholds, priorities and any weights must be chosen explicitly during implementation. Do not invent them implicitly in a prompt.

Backend code enforces eligibility and calculates comparable metrics. Gemini can explain trade-offs or propose a selection from the supplied eligible candidates; the backend validates any selected ID. If costs or assumptions are not comparable, explain that limitation rather than forcing a precise ranking.

“Best” means best among the evaluated predefined candidates under the stated policy, not a globally optimal or field-validated design.

### `get_build_package` and `get_3d_layout`

Both retrieve material for the same design/configuration version. The build package should identify preliminary details and any required structural/site verification. The layout returns assets or structured scene information for our renderer; it does not require model-generated executable code.

## 6. Minimum catalogue metadata

| Group | Fields to define |
| --- | --- |
| Identity | Stable design ID, version, name, description and provenance |
| Geometry | Dimensions, required footprint, orientation assumptions and layout units |
| Structure | Shape, covering, framing specifications and ventilation arrangement |
| Growing layout | Supported crops, cultivation method, zones and capacity assumptions |
| Climate suitability | Applicable ranges/rules, seasonal constraints, evidence and limitations |
| Equipment | Types, positions, capacities and operating assumptions |
| Construction cost | Currency, dated price basis, quantities, inclusions and exclusions |
| Operating cost | Currency, period, crop cycle/occupancy assumptions, tariffs, labour and included inputs |
| Build package | Versioned materials, assembly and installation references |
| 3D scene | Asset or scene reference, object IDs, scale and coordinate convention |

Catalogue operating costs are reference estimates unless adjusted for the site's resource use and prices. Display them accordingly.

## 7. Shared data conventions

- Use stable IDs to connect records and retain versions of changing profiles/designs.
- State measurement units explicitly and use consistent timestamps/time zones.
- Include provenance, assumptions and quality warnings in results where applicable.
- Distinguish sample data, provider data, calculated estimates and verified site facts.
- Preserve the weather period and catalogue/crop-profile versions used in a recommendation.
- Tool results should distinguish success, clarification needed, no match and error; do not encode every outcome as successful text.
- Large files and time series stay in storage; return references plus useful summaries.
- Saving a recommendation does not approve construction or prove the site's suitability.

Exact database tables, API envelopes and field schemas will be defined as each capability is implemented.

## 8. Sample-data shell

Start with a deterministic demonstration dataset:

1. A sample location record for the walkthrough, explicitly distinguished from verified site coordinates.
2. A small sample weather dataset with a documented time period and units.
3. Tomato and capsicum sample requirement profiles.
4. Two or three predefined greenhouse designs with different suitability metadata.
5. Illustrative construction and operating costs with stated assumptions.
6. A build package and 3D layout for every selectable design.

The shell should use the same logical inputs and outputs that real providers will use. Matching and data handoffs can be functional even while data is illustrative. Do not describe catalogue matching or fabricated results as a physical greenhouse simulation.

## 9. Completion checks for this workflow

- A user request produces structured crop and location inputs.
- Ambiguous locations request confirmation rather than silently selecting one.
- A climate profile is traceable to its weather data.
- Both design lookups use the same catalogue version.
- Candidate preparation combines climate and crop eligibility correctly.
- No-overlap cases return no suitable predefined design.
- Recommendations use an explicit policy and expose cost assumptions.
- Build package and 3D scene match the selected design version.
- Sample results are clearly labelled throughout the UI.
- Provider failures and incomplete data are surfaced without fabricated replacements.
- The workflow retains enough information to reproduce its result.

## 10. Deferred extensions

- Full greenhouse climate simulation in candidate evaluation.
- Dynamic geometry, equipment placement and design optimisation.
- Climate-signature or surrogate-model research.
- More crops, varieties and cultivation systems.
- Site-adjusted production, profit and expansion modelling.
- Commissioning, sensors, equipment control and daily farm management.
- Contracts, logistics, worker coordination and commercial agents.

These remain part of the wider product direction. They are not prerequisites for completing this initial recommendation workflow.
