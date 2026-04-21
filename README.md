# Proyecto 2: Pipeline Distribuido IoT/Edge con Tolerancia a Fallos

Este proyecto implementa un sistema robusto en **Rust** para el procesamiento de datos en tiempo real, diseñado para operar sobre una infraestructura de **Docker** y una red **VPN (WireGuard)** bajo condiciones de red degradadas.

## 🚀 Estructura del Proyecto
- **/rust**: Código fuente del Sensor, Nodo Edge y Coordinador.
- **/docker**: Configuraciones de contenedores para cada rol.
- **/netem**: Scripts para simulación de escenarios de red (latencia, pérdida, etc.).
- **/vpn**: Configuraciones de infraestructura de red segura.

## 🛠️ Requisitos
- Docker y Docker Compose
- Rust (Cargo)
- WireGuard
- Herramientas de red (`iproute2`, `iperf3`)

## 📈 Escenarios de Red (Simulación)
El sistema está diseñado para ser evaluado bajo:
1. Latencia alta
2. Pérdida de paquetes
3. Corrupción de datos
4. Reordenamiento de paquetes
5. Escenario mixto

## 👷 Autor
- Edgar Arreola (Solo Project)
