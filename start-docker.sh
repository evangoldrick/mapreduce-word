#!/bin/bash

./run-command-in-each-backend.sh "cargo build"

docker compose up -d
