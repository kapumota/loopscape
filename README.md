### Loopscape

[![CI](https://github.com/kapumota/loopscape/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/kapumota/loopscape/actions/workflows/ci.yml)
![Version](https://img.shields.io/badge/version-0.9.0-blue)
![Release](https://img.shields.io/badge/release-v0.9.0-blue)
![License](https://img.shields.io/badge/license-MIT-blue)
![Rust](https://img.shields.io/badge/rust-stable-orange)
![WebAssembly](https://img.shields.io/badge/target-wasm32--unknown--unknown-blue)
![Bevy](https://img.shields.io/badge/engine-Bevy-purple)
![Benchmarks](https://img.shields.io/badge/benchmarks-reproducibles-blue)
![Validacion profunda](https://img.shields.io/badge/validacion-profunda%20manual-lightgrey)
![Auditoria Rust](https://img.shields.io/badge/auditoria-rust%20manual-lightgrey)
![Secretos](https://img.shields.io/badge/secretos-escaneo%20manual-lightgrey)
![Artefacto web](https://img.shields.io/badge/artefacto-web%20manual-lightgrey)
![HF Spaces](https://img.shields.io/badge/HF%20Spaces-demo%20publicada-blue)

#### Resumen

Loopscape es un sandbox visual de automatizacion y computo cognitivo construido con Rust, Bevy y WebAssembly. El proyecto modela la evolucion de loops agenticos desde un ciclo ReAct secuencial hasta una red de orquestacion multiagente con supervisores, trabajadores, latidos, replay, metricas y fallos simulados.

La version actual corresponde a `0.9.0`. Es un cierre experimental preparado para investigacion, docencia avanzada, evaluacion reproducible y demostracion publica.

#### Vista rapida

![Flujo conceptual de Loopscape](docs/assets/loopscape-flujo.svg)

![Pipeline de validacion de Loopscape](docs/assets/loopscape-validacion.svg)

#### Demo web en Hugging Face Spaces

La demo web de Loopscape esta publicada y probada en Hugging Face Spaces:

[Abrir demo web de Loopscape](https://huggingface.co/spaces/kapumota/loopscape)

La publicacion usa el artefacto web generado por Trunk desde `dist/`. El archivo WebAssembly del Space se almacena con Git LFS porque supera el limite de archivo ordinario.

#### Como usar Loopscape

Loopscape puede usarse de tres formas complementarias:

1. Como demo web publica en Hugging Face Spaces.
2. Como aplicacion local nativa con `cargo run`.
3. Como laboratorio reproducible con escenarios, benchmarks, replay y reportes.

Para una primera exploracion se recomienda abrir la demo web, cambiar entre eras con las teclas `1` a `5`, activar el modo Rayos X con `X` y provocar un fallo bizantino con `B` en la Era 5.

#### Ruta rapida de exploracion

1. Abre la demo web publicada en Hugging Face Spaces.
2. Observa la Era 1 como ciclo ReAct secuencial.
3. Presiona `2`, `3`, `4` y `5` para comparar la evolucion del sistema.
4. Usa `WASD` o las flechas para mover la camara.
5. Presiona `X` para alternar el modo Rayos X.
6. Presiona `L` para alternar el panel LLM.
7. En la Era 3, presiona `M` para mutar el ADN de comportamiento.
8. En la Era 5, presiona `B` para inyectar un fallo bizantino simplificado.
9. Revisa `docs/GUIA_USO.md` para una lectura guiada.
10. Revisa `docs/DEMO_GUIADA.md` para una secuencia de demostracion.

#### Que observar en cada era

| Era | Que representa | Que observar |
|---|---|---|
| Era 1 | Ciclo ReAct basico | Secuencia Think, Act y Observe |
| Era 2 | Autoprompting | Aparicion de prompts como mecanismo de adaptacion |
| Era 3 | Ralph Loop | Mutacion de ADN de comportamiento con `M` |
| Era 4 | Ralph formalizado | Comandos y estructura de orquestacion mas explicita |
| Era 5 | Orquestacion multiagente | Supervisores, trabajadores, latidos, fallos y recuperacion |

#### Uso como laboratorio reproducible

Para validar que Loopscape no es solo una animacion, ejecuta la ruta reproducible:

```bash
cargo run -- --smoke --seed 123 --ticks 10
cargo test --test escenarios_comparables
bash scripts/run_benchmarks.sh
```

Los escenarios comparables estan en:

```text
scenarios/react_basic.loop
scenarios/dsl_delegation.loop
scenarios/multiagent_failure.loop
```

Los resultados locales de benchmark se generan en:

```text
artifacts/benchmarks/
```

Estos resultados no se versionan. La evidencia resumida se documenta en `docs/RESULTADOS.md` y `docs/INFORME_TECNICO.md`.

#### Cierre experimental v0.9.0

Loopscape se cierra en `v0.9.0` como laboratorio experimental reproducible. Esta version integra demo web, guia de uso, escenarios comparables, benchmarks reproducibles, informe tecnico y documentacion de release.

El objetivo del cierre no es declarar un producto industrial. El objetivo es fijar una version estable para docencia avanzada, investigacion aplicada, demostracion tecnica y futuras extensiones controladas.

La demo publica esta disponible en Hugging Face Spaces:

[Abrir demo web de Loopscape](https://huggingface.co/spaces/kapumota/loopscape)

La lectura recomendada para usar el proyecto es:

- `docs/LECTURA_RAPIDA.md`, primer recorrido en pocos minutos.
- `docs/GUIA_USO.md`, uso local, web y reproducible.
- `docs/DEMO_GUIADA.md`, recorrido por eras y controles.
- `docs/ESCENARIOS_COMPARABLES.md`, entradas estables para pruebas.
- `docs/BENCHMARKS.md`, ejecucion de benchmarks.
- `docs/INFORME_TECNICO.md`, interpretacion tecnica del sistema.
- `docs/CIERRE_EXPERIMENTAL.md`, alcance exacto de `v0.9.0`.

#### Objetivo del proyecto

El objetivo no es solo mostrar agentes en pantalla. Loopscape busca convertirse en un laboratorio interactivo para estudiar:

- ciclos Think, Act y Observe,
- descomposicion automatica de tareas,
- prompts compartidos como ADN de comportamiento,
- comandos formales de orquestacion,
- supervision multiagente, consenso, fallos y recuperacion,
- replay determinista, metricas comparables y benchmarks reproducibles.

#### Estado actual

Loopscape ya cuenta con una base visual y experimental organizada en cinco eras:

- Era 1: ReAct,
- Era 2: Autoprompting,
- Era 3: Ralph Loop,
- Era 4: Ralph formalizado,
- Era 5: Orquestacion multiagente.

Tambien incluye una linea experimental avanzada:

- nucleo determinista separado de Bevy,
- DSL con lexer, parser, validador e interprete,
- visor DSL,
- exportacion e importacion de grafo JSON,
- eventos JSONL y replay determinista,
- metricas CSV y comparacion de corridas,
- proveedor LLM mock y proxy opcional con limites,
- supervisor real con fallos recuperables y fallo bizantino simplificado,
- auditoria manual de workflows, Rust, secretos y validacion profunda,
- reportes de evidencia,
- escenarios comparables y benchmarks reproducibles,
- informe tecnico interno y resultados preliminares.

#### Requisitos

- Rust estable,
- target `wasm32-unknown-unknown` para compilacion web,
- Trunk para ejecutar o compilar la version WebAssembly,
- Node.js solo si se usa el proxy local de LLM,
- Git para trabajar por ramas y generar patches.

#### Instalacion rapida

```bash
rustup target add wasm32-unknown-unknown
cargo install trunk --locked
```

#### Uso nativo

```bash
cargo run
```

#### Smoke nativo

Para verificar que el binario arranca y que el nucleo determinista ejecuta un numero pequeño de ticks:

```bash
cargo run -- --smoke --seed 123 --ticks 10
```

Tambien se puede usar:

```bash
make smoke-native
```

#### Uso web local

```bash
trunk serve
```

Luego abre:

```text
http://localhost:8080
```

#### Validacion recomendada

```bash
make setup
make validate
make smoke-native
make clean
```

Para validar el artefacto web de forma explicita:

```bash
make setup-web
make validate-web
```

Para una revision de cierre experimental:

```bash
make validate-full
make validate-web
cargo clippy --all-targets -- -D warnings
```

#### Escenarios comparables

Los escenarios comparables viven en `scenarios/`:

```text
scenarios/react_basic.loop
scenarios/dsl_delegation.loop
scenarios/multiagent_failure.loop
```

Estos archivos sirven como entradas estables para pruebas, benchmarks e informe tecnico.

#### Benchmarks reproducibles

La configuracion de benchmarks vive en `benchmarks/` y el script principal es:

```bash
bash scripts/run_benchmarks.sh
```

El script genera resultados locales en:

```text
artifacts/benchmarks/
```

Los resultados generados no se versionan. Solo se conserva `artifacts/benchmarks/.gitkeep`.

#### Controles

| Tecla | Accion |
|---|---|
| `1` a `5` | Cambiar entre eras |
| `WASD` o flechas | Mover camara |
| `M` | Mutar ADN en Era 3 |
| `B` | Inyectar fallo bizantino en Era 5 |
| `L` | Alternar panel LLM |
| `X` | Alternar modo Rayos X |

#### Estructura principal

```text
src/
  main.rs
  core/
  dsl/
  eras/
  systems/
scenarios/
benchmarks/
scripts/
docs/
artifacts/
```

#### Documentacion principal

- `docs/GUIA_USO.md`: guia practica para usar Loopscape como demo, simulador y laboratorio.
- `docs/DEMO_GUIADA.md`: recorrido sugerido para presentar la demo web y local.
- `docs/LECTURA_RAPIDA.md`: ruta corta para entender el proyecto en pocos minutos.
- `docs/ARQUITECTURA.md`: arquitectura base del proyecto.
- `docs/PLAN_FASES_AVANZADO.md`: plan de fases del proyecto.
- `docs/VALIDACION_POR_NIVELES.md`: matriz de validacion progresiva.
- `docs/ESCENARIOS_COMPARABLES.md`: descripcion de escenarios comparables.
- `docs/BENCHMARKS.md`: ejecucion y lectura de benchmarks.
- `docs/INFORME_TECNICO.md`: informe tecnico interno.
- `docs/RESULTADOS.md`: resultados preliminares.
- `docs/RELEASE.md`: proceso de release.
- `docs/RELEASE_CANDIDATE.md`: alcance historico de `v0.9.0-rc1`.
- `docs/REVISION_RELEASE_CANDIDATE.md`: revision posterior al release candidate.
- `docs/INDICE_FINAL.md`: indice final del repositorio.
- `docs/HF_SPACES_DEMO.md`: preparacion de demo web para Hugging Face Spaces.

#### Flujo por rama

```bash
git checkout main
git pull --ff-only origin main
git checkout -b fase-nombre

make validate

git add .
git commit -m "fase n: descripcion del cambio"
git push -u origin fase-nombre
```

Despues se abre un Pull Request hacia `main` y se revisa el resultado de CI antes de fusionar.

#### Flujo con patches

Para generar un patch desde la rama de trabajo:

```bash
git diff main...HEAD > patches/fase-nombre.patch
```

Para aplicar un patch en otra copia del repositorio:

```bash
git checkout -b fase-nombre
git apply patches/fase-nombre.patch
make validate
```

#### Politica de CI

El flujo principal de GitHub Actions debe mantenerse liviano. Los PR normales deben validar formato, metadata, pruebas pequeñas y documentacion. Los workflows pesados quedan como ejecuciones manuales.

El workflow web debe mantenerse manual:

```text
workflow_dispatch
sin push automatico
sin GitHub Pages automatico
sube dist como artifact
```

#### Cierre experimental

La version de cierre experimental es `0.9.0`. El tag debe crearse solo desde `main` actualizado despues de fusionar el PR correspondiente.

```bash
git checkout main
git pull --ff-only origin main
git tag -a v0.9.0 -m "cierre experimental v0.9.0"
git push origin v0.9.0
```

#### Licencia

MIT. El proyecto esta orientado a educacion, investigacion aplicada y prototipado de sistemas interactivos.
