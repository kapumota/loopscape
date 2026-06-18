### Lectura rapida de Loopscape

#### En una frase

Loopscape es una plataforma experimental reproducible para estudiar loops agenticos, orquestacion multiagente, replay, metricas, fallos y validacion tecnica usando Rust, Bevy y WebAssembly.

#### Que mirar primero

1. Abre la demo en Hugging Face Spaces.
2. Cambia entre las eras con `1` a `5`.
3. Activa modo Rayos X con `X`.
4. Inyecta fallo en Era 5 con `B`.
5. Lee `docs/GUIA_USO.md`.
6. Ejecuta `cargo test --test escenarios_comparables`.
7. Ejecuta `bash scripts/run_benchmarks.sh`.

#### Que demuestra el proyecto

El proyecto demuestra una ruta incremental desde una animacion agentica hacia un laboratorio reproducible con:

- nucleo determinista,
- DSL,
- escenarios comparables,
- benchmarks reproducibles,
- replay,
- metricas,
- fallos recuperables,
- fallo bizantino simplificado,
- demo web publicada,
- informe tecnico.

#### Que no afirma

Loopscape no afirma ser un producto industrial terminado. Tampoco afirma implementar tolerancia bizantina completa para produccion. Su valor actual es experimental, docente y de investigacion aplicada.

#### Lectura recomendada

```text
docs/GUIA_USO.md
docs/DEMO_GUIADA.md
docs/ESCENARIOS_COMPARABLES.md
docs/BENCHMARKS.md
docs/RESULTADOS.md
docs/INFORME_TECNICO.md
```
