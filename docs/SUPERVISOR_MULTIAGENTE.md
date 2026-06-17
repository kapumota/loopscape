### Supervisor multiagente real

#### Objetivo

La Fase 7.1 introduce un supervisor real dentro del nucleo de Loopscape. El supervisor ya no es solo una representacion visual: mantiene estado de workers, recibe heartbeats, detecta timeouts y aplica una politica de reinicio determinista.

#### Componentes

```text
SupervisorState
WorkerState
HeartbeatEvent
WorkerTimeout
RestartPolicy
```

#### Flujo

```text
registrar worker
recibir heartbeat
avanzar tick del supervisor
detectar timeout
aplicar politica de reinicio
registrar eventos de supervisor
calcular metricas de resiliencia
```

#### Politicas iniciales

```text
Never
OnTimeout
```

`Never` detecta el fallo, pero no reinicia el worker. `OnTimeout` permite reinicios controlados hasta un limite fijo.

#### Determinismo

El supervisor no usa red, reloj de pared, hilos ni aleatoriedad externa. El avance depende solo de ticks, heartbeats y politica de reinicio.

#### Validacion

```bash
cargo test supervisor
cargo test --test supervisor_multiagente
make validate-fast
```
