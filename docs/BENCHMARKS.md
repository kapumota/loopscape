### Benchmarks reproducibles

#### Objetivo

La Fase 10.2 agrega una base ligera de benchmarks reproducibles para Loopscape.

El objetivo es ejecutar los escenarios comparables de la Fase 10.1 con una semilla fija, un numero de ticks definido y salidas legibles en CSV y Markdown.

Esta fase no convierte el CI normal en un flujo pesado. Los benchmarks son manuales.

#### Entradas

```text
benchmarks/escenarios_comparables.csv
scenarios/react_basic.loop
scenarios/dsl_delegation.loop
scenarios/multiagent_failure.loop
```

#### Ejecucion

```bash
bash scripts/run_benchmarks.sh
```

Por defecto usa modo debug para reducir friccion durante desarrollo.

Para modo release:

```bash
LOOPSCAPE_BENCH_MODE=release bash scripts/run_benchmarks.sh
```

#### Salidas

```text
artifacts/benchmarks/resultados.csv
artifacts/benchmarks/resumen.md
artifacts/benchmarks/salidas/react_basic.txt
artifacts/benchmarks/salidas/dsl_delegation.txt
artifacts/benchmarks/salidas/multiagent_failure.txt
```

#### Columnas del CSV

```text
escenario
ruta
semilla
ticks
estado
duracion_ms
eventos_dsl
eventos_nucleo
tareas_completas
tareas_pendientes
```

#### Politica de versionado

Los resultados de benchmark no se versionan por defecto.

Se versiona solamente:

```text
benchmarks/escenarios_comparables.csv
scripts/run_benchmarks.sh
artifacts/benchmarks/.gitkeep
```

#### Relacion con el release candidate

Los benchmarks reproducibles ayudan a evaluar `v0.9.0-rc1`, pero no sustituyen la validacion profunda manual ni los reportes de evidencia.

### Fase 10.3: informe tecnico interno

#### Relacion con benchmarks

Los benchmarks reproducibles alimentan los documentos:

```text
docs/INFORME_TECNICO.md
docs/RESULTADOS.md
```

Las mediciones deben copiarse desde `artifacts/benchmarks/resultados.csv` cuando se ejecute el script manual.
