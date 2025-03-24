#!/bin/bash

openssl genrsa -out cav3.key 2048
openssl req -new -subj "/CN=RSA-2048" -x509 -sha256 -days 3650 -extensions v3_req -config <(cat /etc/ssl/openssl.cnf <(printf "\n[v3_req]\nbasicConstraints=critical,CA:TRUE\nkeyUsage=nonRepudiation,digitalSignature,keyEncipherment\nsubjectAltName=DNS:localhost")) -key cav3.key -out cav3.pem

openssl genrsa -out serverv3.key 2048
openssl req -new -subj "/CN=server-RSA-2048" -key serverv3.key -out serverv3.csr
openssl x509 -req -in serverv3.csr -CA cav3.pem -CAkey cav3.key -CAcreateserial -out serverv3.pem -sha256 -days 3650 -extfile v3.ext

openssl genrsa -out clientv3.key 2048
openssl req -new -subj "/CN=client-RSA-2048" -key clientv3.key -out clientv3.csr
openssl x509 -req -in clientv3.csr -CA cav3.pem -CAkey cav3.key -CAcreateserial -out clientv3.pem -sha256 -days 3650 -extfile v3.ext
