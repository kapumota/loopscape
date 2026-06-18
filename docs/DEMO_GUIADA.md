### Demo guiada de Loopscape

#### Objetivo

Esta demo guiada permite presentar Loopscape en pocos minutos sin asumir conocimiento previo del codigo.

#### Paso 1, abrir la demo web

Abre:

```text
https://huggingface.co/spaces/kapumota/loopscape
```

Explica que la demo es un sandbox visual de automatizacion y computo cognitivo construido con Rust, Bevy y WebAssembly.

#### Paso 2, mostrar la Era 1

Presiona `1`.

Explica que esta era representa un ciclo ReAct basico. El foco es entender una cadena simple de pensamiento, accion y observacion.

#### Paso 3, comparar con la Era 2

Presiona `2`.

Explica que el sistema introduce autoprompting y empieza a reutilizar prompts como parte del comportamiento.

#### Paso 4, mostrar mutacion en Era 3

Presiona `3` y luego `M`.

Explica que la mutacion muestra cambios de comportamiento dentro del ciclo Ralph Loop.

#### Paso 5, mostrar formalizacion en Era 4

Presiona `4`.

Explica que esta era introduce estructura mas formal para comandos, orquestacion y transiciones.

#### Paso 6, mostrar multiagente en Era 5

Presiona `5`.

Explica que esta era presenta supervisores, trabajadores, latidos y recuperacion.

#### Paso 7, inyectar fallo

En la Era 5, presiona `B`.

Explica que esto simula un fallo bizantino simplificado. El objetivo no es demostrar tolerancia bizantina industrial, sino visualizar una condicion adversarial controlada.

#### Paso 8, activar Rayos X

Presiona `X`.

Explica que el modo Rayos X ayuda a observar estructura interna y no solo movimiento visual.

#### Paso 9, conectar con evidencia

Despues de la demo visual, muestra que existen escenarios y benchmarks:

```bash
cargo test --test escenarios_comparables
bash scripts/run_benchmarks.sh
```

#### Cierre de la demo

La conclusion recomendada es:

```text
Loopscape no es solo una animacion. Es una plataforma experimental reproducible para estudiar orquestacion multiagente, DSL, replay, metricas, fallos y validacion tecnica.
```
