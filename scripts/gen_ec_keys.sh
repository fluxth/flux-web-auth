#!/bin/bash

PRIVATE_KEY_FILE=private_key
PUBLIC_KEY_FILE=public_key

openssl ecparam -genkey -name prime256v1 -noout -out ${PRIVATE_KEY_FILE}.pem
openssl ec -in ${PRIVATE_KEY_FILE}.pem -pubout > ${PUBLIC_KEY_FILE}.pem

openssl pkcs8 -nocrypt -topk8 -in ${PRIVATE_KEY_FILE}.pem -out ${PRIVATE_KEY_FILE}.pkcs8.pem
