#!/bin/sh

pushd vc-api-issuer-test-suite
cp -f tests/10-issuer.js tests/10-issuer.js.bak
perl -pi -e "s/^const tag = 'vc-api';/const tag = 'localhost';/" tests/10-issuer.js
npm test
mv -f tests/10-issuer.js.bak tests/10-issuer.js
popd
