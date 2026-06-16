### Plan de fases para un Loopscape avanzado

#### Respuesta corta

Para que Loopscape llegue a un nivel avanzado y no se quede como demo visual, recomiendo 10 fases. Con 5 fases puedes lograr un prototipo fuerte. Con 8 fases puedes tener un software serio. Con 10 fases ya se acerca a un laboratorio interactivo avanzado.

#### Fase 0: importacion inicial

Objetivo: crear repositorio, normalizar estructura y dejar `main` como base limpia.

Entregables:

- repositorio Git inicial;
- README minimo;
- licencia;
- `.gitignore`;
- primer tag `v0.0.0` si se desea congelar la base importada.

Rama sugerida:

```text
fase-0-importacion-inicial
```

#### Fase 1: base profesional reproducible

Objetivo: dejar el proyecto validable por cualquier persona.

Entregables:

- `Makefile`;
- scripts de limpieza y validacion;
- CI basico;
- documentacion de arquitectura;
- documentacion de flujo por ramas y patches;
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

- modulo `core` para tareas, tokens, loops y scheduler;
- simulacion por ticks;
- pruebas unitarias del ciclo Think, Act y Observe;
- estado reproducible con semilla fija;
- eventos internos tipados.

Rama sugerida:

```text
fase-2-core-simulacion
```

#### Fase 3: DSL de orquestacion

Objetivo: implementar comandos formales como `/goal`, `/plan`, `/delegate` y `/verify`.

Entregables:

- lexer;
- parser;
- AST;
- validador semantico;
- interprete minimo;
- pruebas de scripts validos e invalidos.

Rama sugerida:

```text
fase-3-dsl-orquestacion
```

#### Fase 4: editor visual de nodos

Objetivo: permitir que el usuario conecte loops, herramientas y rutas de tareas desde la interfaz.

Entregables:

- seleccion de entidades;
- arrastre de nodos;
- conexiones visuales;
- panel de propiedades;
- modo Rayos X;
- guardado de grafo en JSON.

Rama sugerida:

```text
fase-4-editor-visual
```

#### Fase 5: runtime LLM con sandbox

Objetivo: integrar LLMs sin exponer claves ni romper reproducibilidad.

Entregables:

- proveedor simulado por defecto;
- proxy local;
- abstraccion de proveedor;
- limites de tokens;
- timeouts;
- modo offline;
- trazas ReAct persistibles.

Rama sugerida:

```text
fase-5-runtime-llm-sandbox
```

#### Fase 6: orquestacion multiagente y fallos

Objetivo: convertir los loops en infraestructura gestionada.

Entregables:

- supervisores;
- trabajadores;
- especialistas;
- latidos;
- deteccion de loops colgados;
- reinicio de workers;
- fallos bizantinos simulados;
- votacion o consenso simplificado.

Rama sugerida:

```text
fase-6-multiagente-fallos
```

#### Fase 7: replay, metricas y benchmarks

Objetivo: hacer que cada ejecucion pueda analizarse y compararse.

Entregables:

- grabacion de runs;
- replay determinista;
- metricas de throughput, latencia y uso de tokens;
- benchmarks reproducibles;
- exportacion de resultados en JSON y CSV.

Rama sugerida:

```text
fase-7-replay-benchmarks
```

#### Fase 8: seguridad, politicas y CI avanzado

Objetivo: endurecer el proyecto para uso publico y colaborativo.

Entregables:

- CI con `cargo fmt`, `cargo clippy` y tests;
- validacion WASM;
- escaneo de secretos;
- politica de seguridad;
- permisos minimos en GitHub Actions;
- separacion entre CI ligero y build web manual;
- evidencia de validacion.

Rama sugerida:

```text
fase-8-seguridad-ci
```

#### Fase 9: release web y escritorio

Objetivo: entregar versiones instalables o ejecutables.

Entregables:

- build web con Trunk;
- build web estatico sin publicacion automatica en GitHub Pages;
- release con artefactos;
- changelog por version;
- tag `v0.9.0` o `v1.0.0-rc1`;
- instrucciones para usuarios finales.

Rama sugerida:

```text
fase-9-release-web-desktop
```

#### Fase 10: investigacion avanzada

Objetivo: convertir Loopscape en laboratorio de computo cognitivo, no solo en juego.

Entregables:

- escenarios comparables entre eras;
- metricas por paradigma de control;
- experimentos reproducibles;
- scripts de evaluacion;
- informe tecnico;
- tablero de resultados;
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
