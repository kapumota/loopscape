### Grafo JSON de orquestacion

#### Objetivo

La exportacion de grafo JSON permite guardar la estructura logica de un flujo DSL sin depender del visor visual ni de un editor de nodos.

Esta fase convierte un archivo `.loop` en un archivo JSON con nodos, aristas, metadatos e identificadores estables.

#### Comando principal

```bash
cargo run -- --script examples/rescate.loop --export-graph artifacts/rescate.graph.json
```

#### Validacion remota

```bash
cargo run -- --script examples/rescate.loop --export-graph artifacts/rescate.graph.json
test -f artifacts/rescate.graph.json
make validate-fast
```

#### Estructura del archivo

El archivo exportado contiene tres secciones principales.

```text
metadatos
nodos
aristas
```

#### Metadatos

Los metadatos incluyen la version del formato, la ruta del script de origen, la cantidad de comandos, la cantidad de nodos y la cantidad de aristas.

#### Nodos

Cada nodo representa una unidad logica del programa DSL. Un `/goal` genera un nodo de objetivo. Un `/plan` genera un nodo por cada paso. Los comandos `/delegate`, `/verify` y `/terminate` generan nodos propios.

Cada nodo tiene un `id` estable para que futuras fases puedan comparar grafos, producir replay o mostrar diferencias.

#### Aristas

Las aristas representan el flujo secuencial del programa de orquestacion. Por ahora todas las aristas usan tipo `flow` y etiqueta `siguiente`.

#### Alcance

Esta fase no implementa editor visual, guardado de nodos editables ni ejecucion distribuida. Solo exporta la estructura logica del DSL como JSON reproducible.

### Importacion de grafo JSON

#### Objetivo

La importacion permite cargar un grafo previamente exportado y verificar que conserva nodos, aristas, metadatos e ids estables.

#### Comando principal

```bash
cargo run -- --graph artifacts/rescate.graph.json --seed 123 --ticks 50
```

#### Validacion remota

Si el archivo no existe, generarlo primero con la exportacion de la fase 4.2.

```bash
cargo run -- --script examples/rescate.loop --export-graph artifacts/rescate.graph.json
cargo run -- --graph artifacts/rescate.graph.json --seed 123 --ticks 50
make validate-fast
```

#### Criterio

El importador rechaza grafos con version no soportada, conteos inconsistentes, ids duplicados o aristas que apunten a nodos inexistentes.

### Contrato estable y roundtrip

#### Objetivo

El contrato estable define que un grafo exportado debe poder importarse y serializarse de nuevo sin cambiar su estructura canonica.

El ciclo esperado es:

```text
DSL
JSON exportado
importacion JSON
validacion de contrato
JSON canonico
```

#### Campos obligatorios

El formato conserva estas secciones de primer nivel:

```text
metadatos
nodos
aristas
```

Los nodos deben mantener `id`, `kind`, `label`, `command`, `source_line` y `order`.

Las aristas deben mantener `id`, `from`, `to`, `kind` y `label`.

#### Ids estables

Los nodos exportados usan ids derivados del orden del comando, el tipo de nodo y el indice local.

Las aristas exportadas usan ids secuenciales con la forma `edge-000`, `edge-001` y asi sucesivamente.

#### Validacion recomendada

```bash
cargo test dsl::graph
cargo test --test dsl_graph_contract
cargo run -- --script examples/rescate.loop --export-graph artifacts/rescate.graph.json
cargo run -- --graph artifacts/rescate.graph.json --seed 123 --ticks 50
make validate-fast
```

#### Alcance

Esta fase no cambia el visor visual ni implementa editor de nodos. Su objetivo es estabilizar el formato que usaran replay, metricas, comparacion de escenarios y futuras herramientas visuales.
