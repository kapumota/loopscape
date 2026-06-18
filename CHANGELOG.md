### 0.9.0

Fecha: 2026-06-17

#### Cierre experimental

- Promueve el proyecto desde `0.9.0-rc1` hacia `0.9.0` como cierre experimental.
- Mantiene el alcance como plataforma experimental reproducible, no como producto industrial.
- Conserva la demo web publicada en Hugging Face Spaces.
- Conserva escenarios comparables, benchmarks reproducibles, informe tecnico y resultados preliminares.
- Agrega documentacion de cierre experimental y roadmap posterior a `v0.9.0`.
- Deja preparado el tag `v0.9.0` para crearse solo desde `main` despues del merge.

### 0.9.0-rc1

#### Estado

Release candidate experimental de Loopscape.

Esta version cierra el ciclo inicial de simulacion determinista, DSL, supervisor multiagente, fallos recuperables, fallo bizantino simplificado, validacion manual profunda, auditoria Rust, escaneo manual de secretos, artefacto web manual y reportes de evidencia.

#### Cambios incluidos

- Consolida el versionado de release candidate.
- Mantiene publicacion web manual como artefacto, sin GitHub Pages automatico.
- Mantiene validacion profunda como workflow manual.
- Mantiene auditoria Rust manual y programada.
- Mantiene escaneo de secretos manual y programado.
- Mantiene reportes de evidencia en Markdown y JSON.

#### Validacion esperada antes del tag

- `make validate-full`
- `make validate-web`
- `make evidence-report-release`
- Auditoria Rust manual desde GitHub Actions.
- Escaneo manual de secretos desde GitHub Actions.

#### Nota de publicacion

El tag `v0.9.0-rc1` debe crearse solo despues de mergear esta fase en `main`.

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
