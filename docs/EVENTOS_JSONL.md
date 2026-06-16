### Eventos JSONL

La Fase 5.1 agrega registro headless de eventos del nucleo en formato JSONL.

Cada linea del archivo representa un evento determinista producido por la simulacion. El formato esta pensado para replay, auditoria, comparacion de ejecuciones y metricas posteriores.

#### Comando principal

```bash
cargo run -- --script examples/rescate.loop --record artifacts/runs/dev/events.jsonl --seed 123 --ticks 50
```

#### Ruta sugerida

```text
artifacts/runs/dev/events.jsonl
```

Para corridas versionadas o de release puede usarse una ruta numerada:

```text
artifacts/runs/run-001/events.jsonl
```

#### Contrato de cada linea

Cada linea JSONL contiene:

```text
formato
secuencia
tick
tipo
evento
```

El campo `formato` debe usar:

```text
loopscape.core.events.v1
```

El campo `secuencia` empieza en cero y aumenta de uno en uno. El campo `tick` coincide con el tick interno del evento. El campo `tipo` usa nombres estables como `TickAdvanced`, `TaskAssigned`, `AgentStateChanged` o `TaskCompleted`.

#### Validacion local

```bash
cargo run -- --script examples/rescate.loop --record artifacts/runs/dev/events.jsonl --seed 123 --ticks 50
test -f artifacts/runs/dev/events.jsonl
cargo test core::trace
cargo test --test core_trace_jsonl
make validate-fast
```

#### Limpieza

Los archivos dentro de `artifacts/runs` son salidas generadas. No deben agregarse al commit salvo que se decida publicar una traza de ejemplo de forma explicita.
