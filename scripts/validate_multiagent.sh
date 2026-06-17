#!/usr/bin/env bash
set -euo pipefail

# Validacion multiagente para supervisor, fallos recuperables, fallo bizantino y DSL.
# No usa red ni claves. Los artefactos quedan bajo artifacts/validation.

ARTIFACT_DIR="artifacts/validation/multiagente"
METRICS_FILE="$ARTIFACT_DIR/fallos_metrics.csv"
BYZANTINE_LOG="$ARTIFACT_DIR/byzantine_vote.txt"
DSL_METRICS_FILE="$ARTIFACT_DIR/dsl_fallos_metrics.csv"

mkdir -p "$ARTIFACT_DIR"

printf 'Verificando compilacion multiagente\n'
cargo check --locked --all-targets

printf 'Ejecutando pruebas multiagente\n'
cargo test --locked failure
cargo test --locked byzantine
cargo test --locked dsl
cargo test --locked --test fallos_cli_dsl
cargo test --locked --test validacion_multiagente

printf 'Ejecutando escenario CLI de fallo recuperable\n'
cargo run --locked -- --headless --ticks 12 --agents 3 --tasks 6 \
  --supervisor-timeout 2 \
  --worker-restart-limit 1 \
  --worker-failure 1:3:4 \
  --metrics "$METRICS_FILE"

python3 - <<'PY_CHECK_METRICS'
from pathlib import Path
import csv

path = Path("artifacts/validation/multiagente/fallos_metrics.csv")
if not path.exists():
    raise SystemExit("no se genero metrics.csv de fallos")

with path.open(newline="", encoding="utf-8") as fh:
    rows = list(csv.DictReader(fh))

if not rows:
    raise SystemExit("metrics.csv no contiene filas")

row = rows[-1]
detected = int(row.get("fallos_detectados", "0"))
recovered = int(row.get("fallos_recuperados", "0"))

if detected < 1:
    raise SystemExit("se esperaba al menos un fallo detectado")
if recovered < 1:
    raise SystemExit("se esperaba al menos un fallo recuperado")
if recovered > detected:
    raise SystemExit("los fallos recuperados no pueden superar a los detectados")
PY_CHECK_METRICS

printf 'Ejecutando escenario CLI de votacion bizantina\n'
cargo run --locked -- --byzantine-vote verdadero --agents 3 --byzantine-failure 2:falso \
  > "$BYZANTINE_LOG"

grep -q "Votacion bizantina simplificada" "$BYZANTINE_LOG"
grep -q "Resultado aceptado: true" "$BYZANTINE_LOG"
grep -q "Decision: aceptada" "$BYZANTINE_LOG"

printf 'Ejecutando escenario DSL multiagente\n'
cargo run --locked -- --script examples/fallos_recuperables.loop --seed 123 --ticks 12 \
  --metrics "$DSL_METRICS_FILE"

test -f "$DSL_METRICS_FILE"

printf 'Validacion multiagente completa\n'
