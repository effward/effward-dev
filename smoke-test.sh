#!/bin/bash

url="http://localhost:8080/hey"
expected="Hey there!"

echo "Sleeping for 5 seconds..."
sleep 5

echo "Making smoke test request..."
response=$(curl -s "$url")

if [[ "$response" == *"$expected"* ]]; then
  echo "Success: Response contains '$expected'"
  exit 0
else
  echo "Error: Response does not contain '$expected'"
  exit 1
fi