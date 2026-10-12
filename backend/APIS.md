**Health**
curl -X GET "http://localhost:8000/health" \
| jq  

curl -X POST "http://localhost:8000/messaging/user-message" \
-H "Content-Type: application/json" \
-d '{
"user_message": "Calculate vdp when temp is 20.0 and relative humidity is 70%"
}' | jq 

curl -X POST "http://localhost:8000/tools/start-simulation" \
-H "Content-Type: application/json" \
| jq  

curl -X POST "http://localhost:8000/tools/calculate-vdp" \
-H "Content-Type: application/json" \
-d '{
"air_temperature": 28.0,
"relative_humidity": 70.0
}' | jq