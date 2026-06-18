#!/usr/bin/env bash
set -euo pipefail

# Ejecuta escenarios comparables y guarda resultados reproducibles.
# Este script es manual y no forma parte del CI automatico de PR pequeños.

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
INPUT_FILE="${ROOT_DIR}/benchmarks/escenarios_comparables.csv"
OUTPUT_DIR="${ROOT_DIR}/artifacts/benchmarks"
OUTPUT_RUNS_DIR="${OUTPUT_DIR}/salidas"
RESULTS_CSV="${OUTPUT_DIR}/resultados.csv"
SUMMARY_MD="${OUTPUT_DIR}/resumen.md"
CARGO_MODE="${LOOPSCAPE_BENCH_MODE:-debug}"

mkdir -p "${OUTPUT_RUNS_DIR}"

if [[ ! -f "${INPUT_FILE}" ]]; then
  echo "No se encontro ${INPUT_FILE}"
  exit 1
fi

extract_metric() {
  local file_path="$1"
  local label="$2"

  grep -E "^${label}:" "${file_path}" | head -n 1 | sed "s/^${label}: *//" || true
}

run_case() {
  local name="$1"
  local scenario_path="$2"
  local seed="$3"
  local ticks="$4"
  local output_file="${OUTPUT_RUNS_DIR}/${name}.txt"
  local start_ms
  local end_ms
  local duration_ms
  local status="ok"

  echo "Ejecutando benchmark ${name}"

  start_ms="$(date +%s%3N)"

  if [[ "${CARGO_MODE}" == "release" ]]; then
    if ! cargo run --release --quiet -- --script "${scenario_path}" --seed "${seed}" --ticks "${ticks}" > "${output_file}" 2>&1; then
      status="fallo"
    fi
  else
    if ! cargo run --quiet -- --script "${scenario_path}" --seed "${seed}" --ticks "${ticks}" > "${output_file}" 2>&1; then
      status="fallo"
    fi
  fi

  end_ms="$(date +%s%3N)"
  duration_ms="$((end_ms - start_ms))"

  local dsl_events
  local core_events
  local completed_tasks
  local pending_tasks

  dsl_events="$(extract_metric "${output_file}" "Eventos DSL generados")"
  core_events="$(extract_metric "${output_file}" "Eventos del nucleo generados")"
  completed_tasks="$(extract_metric "${output_file}" "Tareas completas")"
  pending_tasks="$(extract_metric "${output_file}" "Tareas pendientes")"

  printf '%s,%s,%s,%s,%s,%s,%s,%s,%s,%s\n' \
    "${name}" \
    "${scenario_path}" \
    "${seed}" \
    "${ticks}" \
    "${status}" \
    "${duration_ms}" \
    "${dsl_events:-NA}" \
    "${core_events:-NA}" \
    "${completed_tasks:-NA}" \
    "${pending_tasks:-NA}" >> "${RESULTS_CSV}"

  if [[ "${status}" != "ok" ]]; then
    echo "Benchmark ${name} fallo. Revisa ${output_file}"
    return 1
  fi
}

cat > "${RESULTS_CSV}" <<'CSV'
escenario,ruta,semilla,ticks,estado,duracion_ms,eventos_dsl,eventos_nucleo,tareas_completas,tareas_pendientes
CSV

had_error=0
while IFS=, read -r name scenario_path seed ticks; do
  if [[ "${name}" == "nombre" ]]; then
    continue
  fi

  if [[ -z "${name}" ]]; then
    continue
  fi

  if ! run_case "${name}" "${scenario_path}" "${seed}" "${ticks}"; then
    had_error=1
  fi
done < "${INPUT_FILE}"

{
  echo "### Resumen de benchmarks reproducibles"
  echo
  echo "#### Configuracion"
  echo
  echo "Modo cargo: ${CARGO_MODE}"
  echo
  echo "#### Resultados"
  echo
  echo '```text'
  cat "${RESULTS_CSV}"
  echo '```'
  echo
  echo "#### Archivos"
  echo
  echo '```text'
  echo "${RESULTS_CSV#${ROOT_DIR}/}"
  echo "${SUMMARY_MD#${ROOT_DIR}/}"
  echo "artifacts/benchmarks/salidas/"
  echo '```'
} > "${SUMMARY_MD}"

echo "Benchmarks reproducibles completados"
echo "${RESULTS_CSV}"
echo "${SUMMARY_MD}"

exit "${had_error}"
