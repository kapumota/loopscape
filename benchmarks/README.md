### Benchmarks reproducibles

#### Objetivo

Este directorio contiene la lista de escenarios comparables usados por `scripts/run_benchmarks.sh`.

La Fase 10.2 no agrega un sistema de benchmarking estadistico completo. Agrega una base reproducible para ejecutar los escenarios de la Fase 10.1 con una semilla y un numero de ticks definidos.

#### Archivos

```text
benchmarks/escenarios_comparables.csv
scripts/run_benchmarks.sh
artifacts/benchmarks/.gitkeep
```

#### Uso basico

```bash
bash scripts/run_benchmarks.sh
```

#### Salidas generadas

```text
artifacts/benchmarks/resultados.csv
artifacts/benchmarks/resumen.md
artifacts/benchmarks/salidas/*.txt
```

#### Politica

Los resultados generados no se versionan por defecto. Solo se conserva `.gitkeep` para mantener la carpeta de salida esperada.
