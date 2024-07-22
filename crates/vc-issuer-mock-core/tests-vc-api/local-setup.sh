#!/bin/sh
set -e

pushd vc-data-model-2.0-test-suite
npm i
cp -f ../localConfig.cjs .
popd
