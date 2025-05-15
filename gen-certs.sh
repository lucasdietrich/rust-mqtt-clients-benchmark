#!/bin/bash
set -e

# === Configurable variables ===
BROKER_CN="localhost"
CLIENT_CN="client"
DAYS_VALID=365

# === Prepare directories ===
mkdir -p certs
cd certs

# === 1. Generate Broker CA ===
openssl genrsa -out ca.key.pem 2048
openssl req -x509 -new -nodes -key ca.key.pem -sha256 -days $DAYS_VALID -out ca.crt.pem -subj "/CN=MQTT Test CA"

# === 2. Generate Broker Certificate (server) ===
openssl genrsa -out broker.key.pem 2048
openssl req -new -key broker.key.pem -out broker.csr.pem -subj "/CN=$BROKER_CN"
openssl x509 -req -in broker.csr.pem -CA ca.crt.pem -CAkey ca.key.pem -CAcreateserial -out broker.crt.pem -days $DAYS_VALID -sha256

# === 3. Generate Client Certificate ===
openssl genrsa -out client.key.pem 2048
openssl req -new -key client.key.pem -out client.csr.pem -subj "/CN=$CLIENT_CN"
openssl x509 -req -in client.csr.pem -CA ca.crt.pem -CAkey ca.key.pem -CAcreateserial -out client.crt.pem -days $DAYS_VALID -sha256

# === 4. Combine for Mosquitto & Paho ===
cat client.crt.pem client.key.pem > client.pem  # Used by Paho MQTT
cat broker.crt.pem broker.key.pem > server.pem  # Optional for brokers that require a combined file

echo "Certificates generated in ./certs/"
