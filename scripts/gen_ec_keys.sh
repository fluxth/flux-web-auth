#!/bin/bash

PRIVATE_KEY_FILE=private_key.pem
PUBLIC_KEY_FILE=public_key.pem

openssl ecparam -genkey -name secp384r1 -noout -out $PRIVATE_KEY_FILE
openssl ec -in $PRIVATE_KEY_FILE -pubout > $PUBLIC_KEY_FILE
