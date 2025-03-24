#!/bin/bash

openssl ecparam -genkey -name secp256r1 | openssl ec -out cav3.key
openssl req -new -subj "/CN=ECC-secp256r1" -x509 -days 3650 -extensions v3_ca -reqexts v3_req -config <(cat /etc/ssl/openssl.cnf <(printf "\n[v3_req]\nbasicConstraints=critical,CA:TRUE\nkeyUsage=nonRepudiation,digitalSignature,keyEncipherment\nsubjectAltName=DNS:localhost")) -key cav3.key -out cav3.pem
# Create Truststore adding a CA
keytool -keystore truststore.p12 -storepass password -alias CARoot -import -file cav3.pem

openssl ecparam -genkey -name secp256r1 | openssl ec -out serverv3.key
openssl req -new -subj "/CN=server-ECC-secp256r1" -addext "subjectAltName = DNS:localhost" -key serverv3.key -out serverv3.csr
openssl x509 -req -days 3650 -extfile <(printf "\nbasicConstraints=CA:FALSE\nkeyUsage=nonRepudiation,digitalSignature,keyEncipherment\nsubjectAltName=DNS:localhost") -in serverv3.csr -CA cav3.pem -CAkey cav3.key -set_serial 01 -out serverv3.pem

openssl ecparam -genkey -name secp256r1 | openssl ec -out clientv3.key
openssl req -new -subj "/CN=client01-ECC-secp256r1" -addext "subjectAltName = DNS:localhost" -key clientv3.key -out clientv3.csr
openssl x509 -req -days 3650 -extfile <(printf "\nbasicConstraints=CA:FALSE\nkeyUsage=nonRepudiation,digitalSignature,keyEncipherment\nsubjectAltName=DNS:localhost") -in clientv3.csr -CA cav3.pem -CAkey cav3.key -set_serial 02 -out clientv3.pem

