#!/bin/bash
set -e

(
    cd vc-data-model-2.0-test-suite
    cp -f ../localConfig.cjs .
    npm test
    rm -f localConfig.cjs
)
