### Plan de fases para un Loopscape avanzado

#### Respuesta corta

Para que Loopscape llegue a un nivel avanzado y no se quede como demo visual, recomiendo 10 fases. Con 5 fases puedes lograr un prototipo fuerte. Con 8 fases puedes tener un software serio. Con 10 fases ya se acerca a un laboratorio interactivo avanzado.

#### Fase 0: importacion inicial

Objetivo: crear repositorio, normalizar estructura y dejar `main` como base limpia.

Entregables:

- repositorio Git inicial,
- README minimo,
- licencia,
- `.gitignore`,
- primer tag `v0.0.0` si se desea congelar la base importada.

Rama sugerida:

```text
fase-0-importacion-inicial
```

#### Fase 1: base profesional reproducible

Objetivo: dejar el proyecto validable por cualquier persona.

Entregables:

- `Makefile`,
- scripts de limpieza y validacion,
- CI basico,
- documentacion de arquitectura,
- documentacion de flujo por ramas y patches,
- normalizacion de comentarios y cadenas visibles en espanol.

Rama sugerida:

```text
fase-1-base-profesional
```

Criterio de cierre:

```bash
make validate
```

#### Fase 2: nucleo de simulacion determinista

Objetivo: separar el comportamiento de los loops del renderizado.

Entregables:

- modulo `core` para tareas, tokens, loops y scheduler,
- simulacion por ticks,
- pruebas unitarias del ciclo Think, Act y Observe,
- estado reproducible con semilla fija,
- eventos internos tipados.

Rama sugerida:

```text
fase-2-core-simulacion
```

#### Fase 3: DSL de orquestacion

Objetivo: implementar comandos formales como `/goal`, `/plan`, `/delegate` y `/verify`.

Entregables:

- lexer,
- parser,
- AST,
- validador semantico,
- interprete minimo,
- pruebas de scripts validos e invalidos.

Rama sugerida:

```text
fase-3-dsl-orquestacion
```

#### Fase 4: editor visual de nodos

Objetivo: permitir que el usuario conecte loops, herramientas y rutas de tareas desde la interfaz.

Entregables:

- seleccion de entidades,
- arrastre de nodos,
- conexiones visuales,
- panel de propiedades,
- modo Rayos X,
- guardado de grafo en JSON.

Rama sugerida:

```text
fase-4-editor-visual
```

#### Fase 5: runtime LLM con sandbox

Objetivo: integrar LLMs sin exponer claves ni romper reproducibilidad.

Entregables:

- proveedor simulado por defecto,
- proxy local,
- abstraccion de proveedor,
- limites de tokens,
- timeouts,
- modo offline,
- trazas ReAct persistibles.

Rama sugerida:

```text
fase-5-runtime-llm-sandbox
```

#### Fase 6: orquestacion multiagente y fallos

Objetivo: convertir los loops en infraestructura gestionada.

Entregables:

- supervisores,
- trabajadores,
- especialistas,
- latidos,
- deteccion de loops colgados,
- reinicio de workers,
- fallos bizantinos simulados,
- votacion o consenso simplificado.

Rama sugerida:

```text
fase-6-multiagente-fallos
```

#### Fase 7: replay, metricas y benchmarks

Objetivo: hacer que cada ejecucion pueda analizarse y compararse.

Entregables:

- grabacion de runs,
- replay determinista,
- metricas de throughput, latencia y uso de tokens,
- benchmarks reproducibles,
- exportacion de resultados en JSON y CSV.

Rama sugerida:

```text
fase-7-replay-benchmarks
```

#### Fase 8: seguridad, politicas y CI avanzado

Objetivo: endurecer el proyecto para uso publico y colaborativo.

Entregables:

- CI con `cargo fmt`, `cargo clippy` y tests,
- validacion WASM,
- escaneo de secretos,
- politica de seguridad,
- permisos minimos en GitHub Actions,
- separacion entre CI ligero y build web manual,
- evidencia de validacion.

Rama sugerida:

```text
fase-8-seguridad-ci
```

#### Fase 9: release web y escritorio

Objetivo: entregar versiones instalables o ejecutables.

Entregables:

- build web con Trunk,
- build web estatico sin publicacion automatica en GitHub Pages,
- release con artefactos,
- changelog por version,
- tag `v0.9.0` o `v1.0.0-rc1`,
- instrucciones para usuarios finales.

Rama sugerida:

```text
fase-9-release-web-desktop
```

#### Fase 10: investigacion avanzada

Objetivo: convertir Loopscape en laboratorio de computo cognitivo, no solo en juego.

Entregables:

- escenarios comparables entre eras,
- metricas por paradigma de control,
- experimentos reproducibles,
- scripts de evaluacion,
- informe tecnico,
- tablero de resultados,
- dataset pequeno de tareas y trazas.

Rama sugerida:

```text
fase-10-investigacion-avanzada
```

#### Nivel esperado por cantidad de fases

```text
Fases 0 a 1: base limpia y reproducible.
Fases 0 a 3: prototipo tecnico fuerte.
Fases 0 a 5: demo avanzada con DSL y LLM sandbox.
Fases 0 a 8: software serio con validacion, seguridad y benchmarks.
Fases 0 a 10: laboratorio avanzado con valor academico y tecnico.
```

#### Recomendacion final

No conviene saltar directo a la Era 5. Primero hay que separar nucleo, simulacion, DSL, visualizacion y sandbox LLM. Ese orden evita que el proyecto se vuelva una demo dificil de mantener.

#### Fase 3.1: AST y modelo de comandos

Estado: implementada como microfase inicial del DSL.

Entregables:

```text
src/dsl/mod.rs
src/dsl/ast.rs
src/dsl/command.rs
src/dsl/error.rs
docs/DSL_ORQUESTACION.md
```

Validacion:

```bash
cargo test dsl
make validate-fast
```

#### Fase 3.2: lexer minimo

Estado: implementada como microfase del DSL.

Entregables:

```text
src/dsl/token.rs
src/dsl/lexer.rs
```

El lexer reconoce:

```text
slash commands
identificadores
flechas
numeros
strings simples
saltos de linea
comentarios
```

Validacion:

```bash
cargo test dsl::lexer
make validate-fast
```

#### Fase 3.3: parser minimo

Estado: implementada como microfase del DSL.

Entregables:

```text
src/dsl/parser.rs
examples/rescate.loop
```

El parser convierte tokens en `OrchestrationProgram` y soporta el flujo minimo de orquestacion.

Validacion:

```bash
cargo test dsl::parser
cargo test dsl
make validate-fast
```

#### Fase 3.4: validador semantico del DSL

Objetivo: detectar programas DSL mal formados antes de ejecutarlos.

Entregables:

```text
src/dsl/validator.rs
validacion de un unico /goal
validacion de /plan no vacio
validacion de /delegate con worker
validacion de orden /verify antes de /terminate when verified
pruebas unitarias del validador
```

Cierre:

```bash
cargo test dsl::validator
make validate-fast
```

#### Fase 3.5: interprete del DSL hacia eventos del nucleo

Objetivo: convertir un programa DSL validado en eventos internos del nucleo.

Entregables:

```text
src/dsl/interpreter.rs
CoreEvent::GoalCreated
CoreEvent::PlanStepCreated
CoreEvent::DelegationRequested
CoreEvent::VerificationRequested
CoreEvent::TerminationPolicySet
pruebas unitarias del interprete
```

Cierre:

```bash
cargo test dsl::interpreter
cargo test core
make validate-fast
```

#### Fase 3.6: CLI y ejemplos del DSL

Objetivo: ejecutar scripts `.loop` desde el binario nativo.

Entregables:

```text
examples/rescate.loop
examples/verificacion.loop
argumento --script
soporte para --seed
soporte para --ticks
documentacion de ejecucion CLI
```

Cierre:

```bash
cargo run -- --script examples/rescate.loop --seed 123 --ticks 50
make validate-fast
```

#### Fase 4.1: visor de programas DSL

Objetivo: mostrar el programa DSL cargado en pantalla sin implementar todavia un editor visual completo.

Entregables:

- panel lateral de comandos DSL,
- estado por comando: pendiente, activo, completado o error,
- resaltado del comando actual,
- documentacion del visor visual.

Validacion:

```bash
make validate-fast
cargo run -- --script examples/rescate.loop --seed 123 --ticks 50
```

### Fase 4.2: exportacion de grafo JSON

#### Objetivo

Guardar la estructura logica del flujo DSL como JSON reproducible antes de implementar un editor visual completo.

#### Entregables

```text
graph.json
nodos
aristas
metadatos
ids estables
```

#### Validacion

```bash
cargo run -- --script examples/rescate.loop --export-graph artifacts/rescate.graph.json
test -f artifacts/rescate.graph.json
make validate-fast
```

### Fase 4.3: importacion de grafo JSON

#### Objetivo

Cargar un grafo previamente exportado y validar que su estructura logica conserva nodos, aristas, metadatos e ids estables.

#### Entregables

```text
argumento --graph
importador JSON
validacion de ids
validacion de aristas
pruebas unitarias del grafo
```

#### Validacion

```bash
cargo run -- --script examples/rescate.loop --export-graph artifacts/rescate.graph.json
cargo run -- --graph artifacts/rescate.graph.json --seed 123 --ticks 50
make validate-fast
```

### Fase 4.4: roundtrip y contrato estable del grafo JSON

#### Objetivo

Garantizar que el ciclo DSL, grafo JSON, importacion y serializacion canonica sea estable.

#### Entregables

```text
test de roundtrip
contrato publico del formato
validacion de ids estables
validacion de metadatos
validacion de nodos y aristas
documentacion del contrato JSON
```

#### Validacion

```bash
cargo test dsl::graph
cargo test --test dsl_graph_contract
cargo run -- --script examples/rescate.loop --export-graph artifacts/rescate.graph.json
cargo run -- --graph artifacts/rescate.graph.json --seed 123 --ticks 50
make validate-fast
```

### Fase 5.1: eventos JSONL

#### Objetivo

Registrar eventos deterministas del nucleo en formato JSONL antes de introducir LLM real.

#### Entregables

```text
comando --record
archivo events.jsonl
contrato de linea JSONL
pruebas de roundtrip de traza
validacion de determinismo por seed
```

#### Validacion

```bash
cargo run -- --script examples/rescate.loop --record artifacts/runs/dev/events.jsonl --seed 123 --ticks 50
test -f artifacts/runs/dev/events.jsonl
make validate-fast
```

### Fase 5.2 replay determinista

Objetivo: reproducir una traza JSONL grabada previamente y validar su contrato secuencial.

Entregables:

```text
comando --replay
modulo core::replay
validacion de ticks monotonos
pruebas de replay determinista
documentacion headless
```

Criterio de aceptacion:

```bash
cargo test replay
make validate-fast
```

### Fase 5.3: metricas CSV

#### Objetivo

Exportar metricas comparables de simulacion en CSV antes de introducir LLM real.

#### Entregables

```text
comando --metrics
archivo metrics.csv
columnas estables
pruebas de determinismo de metricas
documentacion headless
```

#### Validacion

```bash
cargo run -- --script examples/rescate.loop --metrics artifacts/runs/dev/metrics.csv --seed 123 --ticks 50
test -f artifacts/runs/dev/metrics.csv
make validate-fast
```

### Fase 5.4: comparacion de corridas

#### Objetivo

Comparar corridas mediante archivos `metrics.csv` para detectar diferencias reproducibles entre escenarios, seeds o configuraciones.

#### Entregables

```text
comando --compare-metrics
reporte comparison.csv
pruebas de comparacion
contrato de columnas de comparacion
documentacion headless
```

#### Validacion

```bash
cargo test compare
cargo test --test metrics_comparison
make validate-fast
```

### Fase 6.1: provider LLM simulado

#### Objetivo

Introducir una interfaz LLM segura sin red ni claves.

#### Entregables

```text
MockProvider
ReplayProvider
contrato LlmProvider
pruebas de sandbox
```

#### Validación

```bash
cargo test llm
cargo test --test llm_mock_provider
make validate-fast
```

### Fase 6.2: limites de tokens y timeouts simulados

#### Objetivo

Agregar limites simulados al runtime LLM sandbox antes de permitir providers reales.

#### Entregables

```text
LlmLimits
max_prompt_tokens
max_response_tokens
timeout_ticks
pruebas de limites
```

#### Validación

```bash
cargo test llm
cargo test --test llm_limits
make validate-fast
```

### Fase 6.3: proxy HTTP opcional

#### Objetivo

Preparar un proxy HTTP opcional detras de feature flag antes de conectar providers reales.

#### Entregables

```text
feature llm-proxy
contrato de proxy HTTP
limite de body
timeout obligatorio
CORS restringido
sin claves hardcodeadas
```

### Fase 7.1: supervisor multiagente real

#### Objetivo

Agregar un supervisor determinista que mantenga estado real de workers, heartbeats, timeouts y politica de reinicio.

#### Entregables

```text
SupervisorState
WorkerState
HeartbeatEvent
WorkerTimeout
RestartPolicy
```

#### Validacion

```bash
cargo test supervisor
cargo test --test supervisor_multiagente
make validate-fast
```

### Fase 7.2: fallos recuperables de workers

#### Objetivo

Integrar el supervisor al runtime de simulacion para simular workers colgados, timeouts y recuperacion controlada.

#### Entregables

```text
RecoverableFailurePlan
WorkerFailureSpec
SupervisorState dentro de SimulationState
eventos WorkerTimedOut y WorkerRestarted
metricas CSV con fallos detectados y recuperados
```

#### Validacion

```bash
cargo test failure
cargo test supervisor
cargo test --test fallos_recuperables
make validate-fast
```

### Fase 7.3: fallo bizantino simplificado

#### Objetivo

Modelar respuestas falsas de workers y aplicar una votacion simple para decidir si un resultado se acepta o se rechaza.

#### Entregables

```text
ByzantineFailurePlan
WorkerResponse
SimpleMajorityVoter
VotingOutcome
pruebas de mayoria
pruebas de empate
pruebas de respuestas falsas
```

#### Validacion

```bash
cargo test byzantine
cargo test --test fallo_bizantino
make validate-fast
```

### Fase 7.4: fallos por CLI y DSL

#### Objetivo

Exponer fallos recuperables y fallos bizantinos desde CLI y DSL, usando las APIs creadas en las fases 7.1, 7.2 y 7.3.

#### Entregables

```text
--worker-failure
--supervisor-timeout
--worker-restart-limit
--byzantine-failure
--byzantine-vote
/worker-failure
/byzantine-failure
/byzantine-vote
```

#### Validacion

```bash
cargo test failure
cargo test byzantine
cargo test dsl
cargo test --test fallos_cli_dsl
make validate-fast
```

### Fase 7.5: endurece validacion multiagente

#### Objetivo

Cerrar la parte 7 con un gate reproducible para escenarios adversariales multiagente.

#### Entregables

- `make validate-multiagent`,
- script `scripts/validate_multiagent.sh`,
- pruebas de CLI multiagente,
- ejemplos `.loop` de fallos recuperables y fallo bizantino,
- documentacion de validacion multiagente,
- limpieza de artefactos de validacion.

#### Resultado esperado

La Fase 8 puede partir de una base con supervisor, recuperacion, votacion simple, DSL, CLI y validacion integrada.

### Fase 8.1: auditoría ligera de workflows

#### Objetivo

Endurecer GitHub Actions sin volver el CI pesado.

#### Cambios

```text
permissions mínimos
no deploy automático
no secrets en PR
actions actualizadas
FORCE_JAVASCRIPT_ACTIONS_TO_NODE24=true
```

#### Resultado

El proyecto queda preparado para reportes y artefactos de Fase 8 sin abrir permisos de escritura ni flujos de despliegue prematuros.


### Fase 8.2: auditoria Rust manual

#### Objetivo

Agregar auditoria Rust de dependencias sin ralentizar cada PR.

#### Entregables

```text
.github/workflows/rust-security.yml
docs/AUDITORIA_RUST.md
make audit-rust
make audit-rust-audit
make audit-rust-deny
```

#### Criterio de aceptacion

```text
la auditoria Rust no corre en pull_request
la auditoria Rust no corre en push
la auditoria Rust puede ejecutarse manualmente
la auditoria Rust corre semanalmente
los permisos del workflow son de solo lectura
no se usan secretos
no hay deploy automatico
```

#### Siguiente fase

La siguiente fase recomendada es Fase 8.3, validacion profunda manual.

### Fase 8.3: escaneo manual de secretos

#### Objetivo

Detectar secretos versionados sin bloquear todos los PR pequenos.

#### Cambios

```text
.github/workflows/secrets-scan.yml
docs/ESCANEO_SECRETOS.md
```

#### Criterio de aceptacion

```text
El escaneo se ejecuta manualmente o por schedule semanal.
No se ejecuta por pull request.
No se ejecuta por push.
No usa secretos.
No hace deploy.
No usa permisos de escritura.
```

#### Siguiente fase

La siguiente fase recomendada es validacion profunda manual para pruebas funcionales mas completas.

### Fase 8.4: validacion profunda manual

#### Objetivo

Agregar una puerta manual para pruebas funcionales pesadas sin ejecutarlas en cada PR.

#### Cambios

```text
.github/workflows/deep-validation.yml
docs/VALIDACION_PROFUNDA_MANUAL.md
```

#### Criterio de aceptacion

```text
El workflow se ejecuta por workflow_dispatch.
No se ejecuta por pull request.
No se ejecuta por push.
No usa secretos.
No hace deploy.
Ejecuta make validate-fast.
Ejecuta make validate-multiagent.
Valida escenarios DSL, metricas y replay.
```

#### Siguiente fase

La siguiente fase recomendada es reportes de evidencia para hacer legibles los resultados de validacion.

### Fase 8.5: reportes de evidencia

#### Objetivo

Convertir resultados de validacion, auditoria Rust, escaneo de secretos, metricas y replay en reportes Markdown y JSON.

#### Cambios

```text
scripts/generate_evidence_report.py
.github/workflows/evidence-report.yml
docs/REPORTES_EVIDENCIA.md
```

#### Criterio de aceptacion

```text
El reporte se genera manualmente.
No ejecuta pruebas pesadas.
No se ejecuta por pull request.
No se ejecuta por push.
Produce reporte-evidencia.md.
Produce reporte-evidencia.json.
Resume metricas, replay, auditoria Rust y escaneo de secretos cuando existen.
```

#### Siguiente fase

La siguiente fase recomendada es preparar release experimental con versionado, changelog y checklist de publicacion.

### Fase 9: release controlado

#### Fase 9.1: version y changelog

Objetivo: preparar el versionado inicial de releases sin publicar web automaticamente.

Entregables:

```text
VERSION
CHANGELOG.md
docs/RELEASE.md
```

Criterio de aceptacion:

```text
la version de desarrollo queda declarada
el changelog incluye el estado 0.9.0-dev
el documento de release explica que el tag rc se crea solo desde main
```

#### Fase 9.2: artefacto web manual

Objetivo: generar artefacto web manual como artifact, no como Pages.

#### Fase 9.3: release candidate

Objetivo: cerrar `v0.9.0-rc1` desde `main` despues de validar y mergear el PR correspondiente.

### Fase 9.4: revision posterior al release candidate

#### Objetivo

Registrar la evaluacion posterior a `v0.9.0-rc1`.

#### Entregables

- `docs/REVISION_RELEASE_CANDIDATE.md`.
- Actualizacion de documentacion de release.
- Criterios para avanzar a `v0.9.0` estable.
- Criterios para crear `v0.9.0-rc2`.

#### Fuera de alcance

- Crear tag nuevo.
- Publicar GitHub Release.
- Publicar web automaticamente.
- Cambiar funcionalidad del nucleo.

### Fase 10.1: escenarios comparables

#### Objetivo

Agregar escenarios DSL comparables para pruebas, benchmarks e informe tecnico.

#### Entregables

- `scenarios/react_basic.loop`.
- `scenarios/dsl_delegation.loop`.
- `scenarios/multiagent_failure.loop`.
- `docs/ESCENARIOS_COMPARABLES.md`.
- `tests/escenarios_comparables.rs`.

#### Regla

Los escenarios deben ser pequeños, reproducibles y no deben depender de red, secretos ni servicios externos.

### Fase 10.2: benchmarks reproducibles

#### Objetivo

Agregar benchmarks manuales y reproducibles sobre los escenarios comparables de la Fase 10.1.

#### Entregables

```text
benchmarks/
scripts/run_benchmarks.sh
artifacts/benchmarks/.gitkeep
docs/BENCHMARKS.md
```

#### Criterio de aceptacion

Los benchmarks deben generar CSV y resumen Markdown sin incorporarse al CI automatico de PR pequeños.

### Fase 10.3: informe tecnico interno

#### Objetivo

Consolidar arquitectura, alcance, validacion, escenarios comparables, benchmarks reproducibles, riesgos y criterios de cierre experimental.

#### Entregables

```text
docs/INFORME_TECNICO.md
docs/RESULTADOS.md
```

#### Criterio de aceptacion

El informe no debe inventar metricas. Los resultados numericos deben provenir de una corrida local de benchmarks.

### Fase 10.4: badges e indice final

#### Objetivo

Mejorar la entrada publica del repositorio con badges, graficos e indice final sin modificar el nucleo de simulacion.

#### Entregables

- `README.md` actualizado.
- `docs/INDICE_FINAL.md`.
- `docs/assets/loopscape-flujo.svg`.
- `docs/assets/loopscape-validacion.svg`.

### Fase 10.4.1: correccion de badges y demo HF Spaces

#### Objetivo

Corregir badges que dependen de informacion no disponible en repositorios privados o sin release publico, evitar imagenes rotas en el README y documentar el camino para publicar una demo web en Hugging Face Spaces.

#### Alcance

Esta fase no modifica el nucleo de simulacion. Solo ajusta documentacion, badges seguros y recursos visuales de presentacion.

### Fase 10.4.2: publicacion de demo HF Spaces

#### Objetivo

Actualizar la documentacion publica despues de publicar y probar la demo web en Hugging Face Spaces.

#### Alcance

Esta fase no modifica el nucleo de simulacion. Solo actualiza el README, la documentacion de la demo y el indice final con el enlace publico probado.

#### Enlace

```text
https://huggingface.co/spaces/kapumota/loopscape
```

### Fase 10.5, guia de uso y demo guiada

#### Objetivo

Hacer que Loopscape sea entendible para usuarios externos antes del cierre experimental `v0.9.0`.

#### Entregables

- Guia de uso.
- Demo guiada.
- Lectura rapida.
- README con instrucciones de uso mas detalladas.
