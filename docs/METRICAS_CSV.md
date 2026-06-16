### Metricas CSV

La Fase 5.3 agrega exportacion headless de metricas comparables de simulacion en formato CSV.

#### Comando principal

```bash
cargo run -- --script examples/rescate.loop --metrics artifacts/runs/dev/metrics.csv --seed 123 --ticks 50
```

#### Ruta sugerida

```text
artifacts/runs/dev/metrics.csv
```

Para ejecuciones versionadas puede usarse una ruta numerada:

```text
artifacts/runs/run-001/metrics.csv
```

#### Columnas exportadas

```text
ticks
tareas_completadas
loops_activos
tokens_usados
fallos_detectados
fallos_recuperados
latencia_promedio
```

#### Alcance actual

Las columnas `tokens_usados`, `fallos_detectados` y `fallos_recuperados` quedan en cero mientras no exista runtime LLM ni modelo de fallos recuperables.

La columna `latencia_promedio` usa latencia simulada a partir de las duraciones de asignacion de tareas, no tiempo real de pared.

#### Validacion local

```bash
cargo run -- --script examples/rescate.loop --metrics artifacts/runs/dev/metrics.csv --seed 123 --ticks 50
test -f artifacts/runs/dev/metrics.csv
cargo test metrics
cargo test --test simulation_metrics_csv
make validate-fast
```

#### Limpieza

Los archivos dentro de `artifacts/runs` son salidas generadas. No deben agregarse al commit salvo que se publique una corrida de ejemplo de forma explicita.

### Comparacion de metricas

Los archivos `metrics.csv` pueden compararse con `--compare-metrics`.

```bash
cargo run -- --compare-metrics artifacts/runs/base/metrics.csv artifacts/runs/dev/metrics.csv --compare-output artifacts/runs/dev/comparison.csv
```

El reporte de comparacion usa las columnas `metrica`, `base`, `candidata` y `diferencia`.
