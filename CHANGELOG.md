### Changelog

#### 0.9.0-dev

Estado: desarrollo hacia `v0.9.0-rc1`.

Cambios preparados:

- Cierre de Fase 8 con CI liviano y seguro.
- Auditoria Rust manual y programada.
- Escaneo manual de secretos.
- Validacion profunda manual.
- Reportes de evidencia en Markdown y JSON.
- Preparacion inicial de release controlado sin despliegue automatico.

Notas de release:

- Esta version no publica GitHub Pages automaticamente.
- El tag de release candidate se debe crear solo desde `main` despues del merge de la fase correspondiente.
- Los artefactos pesados se generan por workflows manuales.

#### v0.1.0

- Base inicial de Loopscape con cinco eras visuales.
- Integracion preliminar con LLM y proxy web.
- Soporte de ejecucion nativa y WebAssembly.

#### Fase 1 propuesta

- Agrega comandos reproducibles de validacion y limpieza.
- Agrega documentacion tecnica de arquitectura y flujo por ramas.
- Agrega CI para formato, compilacion, pruebas y build web.
- Agrega pruebas unitarias iniciales para el parser ReAct.
