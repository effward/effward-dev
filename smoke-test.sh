#!/bin/bash

url="http://localhost:8080/health"
expected="healthy"

echo "Sleeping for 20 seconds..."
sleep 20

echo "Making smoke test request..."
response=$(curl -s "$url")

if [[ "$response" == *"$expected"* ]]; then
  echo "Success: Response contains '$expected'"
  exit 0
else
  echo "Error: Response does not contain '$expected'"
  exit 1
fi