
### Replay determinista

El replay determinista permite cargar una traza JSONL grabada previamente y validar que mantiene el contrato secuencial de eventos del núcleo.

#### Comando principal

```bash
cargo run -- --replay artifacts/runs/dev/events.jsonl
```

#### Flujo recomendado

Primero se graba una ejecución:

```bash
cargo run -- --script examples/rescate.loop --record artifacts/runs/dev/events.jsonl --seed 123 --ticks 50
```

Luego se reproduce la traza:

```bash
cargo run -- --replay artifacts/runs/dev/events.jsonl
```

#### Contrato validado

El replay verifica:

```text
formato JSONL soportado
secuencia estable de eventos
tick monotono sin retrocesos
tipo de evento consistente
evento interno consistente con el registro
```

#### Alcance

Esta fase no ejecuta todavía una simulación inversa ni reconstruye estados visuales. Su objetivo es validar que una traza grabada puede leerse, verificarse y resumirse de forma reproducible en modo headless.
