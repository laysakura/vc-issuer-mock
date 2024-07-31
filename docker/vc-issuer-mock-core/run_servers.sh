#!/bin/bash

nginx &

# runs on port 3000
/usr/local/bin/didkit-http &

# Run the vc-issuer-mock-core server
## Read signing keys from files the Render platform places.
for secret in ISSMOCK_PRIV_EC_P384 ISSMOCK_PRIV_OKP_ED25519; do
    file="/etc/secrets/$secret"
    if [ -r "$file" ]; then
        export $secret=$(cat "$file")
        echo "$secret has been set from $file"
    fi
done
## runs on port 40080
/usr/local/bin/vc-issuer-mock-core &

wait # until all backgroup processes are finished (infinite)
