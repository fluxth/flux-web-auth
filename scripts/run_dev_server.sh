#!/bin/bash

SCRIPT_PATH=$(dirname $0)

export ROCKET_JWT_PUBLIC_KEY=$(cat $SCRIPT_PATH/../temp/public_key.pem)
export ROCKET_JWT_PRIVATE_KEY=$(cat $SCRIPT_PATH/../temp/private_key.pem)

cargo watch -x run
