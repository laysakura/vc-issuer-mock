#!/bin/bash

max_attempts=10
attempt=1

while [ $attempt -le $max_attempts ]
do
  response=$(curl -s -o /dev/null -w "%{http_code}" http://localhost:8000/health)
  if [ $response = "200" ]; then
    echo "Health check passed on attempt $attempt"
    exit 0
  else
    echo "Health check failed with status code: $response (Attempt $attempt/$max_attempts)"
    if [ $attempt -eq $max_attempts ]; then
      echo "Health check failed after $max_attempts attempts"
      exit 1
    fi
    attempt=$((attempt+1))
    sleep 1
  fi
done
