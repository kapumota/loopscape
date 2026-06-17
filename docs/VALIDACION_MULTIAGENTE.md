
### Validacion multiagente

Este documento describe la validacion endurecida del bloque multiagente de Loopscape.

#### Objetivo

La validacion multiagente verifica que el supervisor, los fallos recuperables, el fallo bizantino simplificado, el DSL y la CLI funcionen juntos sin depender de red ni claves.

#### Alcance

La validacion cubre:

- compilacion con `cargo check --locked --all-targets`;
- pruebas de fallos recuperables;
- pruebas de fallo bizantino simplificado;
- pruebas del DSL;
- pruebas de exposicion CLI y DSL;
- ejecucion CLI con worker colgado;
- exportacion de metricas CSV;
- verificacion de `fallos_detectados` y `fallos_recuperados`;
- ejecucion CLI de votacion bizantina.

#### Comando principal

```bash
make validate-multiagent
```

#### Artefactos locales

La validacion genera artefactos temporales en:

```text
artifacts/validation/multiagente
```

Estos artefactos no deben commitearse.

#### Escenario de fallo recuperable

```bash
cargo run --locked -- --headless --ticks 12 --agents 3 --tasks 6   --supervisor-timeout 2   --worker-restart-limit 1   --worker-failure 1:3:4   --metrics artifacts/validation/multiagente/fallos_metrics.csv
```

#### Escenario bizantino simplificado

```bash
cargo run --locked -- --byzantine-vote verdadero --agents 3 --byzantine-failure 2:falso
```

#### Escenario DSL

```bash
cargo run --locked -- --script examples/fallos_recuperables.loop --seed 123 --ticks 12   --metrics artifacts/validation/multiagente/dsl_fallos_metrics.csv
```

#### Criterio de aceptacion

La fase se considera valida si:

- `cargo check --locked --all-targets` pasa;
- las pruebas de fallos recuperables pasan;
- las pruebas de fallo bizantino pasan;
- las pruebas DSL pasan;
- `tests/validacion_multiagente.rs` pasa;
- la metrica `fallos_detectados` es mayor o igual que uno en el escenario con worker colgado;
- la metrica `fallos_recuperados` es mayor o igual que uno en el escenario recuperable;
- los fallos recuperados no superan a los fallos detectados;
- la votacion bizantina reporta mayoria aceptada.

#### Preparacion para Fase 8

Esta fase deja el bloque multiagente listo para una siguiente etapa centrada en reportes, dashboards, explicabilidad o suites de escenarios.

### Relación con CI profesional ligero

#### Uso recomendado

La validación multiagente se mantiene disponible mediante `make validate-multiagent`.

La auditoría de workflows de Fase 8.1 no ejecuta esta validación pesada por defecto en cada pull request. Esto conserva un CI ligero y permite ejecutar la validación multiagente cuando se cambia el núcleo, el DSL o los escenarios.


### Relacion con auditoria Rust

#### Separacion de responsabilidades

La validacion multiagente comprueba comportamiento funcional. La auditoria Rust comprueba avisos de seguridad de dependencias.

Ambas validaciones son manuales o programadas, pero no forman parte obligatoria del CI automatico de cada PR.

### Integracion con validacion profunda manual

#### Uso en Fase 8.4

La validacion multiagente queda integrada en el workflow manual de validacion profunda.

El objetivo es mantener el CI automatico liviano y reservar las pruebas pesadas para ejecuciones manuales.
