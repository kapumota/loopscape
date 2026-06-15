### Loopscape

Loopscape es un sandbox visual de automatizacion y computo cognitivo construido con Rust, Bevy y WebAssembly. El proyecto modela la evolucion de loops agenticos desde un ciclo ReAct secuencial hasta una red de orquestacion multiagente con supervisores, trabajadores, latidos y fallos simulados.

#### Objetivo del proyecto

El objetivo no es solo mostrar agentes en pantalla. Loopscape busca convertirse en un laboratorio interactivo para estudiar:

- ciclos Think, Act y Observe;
- descomposicion automatica de tareas;
- prompts compartidos como ADN de comportamiento;
- comandos formales de orquestacion;
- supervision multiagente, consenso, fallos y recuperacion.

#### Estado actual

Esta version contiene una base jugable y visual con cinco eras:

- Era 1: ReAct;
- Era 2: Autoprompting;
- Era 3: Ralph Loop;
- Era 4: Ralph formalizado;
- Era 5: Orquestacion multiagente.

La Fase 1 profesionaliza la base sin cambiar todavia el concepto central del juego. Agrega validacion reproducible, limpieza, documentacion tecnica, flujo por ramas, preparacion para parches y CI ligero sin despliegue automatico en GitHub Pages.

#### Requisitos

- Rust estable;
- target `wasm32-unknown-unknown` para compilacion web;
- Trunk solo para ejecutar o compilar la version WASM;
- Node.js solo si se usa el proxy local de LLM.

#### Uso nativo

```bash
cargo run
```

#### Uso web local

```bash
rustup target add wasm32-unknown-unknown
cargo install trunk --locked
trunk serve
```

Luego abre:

```text
http://localhost:8080
```

#### Comandos recomendados

```bash
make setup
make validate
make clean
```

#### Flujo por rama

```bash
git checkout main
git pull origin main
git checkout -b fase-1-base-profesional

make validate

git add .
git commit -m "fase 1: endurece base reproducible de Loopscape"
git push -u origin fase-1-base-profesional
```

Despues se abre un Pull Request hacia `main` y se revisa el resultado de CI antes de fusionar.

#### Flujo con patches

Para generar un patch desde la rama de trabajo:

```bash
git diff main...HEAD > patches/fase-1-base-profesional.patch
```

Para aplicar un patch en otra copia del repositorio:

```bash
git checkout -b fase-1-base-profesional
git apply patches/fase-1-base-profesional.patch
make check
make test
```

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
  components.rs
  resources.rs
  events.rs
  llm_integration.rs
  networking.rs
  eras/
    react.rs
    self_prompting.rs
    ralph.rs
    productized.rs
    orchestration.rs
  systems/
    camera.rs
    rendering.rs
    ui.rs
docs/
  ARQUITECTURA.md
  FASE_1_BASE_PROFESIONAL.md
  CI_LIGERO.md
  CI_LIGERO.md
  FLUJO_RAMA_PATCHES.md
  CREAR_REPOSITORIO_RAMAS.md
  PLAN_FASES_AVANZADO.md
scripts/
  clean.sh
  validate.sh
```


#### Documentacion de avance

- `docs/CREAR_REPOSITORIO_RAMAS.md`: flujo desde crear repositorio hasta trabajar por ramas y patches.
- `docs/PLAN_FASES_AVANZADO.md`: plan de 10 fases para llevar Loopscape a nivel avanzado.
- `docs/ARQUITECTURA.md`: arquitectura base del proyecto.
- `docs/FASE_1_BASE_PROFESIONAL.md`: alcance de la Fase 1.
- `docs/CI_LIGERO.md`: separacion entre CI diario, build web manual y despliegue externo.
- `docs/CI_LIGERO.md`: separacion entre CI diario, build web manual y despliegue externo.

#### Criterio de Fase 1 lista

La Fase 1 se considera lista cuando pasa este comando:

```bash
make validate
```

Para revisar la version web de forma explicita:

```bash
make setup-web
make validate-web
```

#### Licencia

MIT. El proyecto esta orientado a educacion, investigacion aplicada y prototipado de sistemas interactivos.

#### CI ligero

El flujo principal de GitHub Actions valida solo estilo, formato, compilacion nativa y pruebas. El build WebAssembly queda separado en un workflow manual y no publica en GitHub Pages. Ver `docs/CI_LIGERO.md`.
