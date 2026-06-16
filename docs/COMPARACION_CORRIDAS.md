### Comparacion de corridas

La Fase 5.4 agrega comparacion headless entre dos archivos `metrics.csv` generados por Loopscape.

#### Comando principal

```bash
cargo run -- --compare-metrics artifacts/runs/base/metrics.csv artifacts/runs/dev/metrics.csv
```

#### Exportar reporte de comparacion

```bash
cargo run -- --compare-metrics artifacts/runs/base/metrics.csv artifacts/runs/dev/metrics.csv --compare-output artifacts/runs/dev/comparison.csv
```

#### Flujo recomendado

```bash
cargo run -- --script examples/rescate.loop --metrics artifacts/runs/base/metrics.csv --seed 123 --ticks 50
cargo run -- --script examples/rescate.loop --metrics artifacts/runs/dev/metrics.csv --seed 124 --ticks 50
cargo run -- --compare-metrics artifacts/runs/base/metrics.csv artifacts/runs/dev/metrics.csv --compare-output artifacts/runs/dev/comparison.csv
```

#### Metricas comparadas

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

La comparacion trabaja sobre el resumen CSV de cada corrida. No interpreta todavia trazas JSONL completas ni calcula diferencias evento por evento.

#### Validacion local

```bash
cargo test compare
cargo test --test metrics_comparison
make validate-fast
```

#### Limpieza

Los archivos dentro de `artifacts/runs` son salidas generadas. No deben agregarse al commit por defecto.
