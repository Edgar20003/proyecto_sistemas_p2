# Proyecto 2: Pipeline Distribuido IoT/Edge con Tolerancia a Fallos

Este proyecto implementa un sistema robusto en **Rust** para el procesamiento de telemetría IoT, diseñado para operar bajo condiciones de red degradada.

## 📐 Arquitectura del Sistema
![Diagrama de Arquitectura](docs/evidencias/diagrama_arquitectura.png)

## 🚀 Estructura del Proyecto
- **/rust**: Microservicios de Sensor, Nodo Edge y Coordinador.
- **/docker**: Dockerfiles e infraestructura de contenedores.
- **/netem**: Scripts de simulación de red (`tc netem`).
- **/docs**: Plan de ejecución, Riesgos técnicos y Minutas de Scrum.
- **/vpn**: Plantillas de configuración para WireGuard.

## 🛠️ Cómo ejecutar el Prototipo (Avance 30 Abril)
Para probar el flujo actual sin Docker, sigue estos pasos en tres terminales:

1. **Coordinador:** `cd rust && cargo run -p coordinator`
2. **Edge Node:** `cd rust && cargo run -p edge`
3. **Sensor:** `cd rust && cargo run -p sensor`

## 📡 Simulación de Red
Para aplicar la latencia de 200ms validada en el avance:
sudo ./netem/latencia.sh

Para limpiar las reglas de red:
sudo ./netem/limpiar.sh

👷 Autor
Edgar Arreola - Programación de Sistemas Avanzados (Solo Project)



