#!/bin/bash

# runs on port 3000
/usr/local/bin/didkit-http &

# runs on port 40080
/usr/local/bin/vc-issuer-mock-core &

wait # until all backgroup processes are finished (infinite)
