### Cierre experimental v0.9.0

#### Proposito

Este documento define el cierre experimental de Loopscape en la version `0.9.0`.

La version `0.9.0` no declara un producto industrial. Declara un laboratorio experimental reproducible para estudiar loops agenticos, orquestacion multiagente, DSL, replay, metricas, fallos simulados, benchmarks e informes tecnicos.

#### Estado de cierre

Loopscape queda cerrado como una plataforma experimental con los siguientes elementos:

- Demo web publicada en Hugging Face Spaces.
- Uso nativo mediante `cargo run`.
- Smoke nativo con semilla y ticks controlados.
- Nucleo determinista separado de Bevy.
- DSL con lexer, parser, validador e interprete.
- Visor DSL.
- Exportacion e importacion de grafo JSON.
- Eventos JSONL y replay determinista.
- Metricas CSV y comparacion de corridas.
- Proveedor LLM mock y proxy opcional con limites.
- Supervisor multiagente con fallos recuperables.
- Fallo bizantino simplificado.
- Escenarios comparables.
- Benchmarks reproducibles.
- Informe tecnico interno.
- Resultados preliminares.
- Guia de uso, demo guiada y lectura rapida.

#### Criterios cumplidos

La version se considera cerrada si se cumplen estos criterios:

- `VERSION` contiene `0.9.0`.
- `CHANGELOG.md` contiene la entrada `0.9.0`.
- La demo en Hugging Face Spaces esta publicada y documentada.
- El README explica como usar el proyecto.
- Los escenarios comparables existen en `scenarios/`.
- Los benchmarks reproducibles existen en `benchmarks/` y `scripts/`.
- La documentacion tecnica contiene informe, resultados, validacion y release.
- La validacion de docs no contiene secciones pendientes sobre una demo inexistente.

#### Validacion recomendada

```bash
cargo fmt
cargo metadata --locked --format-version 1 --no-deps > /dev/null
cargo test --test escenarios_comparables
git diff --check
```

Para una revision mas fuerte antes del tag:

```bash
make validate-full
make validate-web
cargo clippy --all-targets -- -D warnings
```

#### Tag de cierre

El tag se crea solo despues de fusionar el PR en `main`:

```bash
git checkout main
git pull --ff-only origin main
git tag -a v0.9.0 -m "cierre experimental v0.9.0"
git push origin v0.9.0
```

#### Fuera de alcance

Queda fuera del cierre experimental:

- Producto industrial.
- API publica estable.
- Compatibilidad semantica garantizada.
- Matriz completa de plataformas.
- Pruebas prolongadas con usuarios externos.
- Publicacion automatica de GitHub Pages.
- Firmado de artefactos.
- Soporte operativo.

#### Decision

Loopscape queda listo para publicarse como `v0.9.0` experimental.

El siguiente trabajo debe ubicarse despues del cierre, no dentro de esta version.
