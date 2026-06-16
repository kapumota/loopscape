### DSL de orquestacion

#### Objetivo

El DSL de orquestacion define una forma estructurada de describir objetivos, planes, delegaciones, verificaciones y politicas de terminacion dentro de Loopscape.

Esta fase define el AST y el modelo de comandos. Todavia no incluye lexer, parser, validador semantico ni interprete.

#### Comandos minimos

```text
/goal rescatar_victimas
/plan buscar -> clasificar -> asistir
/delegate sector_a worker_1
/verify checklist_final
/terminate when_verified
```

#### Modelo interno

El modulo `src/dsl` introduce los tipos principales:

```text
CommandKind
OrchestrationCommand
OrchestrationProgram
DslError
```

`CommandKind` representa los comandos soportados. `OrchestrationCommand` representa un comando normalizado con argumentos. `OrchestrationProgram` agrupa comandos y permite contar comandos por tipo. `DslError` centraliza errores estructurados del DSL.

#### Alcance de la fase 3.1

Esta fase no interpreta texto de usuario. Los comandos se construyen desde Rust mediante constructores tipados. El parser sera agregado en una fase posterior.

#### Validacion

```bash
cargo test dsl
make validate-fast
```

#### Criterio de cierre

La fase queda cerrada si el modulo `src/dsl` compila como parte de la biblioteca, si los comandos minimos tienen representacion tipada y si las pruebas del DSL pasan sin levantar Bevy ni compilar WebAssembly.

#### Lexer minimo

La fase 3.2 agrega un lexer para convertir texto del DSL en tokens. El lexer reconoce comandos con barra, identificadores, flechas, numeros, cadenas simples, saltos de linea y comentarios.

Ejemplo de entrada:

```text
/goal rescatar_victimas
/plan buscar -> clasificar -> asistir
/delegate sector_a worker_1
/verify "checklist final"
/terminate 10
```

Tipos de token principales:

```text
Command
Identifier
Number
StringLiteral
Arrow
Newline
Comment
Eof
```

El lexer no construye todavia un programa de orquestacion. Esa responsabilidad queda para el parser de la fase 3.3.

Validacion:

```bash
cargo test dsl::lexer
make validate-fast
```

#### Parser minimo

La fase 3.3 agrega un parser minimo para convertir tokens en un `OrchestrationProgram`. El parser usa el lexer de la fase 3.2 y construye comandos tipados del AST definido en la fase 3.1.

Ejemplo soportado:

```text
/goal rescatar_victimas
/plan buscar -> clasificar -> asistir
/delegate sector_a worker_1
/verify checklist_final
/terminate when verified
```

Reglas iniciales:

```text
cada linea util inicia con un comando DSL
/plan usa flechas entre pasos
las flechas solo se aceptan dentro de /plan
las lineas vacias se ignoran
los errores se reportan con mensajes en espanol
```

Validacion:

```bash
cargo test dsl::parser
cargo test dsl
make validate-fast
```

#### Validador semantico

La fase 3.4 agrega validacion semantica sobre el `OrchestrationProgram` construido por el parser. El objetivo es detectar programas mal formados antes de conectar el DSL con el nucleo de simulacion.

Reglas minimas:

```text
el programa debe tener exactamente un /goal
/plan no puede estar vacio
/delegate debe tener objetivo y worker
/delegate no puede referenciar un worker vacio
/verify debe aparecer antes de /terminate when verified
los comandos desconocidos conservan un error claro desde el lexer
```

Ejemplo valido:

```text
/goal rescatar_victimas
/plan buscar -> clasificar -> asistir
/delegate sector_a worker_1
/verify checklist_final
/terminate when verified
```

Validacion:

```bash
cargo test dsl::validator
make validate-fast
```

#### Interprete hacia eventos del nucleo

La fase 3.5 agrega una traduccion desde programas DSL validados hacia eventos internos del nucleo. Esta traduccion todavia no ejecuta una simulacion completa. Solo produce una traza tipada que luego podra consumir el scheduler o una capa de replay.

Salida esperada para un flujo minimo:

```text
GoalCreated
PlanStepCreated
DelegationRequested
VerificationRequested
TerminationPolicySet
```

Ejemplo:

```text
/goal rescatar_victimas
/plan buscar -> clasificar -> asistir
/delegate sector_a worker_1
/verify checklist_final
/terminate when verified
```

Validacion:

```bash
cargo test dsl::interpreter
cargo test core
make validate-fast
```

#### Ejecucion desde CLI

La fase 3.6 agrega ejecucion de scripts `.loop` desde el binario nativo. El comando lee el archivo, lo pasa por lexer, parser, validador semantico e interprete, y muestra los eventos DSL generados.

Comando principal:

```bash
cargo run -- --script examples/rescate.loop --seed 123 --ticks 50
```

Ejemplo de script:

```text
/goal rescatar_victimas
/plan buscar -> clasificar -> asistir
/delegate sector_a worker_1
/verify checklist_final
/terminate when verified
```

La opcion `--seed` fija la semilla determinista usada por la corrida de apoyo del nucleo. La opcion `--ticks` define cuantos ticks ejecutar para comprobar que el binario puede arrancar y completar una ejecucion corta.

#### Visor visual del DSL

La fase 4.1 agrega un visor lateral para programas `.loop`. El visor muestra el script cargado, el comando activo y el estado de cada comando.

Comando para validacion por terminal:

```bash
cargo run -- --script examples/rescate.loop --seed 123 --ticks 50
```

Comando para abrir el visor visual:

```bash
cargo run -- --script examples/rescate.loop --visual --seed 123 --ticks 50
```

### Exportacion de grafo JSON

#### Comando

```bash
cargo run -- --script examples/rescate.loop --export-graph artifacts/rescate.graph.json
```

#### Uso

La exportacion genera un archivo JSON con metadatos, nodos, aristas e identificadores estables. Este archivo permite revisar la estructura logica del flujo sin abrir el modo visual.

### Importacion de grafo JSON

#### Comando

```bash
cargo run -- --graph artifacts/rescate.graph.json --seed 123 --ticks 50
```

#### Uso

La importacion carga un grafo JSON exportado, valida su estructura y ejecuta una corrida corta del nucleo determinista para confirmar que el flujo puede inspeccionarse sin abrir el modo visual.

### Roundtrip de grafo JSON

#### Objetivo

El roundtrip garantiza que un script `.loop` pueda convertirse a grafo JSON, importarse de nuevo y conservar ids, nodos, aristas y metadatos estables.

#### Validacion

```bash
cargo test --test dsl_graph_contract
```
