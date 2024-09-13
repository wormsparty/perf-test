#!/bin/bash

cat << EOT > ./config.yml
server:
  host: ""
  port: 8000
  cors_origins:
    - "http://localhost:3000"
  mode: "release"

database:
  address: "$DB_HOST:$DB_PORT"
  user: "$DB_USER"
  pass: "$DB_PASSWORD"
  name: "$DB_NAME"
EOT

echo "Running..."
./main
