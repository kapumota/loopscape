### Resultados preliminares de Loopscape

#### Proposito

Este documento registra resultados preliminares de escenarios comparables y benchmarks reproducibles.

No contiene cifras inventadas. Las mediciones deben copiarse desde una ejecucion local de los benchmarks.

#### Como generar resultados

Ejecuta:

```bash
bash scripts/run_benchmarks.sh
```

Para modo release:

```bash
LOOPSCAPE_BENCH_MODE=release bash scripts/run_benchmarks.sh
```

#### Archivos generados

```text
artifacts/benchmarks/resultados.csv
artifacts/benchmarks/resumen.md
artifacts/benchmarks/salidas/react_basic.txt
artifacts/benchmarks/salidas/dsl_delegation.txt
artifacts/benchmarks/salidas/multiagent_failure.txt
```

#### Escenarios evaluados

```text
react_basic
scenarios/react_basic.loop

.dsl_delegation
scenarios/dsl_delegation.loop

multiagent_failure
scenarios/multiagent_failure.loop
```

#### Columnas de resultados

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

#### Tabla para completar

Completa esta tabla despues de ejecutar `scripts/run_benchmarks.sh`.

```text
escenario             estado    duracion_ms    eventos_dsl    eventos_nucleo    tareas_completas    tareas_pendientes
react_basic           pendiente pendiente      pendiente      pendiente         pendiente           pendiente
dsl_delegation        pendiente pendiente      pendiente      pendiente         pendiente           pendiente
multiagent_failure    pendiente pendiente      pendiente      pendiente         pendiente           pendiente
```

#### Interpretacion esperada

El escenario `react_basic` debe servir como linea base pequena.

El escenario `dsl_delegation` debe reflejar el costo de interpretar y ejecutar delegacion desde DSL.

El escenario `multiagent_failure` debe reflejar el comportamiento de recuperacion ante fallos definidos por escenario.

#### Criterios de lectura

Un resultado es util si declara:

```text
version del proyecto
modo de ejecucion
sistema operativo
comando usado
semilla
ticks
fecha de corrida
```

#### Limitaciones de resultados

Las mediciones de esta fase son preliminares.

No deben presentarse como evidencia de rendimiento industrial porque aun faltan:

```text
multiples corridas
intervalos de confianza
medicion de memoria
matriz de plataformas
escenarios grandes
comparacion contra linea base externa
```

#### Resultado de cierre

El resultado esperado de la Fase 10.3 es un informe tecnico interno y una plantilla clara para registrar resultados reproducibles sin versionar artefactos generados.

### Fase 10.4: lectura de resultados desde README

#### Relacion con resultados

El README ahora enlaza los documentos de escenarios, benchmarks, informe tecnico y resultados preliminares. Esto permite que la lectura de resultados sea visible desde la entrada principal del repositorio.
