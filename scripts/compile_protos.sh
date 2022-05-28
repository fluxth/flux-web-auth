#!/bin/bash

# Usage: ./compile_protos.sh {PROTO_DIR_IN} {PROTO_DIR_OUT}
# Relative to this script file

IN_DIR=$1
OUT_DIR=$2

protoc \
    -I=${IN_DIR} \
    ${IN_DIR}/* \
    --js_out=import_style=commonjs:${OUT_DIR} \
    --grpc-web_out=import_style=commonjs+dts,mode=grpcweb:${OUT_DIR}
