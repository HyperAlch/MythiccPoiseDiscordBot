#!/usr/bin/env bash

clear

echo "Deleting old 'Secrets.toml'..."
rm ./Secrets.toml

echo "Creating new 'Secrets.toml'..."
cp ./secrets/test/Secrets.toml ./Secrets.toml

echo "Running local bot..."
cargo shuttle run