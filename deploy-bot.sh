#!/usr/bin/env bash

clear

echo "Deleting old 'Secrets.toml'..."
rm ./Secrets.toml

echo "Creating new 'Secrets.toml'..."
cp ./secrets/deploy/Secrets.toml ./Secrets.toml

echo "[WARNING]: Resetting a server will destory all persisted data. Do not use lightly!"
echo "Reset Server Before Deployment? (y/n): "
read should_restart_server

if [ $should_restart_server == "y" ]
then
    echo "Restarting server..."
    cargo shuttle project restart --idle-minutes 0
fi

cargo shuttle deploy