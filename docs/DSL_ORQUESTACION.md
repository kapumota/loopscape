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
