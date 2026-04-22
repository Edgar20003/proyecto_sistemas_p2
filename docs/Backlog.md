# Backlog del Proyecto - Pipeline Distribuido

## Product Backlog (Historias de Usuario)
1. **HU01:** Como sistema, debo transmitir datos de sensores vía JSON para centralizar información. (Prioridad: Alta)
2. **HU02:** Como administrador, debo simular fallas de red para validar la resiliencia. (Prioridad: Alta)
3. **HU03:** Como sistema, debo garantizar la entrega de datos ante caídas de la VPN. (Prioridad: Muy Alta)
4. **HU04:** Como analista, debo visualizar métricas de latencia para optimizar el borde (Edge). (Prioridad: Media)

## Sprint 1 Backlog (Finaliza el 30 de Abril)
* [x] Configuración de repositorio y estructura de carpetas.
* [x] Prototipo funcional de comunicación en Rust.
* [x] Script inicial de simulación de latencia con `tc netem`.
* [ ] Creación de contenedores Docker iniciales (Pendiente).
* [ ] Documentación técnica de arquitectura (Diagrama).
