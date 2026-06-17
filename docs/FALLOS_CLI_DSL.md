### Fallos por CLI y DSL

#### Objetivo

La Fase 7.4 expone fallos recuperables y fallos bizantinos simplificados desde la linea de comandos y desde el DSL.

#### CLI

Los fallos recuperables usan el formato `worker:tick_inicio:duracion`.

```bash
cargo run -- --headless --ticks 12 --agents 3 --tasks 6 \
  --supervisor-timeout 2 \
  --worker-restart-limit 1 \
  --worker-failure 1:3:4 \
  --metrics artifacts/runs/fallos/metrics.csv
```

Los fallos bizantinos usan el formato `worker:valor_falso`.

```bash
cargo run -- --byzantine-vote verdadero --agents 3 --byzantine-failure 2:falso
```

#### DSL

El DSL acepta comandos declarativos de fallos.

```text
/goal rescatar_victimas
/plan buscar -> clasificar
/worker-failure 1 4 3
/byzantine-failure 2 falso
/byzantine-vote verdadero
/verify checklist_final
/terminate when verified
```

#### Limpieza de warnings

Esta fase usa las APIs de supervisor, fallos recuperables y fallo bizantino desde rutas reales de ejecucion. La meta es reducir warnings `dead_code` sin ocultarlos con atributos artificiales.

#### Validacion

```bash
cargo fmt
cargo test failure
cargo test byzantine
cargo test dsl
cargo test --test fallos_cli_dsl
make validate-fast
git diff --check
```

### Validacion endurecida

#### Comando recomendado

```bash
make validate-multiagent
```

#### Proposito

Este comando ejecuta pruebas, escenarios CLI y escenarios DSL para asegurar que la exposicion de fallos se mantenga estable antes de avanzar a la Fase 8.
