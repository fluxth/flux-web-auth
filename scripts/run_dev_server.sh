#!/bin/bash

SCRIPT_PATH=$(dirname $0)

export ROCKET_SITE_HOST=localhost:8000
export ROCKET_AUTHTOKEN_COOKIE_NAME=authtoken
export ROCKET_AUTHTOKEN_COOKIE_DOMAIN=localhost
export ROCKET_JWT_PUBLIC_KEY=$(cat $SCRIPT_PATH/../temp/public_key.pem)
export ROCKET_JWT_PRIVATE_KEY=$(cat $SCRIPT_PATH/../temp/private_key.pem)
export ROCKET_ALLOWED_NEXT_HOSTS='["auth.flux.ci",  "search.flux.ci", "localhost"]'

cargo watch -x run
