### Guia de uso de Loopscape

#### Proposito

Esta guia explica como usar Loopscape como demo visual, simulador local y laboratorio reproducible.

Loopscape no debe leerse solo como una animacion. La interfaz visual muestra una capa observable de un sistema que tambien incluye nucleo determinista, DSL, replay, metricas, escenarios comparables y benchmarks reproducibles.

#### Formas de uso

Loopscape tiene tres formas principales de uso:

1. Demo web publica para exploracion rapida.
2. Ejecucion local nativa para pruebas de desarrollo.
3. Laboratorio reproducible para escenarios, benchmarks y reportes.

#### Uso como demo web

Abre la demo publicada en Hugging Face Spaces:

```text
https://huggingface.co/spaces/kapumota/loopscape
```

La demo permite observar la evolucion de cinco eras de automatizacion agentica. La primera lectura debe enfocarse en comparar como cambia la estructura del sistema entre una reaccion secuencial y una orquestacion multiagente.

#### Uso local nativo

Ejecuta la aplicacion local:

```bash
cargo run
```

Para una prueba corta sin sesion larga:

```bash
cargo run -- --smoke --seed 123 --ticks 10
```

#### Uso web local

Compila y sirve la version WebAssembly:

```bash
rustup target add wasm32-unknown-unknown
cargo install trunk --locked
trunk serve
```

Luego abre:

```text
http://localhost:8080
```

#### Controles principales

| Tecla | Uso |
|---|---|
| `1` | Cambia a la Era 1, ciclo ReAct |
| `2` | Cambia a la Era 2, autoprompting |
| `3` | Cambia a la Era 3, Ralph Loop |
| `4` | Cambia a la Era 4, Ralph formalizado |
| `5` | Cambia a la Era 5, orquestacion multiagente |
| `WASD` | Mueve la camara |
| Flechas | Mueven la camara |
| `M` | Muta ADN de comportamiento en la Era 3 |
| `B` | Inyecta fallo bizantino simplificado en la Era 5 |
| `L` | Alterna el panel LLM |
| `X` | Alterna modo Rayos X |

#### Lectura por eras

La Era 1 muestra un ciclo ReAct basico. El usuario debe observar una secuencia simple de pensamiento, accion y observacion.

La Era 2 introduce autoprompting. El usuario debe observar como los prompts compartidos empiezan a representar comportamiento reutilizable.

La Era 3 introduce Ralph Loop. El usuario debe probar la tecla `M` para observar mutaciones de comportamiento.

La Era 4 formaliza la orquestacion. El usuario debe observar que el sistema ya no depende solo de movimiento visual, sino de comandos y estructura.

La Era 5 introduce supervision multiagente. El usuario debe observar trabajadores, supervisor, latidos, fallos y recuperacion.

#### Ruta reproducible

Para usar Loopscape como laboratorio, ejecuta:

```bash
cargo test --test escenarios_comparables
bash scripts/run_benchmarks.sh
```

Los escenarios base son:

```text
scenarios/react_basic.loop
scenarios/dsl_delegation.loop
scenarios/multiagent_failure.loop
```

#### Como leer resultados

Los resultados generados por benchmarks se guardan localmente en:

```text
artifacts/benchmarks/
```

Los resultados no se versionan. La interpretacion estable se documenta en:

```text
docs/RESULTADOS.md
docs/INFORME_TECNICO.md
```

#### Criterio de uso correcto

Una persona externa deberia poder hacer lo siguiente:

1. Abrir la demo web.
2. Cambiar entre las cinco eras.
3. Identificar que representa cada era.
4. Ejecutar el smoke nativo.
5. Ejecutar escenarios comparables.
6. Ejecutar benchmarks reproducibles.
7. Leer resultados e informe tecnico.
