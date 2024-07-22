#!/bin/sh

pushd vc-api-issuer-test-suite
npm i
cp -f ../.localImplementationsConfig.cjs .
popd
