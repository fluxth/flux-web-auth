#!/bin/bash

BASEPATH=$(dirname "$0")
TEMPPATH="${BASEPATH}/../temp"

export ALLOWED_URLS=http://localhost:8080
export JWT_PRIVATE_KEY=$(cat "${TEMPPATH}/private_key.pem")
export JWT_PUBLIC_KEY=$(cat "${TEMPPATH}/public_key.pem")

cargo watch -x run
