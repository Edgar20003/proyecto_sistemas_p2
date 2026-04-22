#!/bin/bash
# Escenario 1: Latencia (Retraso)
INTERFACE="lo"
DELAY="200ms"
JITTER="20ms"

echo "⚠️ Aplicando $DELAY (+/- $JITTER) de latencia en la interfaz $INTERFACE..."

# Limpiar reglas previas
sudo tc qdisc del dev $INTERFACE root 2>/dev/null

# Aplicar nueva regla
sudo tc qdisc add dev $INTERFACE root netem delay $DELAY $JITTER

echo "✅ Hecho. Los paquetes ahora tardarán más en llegar."
echo "Prueba correr el pipeline y verás que los mensajes tardan en aparecer."
