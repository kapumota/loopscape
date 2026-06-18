### Escenarios comparables

#### Objetivo

Este documento describe los escenarios comparables de Loopscape.

La Fase 10.1 agrega escenarios DSL pequeños y reproducibles para comparar ejecuciones basicas, delegacion DSL y fallos multiagente.

Estos escenarios no reemplazan la validacion profunda manual. Sirven como base estable para pruebas, benchmarks y resultados tecnicos.

#### Escenarios incluidos

```text
scenarios/react_basic.loop
scenarios/dsl_delegation.loop
scenarios/multiagent_failure.loop
```

#### Escenario react_basic

Este escenario representa un flujo simple de reaccion.

Cubre:

```text
objetivo unico
plan secuencial
delegacion simple
verificacion final
terminacion verificada
```

Uso sugerido:

```bash
cargo run -- --script scenarios/react_basic.loop --seed 123 --ticks 12
```

#### Escenario dsl_delegation

Este escenario representa una orquestacion con varias delegaciones.

Cubre:

```text
objetivo de coordinacion
plan de preparacion, asignacion, integracion y validacion
workers diferenciados
verificacion de delegacion
terminacion verificada
```

Uso sugerido:

```bash
cargo run -- --script scenarios/dsl_delegation.loop --seed 123 --ticks 12
```

#### Escenario multiagent_failure

Este escenario representa un flujo multiagente con fallo recuperable y fallo bizantino simplificado.

Cubre:

```text
worker colgado
fallo recuperable
respuesta falsa
votacion bizantina simplificada
verificacion de recuperacion y votacion
```

Uso sugerido:

```bash
cargo run -- --script scenarios/multiagent_failure.loop --seed 123 --ticks 12
```

#### Criterios de comparacion

Los escenarios se pueden comparar usando:

```text
cantidad de comandos DSL
cantidad de eventos interpretados
cantidad de nodos del grafo
cantidad de aristas del grafo
presencia o ausencia de fallos
presencia o ausencia de votacion bizantina
```

#### Validacion

La validacion especifica de esta fase es:

```bash
cargo test --test escenarios_comparables
```

Validacion ligera adicional:

```bash
cargo fmt
cargo metadata --locked --format-version 1 --no-deps > /dev/null
git diff --check
```

#### Uso posterior

Estos escenarios quedan listos para:

```text
Fase 10.2: benchmarks reproducibles
Fase 10.3: informe tecnico interno
Fase 10.4: badges e indice final
```
