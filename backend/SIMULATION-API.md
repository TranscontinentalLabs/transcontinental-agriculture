
curl -sS -X POST \
"http://localhost:8000/simulation/get-greenhouse-configuration" \
-H "Content-Type: application/json" \
-d '{}' | jq

