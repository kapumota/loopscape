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
