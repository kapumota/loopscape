### Fallos recuperables de workers

#### Objetivo

La Fase 7.2 conecta el supervisor multiagente al runtime de simulacion. El sistema puede simular workers colgados, detectar timeouts y aplicar reinicios controlados sin introducir no determinismo externo.

#### Componentes

```text
RecoverableFailurePlan
WorkerFailureSpec
SupervisorState integrado en SimulationState
CoreEvent::WorkerTimedOut
CoreEvent::WorkerRestarted
CoreEvent::WorkerRestartLimitReached
```

#### Flujo

```text
crear plan de fallo recuperable
inicializar simulacion con supervisor
avanzar ticks
omitir heartbeat del worker colgado
detectar timeout
aplicar politica de reinicio
registrar eventos de recuperacion
exportar metricas CSV con fallos reales
```

#### Ejemplo conceptual

```text
worker 0 se cuelga desde tick 1 durante 3 ticks
el supervisor deja de recibir heartbeat
al superar timeout_ticks se registra WorkerTimedOut
si RestartPolicy lo permite se registra WorkerRestarted
las metricas fallos_detectados y fallos_recuperados dejan de estar en cero
```

#### Determinismo

Los fallos se definen por worker, tick inicial y duracion. No hay reloj de pared, hilos, red ni aleatoriedad externa.

#### Validacion

```bash
cargo fmt
cargo test failure
cargo test supervisor
cargo test --test fallos_recuperables
make validate-fast
git diff --check
```
