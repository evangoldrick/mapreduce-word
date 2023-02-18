#!/bin/bash

function runCommand() {
    local originalDir=$(pwd)
    cd $1 &&
    $2

    cd $originalDir
}

runCommand backend/controller "$1"
runCommand backend/map "$1"
runCommand backend/reduce "$1"
