# Minuta de Reunión #01 - Sprint 1
**Fecha:** 21 de Abril, 2026 | **Hora:** 14:00 - 15:30
**Participantes:** Edgar Arreola (RT/Dev)

## Temas Discutidos
1. Definición del contrato de datos JSON entre Sensor, Edge y Coordinador.
2. Selección del framework `axum` para la comunicación asíncrona en Rust.
3. Configuración inicial del entorno en VMware con Ubuntu Server.

## Decisiones Tomadas
* Se utilizará un Workspace de Cargo para gestionar los tres microservicios en un solo repositorio.
* La comunicación inicial se realizará vía HTTP para validar el pipeline antes de integrar la VPN.

## Tareas Generadas
* [x] Inicializar repositorio en GitHub.
* [x] Implementar prototipo funcional de Sensor y Coordinador.
* [ ] Configurar túnel WireGuard (Pendiente).

![Imagen de transmicion de datos en terminales](D:\Documentos\Escuela 2026A\Sistemas operativos\Evidencias\Captura de pantalla 2026-04-21 152243.png)
