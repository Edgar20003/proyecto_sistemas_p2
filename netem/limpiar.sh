#!/bin/bash
sudo tc qdisc del dev lo root 2>/dev/null
echo "✅ Red normalizada. Latencia eliminada."
