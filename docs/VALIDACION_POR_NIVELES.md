### Validacion por niveles

#### Objetivo

Esta fase separa la validacion de Loopscape en niveles para reducir el costo de los Pull Requests normales y reservar las comprobaciones pesadas para ejecuciones importantes o manuales.

#### Niveles

```text
validate-fast: estilo y formato
validate: validacion rapida, compilacion nativa y pruebas nativas
validate-web: build WebAssembly manual
validate-full: validacion media, build web y Clippy estricto
```

#### Uso recomendado

Para cambios pequeños de documentacion, workflows o ajustes no criticos:

```bash
make validate-fast
```

Para cambios de codigo Rust o cambios que afectan el nucleo:

```bash
make validate
```

Para revisar el build de navegador:

```bash
make validate-web
```

Para cierres de fase, release candidates o cambios de alto impacto:

```bash
make validate-full
```

#### Criterio de aceptacion

La fase queda cerrada si existen los targets `validate-fast`, `validate`, `validate-web` y `validate-full`, si los scripts correspondientes estan separados y si el workflow principal puede usar validacion rapida para Pull Requests normales.

#### Pruebas rapidas del nucleo

Para cambios que afectan `src/core`, se agregan dos comandos focalizados:

```bash
make test-core
make test-deterministic
```

Estos comandos permiten revisar el nucleo determinista sin ejecutar el build WebAssembly ni abrir la aplicacion visual.

#### Smoke nativo

Para revisar que el binario nativo arranca sin abrir un flujo largo se agrega el comando:

```bash
make smoke-native
```

El comando ejecuta una corrida corta del nucleo determinista:

```bash
cargo run -- --smoke --seed 123 --ticks 10
```

Este paso no reemplaza las pruebas del nucleo. Sirve como comprobacion rapida de arranque del binario nativo.

#### Pruebas rapidas del DSL

Para cambios que afectan `src/dsl`, se puede ejecutar una validacion focalizada sin levantar Bevy ni compilar WebAssembly:

```bash
cargo test dsl
```

Este comando valida el AST y el modelo de comandos del lenguaje de orquestacion.

#### Pruebas rapidas del lexer DSL

Para cambios del lexer se puede ejecutar una prueba focalizada:

```bash
cargo test dsl::lexer
```

Esta validacion no levanta Bevy ni compila WebAssembly.

#### Pruebas rapidas del parser DSL

Para cambios del parser se puede ejecutar una prueba focalizada:

```bash
cargo test dsl::parser
```

Para validar todo el DSL sin levantar Bevy ni compilar WebAssembly:

```bash
cargo test dsl
```

#### Validacion semantica del DSL

La fase 3.4 agrega pruebas rapidas del validador semantico del DSL. Para cambios en reglas de orquestacion se debe ejecutar:

```bash
cargo test dsl::validator
make validate-fast
```

Esta validacion no levanta Bevy ni compila WebAssembly.

#### Interpretacion del DSL hacia eventos

La fase 3.5 agrega pruebas rapidas para convertir un programa DSL validado en eventos internos del nucleo.

Para cambios en el interprete se debe ejecutar:

```bash
cargo test dsl::interpreter
cargo test core
make validate-fast
```

Esta validacion no levanta Bevy ni compila WebAssembly.

#### Ejecucion de scripts DSL

La fase 3.6 agrega una comprobacion manual para scripts `.loop` desde el binario nativo.

```bash
cargo run -- --script examples/rescate.loop --seed 123 --ticks 50
make validate-fast
```

Esta validacion no compila WebAssembly y no publica artefactos. Sirve para verificar el flujo del DSL antes de conectar editor visual o replay.

#### Visor DSL

La fase 4.1 conserva la validacion rapida sin abrir ventana y agrega una ruta visual manual para revisar scripts `.loop` en pantalla.

```bash
make validate-fast
cargo run -- --script examples/rescate.loop --seed 123 --ticks 50
```

Para inspeccion visual manual:

```bash
cargo run -- --script examples/rescate.loop --visual --seed 123 --ticks 50
```

#### Validacion visual en remoto

La validacion automatica no debe ejecutar `--visual` en servidores sin pantalla. El flujo remoto recomendado es headless.

```bash
cargo run -- --script examples/rescate.loop --seed 123 --ticks 50
make validate-fast
```

El modo `--visual` queda reservado para inspeccion manual en una sesion con entorno grafico.

### Validacion de exportacion de grafo

#### Comando recomendado

```bash
cargo run -- --script examples/rescate.loop --export-graph artifacts/rescate.graph.json
test -f artifacts/rescate.graph.json
make validate-fast
```

#### Criterio

La exportacion de grafo es compatible con trabajo remoto porque no requiere `DISPLAY`, Wayland ni ventana grafica.

### Validacion de importacion de grafo

#### Comando recomendado

```bash
cargo run -- --script examples/rescate.loop --export-graph artifacts/rescate.graph.json
cargo run -- --graph artifacts/rescate.graph.json --seed 123 --ticks 50
make validate-fast
```

#### Criterio

La importacion debe funcionar en modo remoto sin `DISPLAY`, Wayland ni ventana grafica.

### Validacion de contrato de grafo JSON

#### Uso

Para cambios en exportacion, importacion o contrato de grafos JSON, ejecutar:

```bash
cargo test dsl::graph
cargo test --test dsl_graph_contract
cargo run -- --script examples/rescate.loop --export-graph artifacts/rescate.graph.json
cargo run -- --graph artifacts/rescate.graph.json --seed 123 --ticks 50
make validate-fast
```

#### Criterio

El grafo debe conservar ids estables, metadatos consistentes, nodos validos, aristas validas y roundtrip canonico.

### Fase 5.1: eventos JSONL

#### Validacion headless

```bash
cargo run -- --script examples/rescate.loop --record artifacts/runs/dev/events.jsonl --seed 123 --ticks 50
test -f artifacts/runs/dev/events.jsonl
cargo test core::trace
cargo test --test core_trace_jsonl
make validate-fast
```

Los archivos generados en `artifacts/runs` son artefactos locales y no deben agregarse al commit.

### Fase 5.2 replay determinista

Validacion recomendada para replay:

```bash
cargo run -- --script examples/rescate.loop --record artifacts/runs/dev/events.jsonl --seed 123 --ticks 50
cargo run -- --replay artifacts/runs/dev/events.jsonl
cargo test replay
make validate-fast
```

El archivo generado en `artifacts/runs/dev/events.jsonl` es un artefacto de ejecucion y no debe agregarse al commit.

### Fase 5.3: metricas CSV

#### Validacion headless

```bash
cargo run -- --script examples/rescate.loop --metrics artifacts/runs/dev/metrics.csv --seed 123 --ticks 50
test -f artifacts/runs/dev/metrics.csv
cargo test metrics
cargo test --test simulation_metrics_csv
make validate-fast
```

El archivo generado en `artifacts/runs/dev/metrics.csv` es un artefacto local y no debe agregarse al commit.

### Fase 5.4: comparacion de corridas

#### Validacion headless

```bash
cargo run -- --script examples/rescate.loop --metrics artifacts/runs/base/metrics.csv --seed 123 --ticks 50
cargo run -- --script examples/rescate.loop --metrics artifacts/runs/dev/metrics.csv --seed 124 --ticks 50
cargo run -- --compare-metrics artifacts/runs/base/metrics.csv artifacts/runs/dev/metrics.csv --compare-output artifacts/runs/dev/comparison.csv
test -f artifacts/runs/dev/comparison.csv
cargo test compare
make validate-fast
```

Los archivos generados en `artifacts/runs` son artefactos locales y no deben agregarse al commit.

### Validación de providers LLM simulados

#### Comandos

```bash
cargo test llm
cargo test --test llm_mock_provider
make validate-fast
```

#### Criterio

Los providers de Fase 6.1 no deben usar red, HTTP real, OpenAI, Ollama ni claves de API.

### Validación de límites LLM simulados

#### Comandos

```bash
cargo test llm
cargo test --test llm_limits
make validate-fast
```

#### Criterio

Los providers LLM simulados deben rechazar prompts, respuestas y latencias simuladas fuera de limite sin usar red ni secretos.

### Validacion de proxy LLM opcional

#### Comando

```bash
cargo check --features llm-proxy
```

#### Criterio

La feature `llm-proxy` debe compilar sin activar red por defecto, sin claves hardcodeadas y con limites obligatorios.

### Validacion del supervisor multiagente

#### Comandos

```bash
cargo test supervisor
cargo test --test supervisor_multiagente
make validate-fast
```

#### Criterio

El supervisor debe detectar timeouts de forma determinista, aplicar la politica de reinicio configurada y reportar metricas de fallos detectados y recuperados sin depender de red ni entorno visual.

### Validacion de fallos recuperables

#### Comandos

```bash
cargo fmt
cargo test failure
cargo test supervisor
cargo test --test fallos_recuperables
make validate-fast
git diff --check
```

#### Criterio

Una corrida con `RecoverableFailurePlan` debe producir eventos de timeout y reinicio. Las metricas CSV deben reflejar `fallos_detectados` y `fallos_recuperados` con valores mayores que cero cuando corresponde.

### Validacion de fallo bizantino simplificado

#### Comandos

```bash
cargo fmt
cargo test byzantine
cargo test --test fallo_bizantino
make validate-fast
git diff --check
```

#### Criterio

Una votacion con mayoria honesta debe aceptar el valor correcto aunque exista un worker con respuesta falsa. Una votacion empatada o sin respuestas suficientes debe rechazarse de forma determinista.

### Validacion de fallos por CLI y DSL

#### Comandos

```bash
cargo fmt
cargo test failure
cargo test byzantine
cargo test dsl
cargo test --test fallos_cli_dsl
make validate-fast
git diff --check
```

#### Criterio

Los fallos recuperables declarados por CLI o DSL deben afectar las metricas de fallos. Los fallos bizantinos deben producir una votacion determinista con mayoria, empate o rechazo por respuestas insuficientes.

### Fase 7.5: validacion multiagente endurecida

#### Objetivo

Agregar un gate explicito para validar supervisor, fallos recuperables, fallo bizantino, CLI y DSL como un bloque integrado.

#### Comando

```bash
make validate-multiagent
```

#### Criterio

La validacion debe compilar todos los targets, ejecutar pruebas multiagente, correr escenarios CLI y verificar metricas de fallos detectados y recuperados.

### Nivel 8.1: auditoría ligera de workflows

#### Comandos

```bash
cargo fmt
cargo metadata --locked --format-version 1 --no-deps
make validate-fast
make validate-multiagent
git diff --check
```

#### Criterio

Los workflows de GitHub Actions deben usar permisos mínimos, no deben leer secretos en pull requests y no deben activar despliegues automáticos.

La validación web queda manual mediante `workflow_dispatch`.


### Nivel de auditoria Rust manual

#### Fase 8.2

La auditoria Rust queda fuera del CI automatico de PR.

Se ejecuta mediante:

```text
workflow_dispatch
schedule semanal
revision previa a release
```

El objetivo es detectar avisos de seguridad de dependencias sin ralentizar cambios normales.

### Nivel de seguridad de repositorio

#### Escaneo de secretos

La Fase 8.3 agrega un escaneo manual y semanal de secretos con `gitleaks`.

Este nivel no reemplaza la validacion funcional. Su objetivo es revisar credenciales, tokens y claves sensibles sin ralentizar los PR pequenos.

#### Comando equivalente

La ejecucion principal se realiza desde GitHub Actions mediante `workflow_dispatch`.

### Nivel de validacion profunda manual

#### Objetivo

La Fase 8.4 agrega una validacion funcional profunda que se ejecuta manualmente.

Este nivel no se ejecuta en cada PR. Se usa para revisar cambios grandes, integraciones delicadas o preparacion de release.

#### Cobertura

```text
make validate-fast
make validate-multiagent
escenarios DSL
fallos recuperables
fallo bizantino
validacion de metricas
validacion de replay
```

#### Workflow

```text
.github/workflows/deep-validation.yml
```

### Nivel de evidencia reproducible

#### Objetivo

La Fase 8.5 agrega reportes de evidencia para resumir los resultados de las validaciones manuales.

Este nivel no reemplaza las validaciones. Su objetivo es hacer que los resultados sean legibles y auditables.

#### Salidas

```text
artifacts/evidence/reporte-evidencia.md
artifacts/evidence/reporte-evidencia.json
```

#### Nivel release controlado

El nivel de release controlado agrupa las validaciones que deben revisarse antes de un release candidate.

Incluye:

```text
version declarada
changelog actualizado
validacion profunda manual
auditoria Rust manual
escaneo manual de secretos
reporte de evidencia
artefacto web manual cuando aplique
```

#### Nivel release web manual

La generacion del artefacto web queda fuera del CI automatico de PR.

Validacion recomendada:

```bash
cargo metadata --locked --format-version 1 --no-deps > /dev/null
git diff --check
grep -R "pull_request:\|push:" .github/workflows/web-build.yml || true
grep -R "upload-artifact" .github/workflows/web-build.yml
```

El objetivo es comprobar estructura y politica sin ejecutar un build web pesado en cada cambio.

### Nivel release candidate

#### Objetivo

La validacion de release candidate agrupa validaciones funcionales, web y evidencia sin ejecutarlas automaticamente en cada PR.

#### Comandos

```bash
make validate-full
make validate-web
make evidence-report-release
```

#### Regla

Estos comandos son manuales y se usan antes de crear `v0.9.0-rc1`.

### Fase 9.4: revision posterior al release candidate

#### Validacion esperada

La fase se valida con comprobaciones ligeras de estructura:

```bash
cargo fmt
cargo metadata --locked --format-version 1 --no-deps > /dev/null
git diff --check
cat VERSION
grep -n "Revision posterior" docs/REVISION_RELEASE_CANDIDATE.md docs/RELEASE.md docs/RELEASE_CANDIDATE.md
```

#### Validacion manual recomendada

La revision puede usar resultados previos o ejecutar manualmente:

```bash
make validate-full
make validate-web
make evidence-report-release
```

### Fase 10.1: escenarios comparables

#### Validacion esperada

```bash
cargo fmt
cargo metadata --locked --format-version 1 --no-deps > /dev/null
cargo test --test escenarios_comparables
git diff --check
```

#### Criterio de aceptacion

Los tres escenarios deben validar, interpretarse como DSL y exportarse como grafo.

El escenario `multiagent_failure.loop` debe exponer fallo recuperable, fallo bizantino simplificado y votacion esperada.

### Fase 10.2: benchmarks reproducibles

#### Validacion recomendada

```bash
cargo fmt
cargo metadata --locked --format-version 1 --no-deps > /dev/null
cargo test --test escenarios_comparables
bash scripts/run_benchmarks.sh
git diff --check
```

#### Salidas esperadas

```text
artifacts/benchmarks/resultados.csv
artifacts/benchmarks/resumen.md
```

### Fase 10.3: informe tecnico interno

#### Validacion recomendada

```bash
cargo fmt
cargo metadata --locked --format-version 1 --no-deps > /dev/null
cargo test --test escenarios_comparables
git diff --check
```

#### Revision documental

```bash
grep -n "Informe tecnico interno" docs/INFORME_TECNICO.md
grep -n "Resultados preliminares" docs/RESULTADOS.md
```

### Fase 10.4: validacion de cierre visual

#### Criterio

La Fase 10.4 se valida revisando que el README tenga badges, enlaces a documentos principales, graficos locales y referencias coherentes a escenarios, benchmarks e informe tecnico.

#### Comandos sugeridos

```bash
cargo fmt
cargo metadata --locked --format-version 1 --no-deps > /dev/null
git diff --check
```

### Fase 10.4.1: validacion de badges y demo

#### Validacion esperada

```text
README.md no muestra repo not found
README.md no muestra no status en workflows manuales usados como badges dinamicos
README.md no referencia un GIF inexistente
docs/HF_SPACES_DEMO.md existe
docs/assets/loopscape-demo-placeholder.svg existe
```

### Fase 10.4.2: validacion de demo publicada

#### Validacion esperada

```text
README.md contiene el enlace publico de Hugging Face Spaces
README.md no contiene repo not found
README.md no contiene no status en badges manuales
README.md no referencia un GIF inexistente
docs/HF_SPACES_DEMO.md registra Git LFS para WebAssembly
la demo abre desde https://huggingface.co/spaces/kapumota/loopscape
```

### Fase 10.5, validacion de uso guiado

#### Criterio

Una persona externa debe poder abrir la demo, entender las cinco eras, ejecutar el smoke nativo, correr escenarios comparables y leer resultados preliminares sin revisar todo el codigo fuente.

#### Validacion manual

```bash
grep -n "Como usar Loopscape" README.md
grep -n "Demo guiada" docs/DEMO_GUIADA.md
grep -n "Guia de uso" docs/GUIA_USO.md
grep -n "Lectura rapida" docs/LECTURA_RAPIDA.md
```

### Fase 11.1: cierre experimental

#### Validacion esperada

- `VERSION` contiene `0.9.0`.
- `CHANGELOG.md` contiene entrada `0.9.0`.
- README apunta a la demo publicada.
- README explica como usar el proyecto.
- Documentacion de cierre existe.
- Roadmap posterior existe.
- No quedan textos pendientes de release candidate en la portada principal.
