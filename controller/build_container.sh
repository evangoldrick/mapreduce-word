#!/bin/bash

# Stop old running process
docker conatiner stop rocketWebTest
docker container rm rocketWebTest
docker image rm rocket-web-test

# Create new container
docker build -t rocket-web-test .
docker run --name rocketWebTest -it -p 8080:8080 rocket-web-test
