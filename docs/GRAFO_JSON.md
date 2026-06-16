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
