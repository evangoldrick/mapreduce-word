#!/bin/bash

./run-command-in-each-backend.sh "cargo build --release"

docker compose build
docker compose up
