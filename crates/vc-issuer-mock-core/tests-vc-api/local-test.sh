#!/bin/sh
set -e

pushd vc-data-model-2.0-test-suite
cp -f ../localConfig.cjs .
npm test
rm -f localConfig.cjs
popd
