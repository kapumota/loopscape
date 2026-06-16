use std::fs::File;
use std::io::Write;
use std::path::Path;

use super::metrics::{read_metrics_csv, SimulationMetricsCsvRow};

pub const METRICS_COMPARISON_CSV_HEADER: &str = "metrica,base,candidata,diferencia";

/// Diferencia estable entre dos corridas de simulacion.
#[derive(Clone, Debug, PartialEq)]
pub struct MetricsDelta {
    pub ticks: i128,
    pub completed_tasks: i128,
    pub active_loops: i128,
    pub tokens_used: i128,
    pub failures_detected: i128,
    pub failures_recovered: i128,
    pub average_latency: f32,
}

/// Comparacion estable entre dos archivos de metricas CSV.
#[derive(Clone, Debug, PartialEq)]
pub struct MetricsComparison {
    pub baseline: SimulationMetricsCsvRow,
    pub candidate: SimulationMetricsCsvRow,
    pub delta: MetricsDelta,
}

pub fn compare_metrics_files<P: AsRef<Path>>(
    baseline_path: P,
    candidate_path: P,
) -> Result<MetricsComparison, String> {
    let baseline = read_metrics_csv(baseline_path)?;
    let candidate = read_metrics_csv(candidate_path)?;
    Ok(compare_metrics_rows(baseline, candidate))
}

pub fn compare_metrics_rows(
    baseline: SimulationMetricsCsvRow,
    candidate: SimulationMetricsCsvRow,
) -> MetricsComparison {
    let delta = MetricsDelta {
        ticks: diff_u64(candidate.ticks, baseline.ticks),
        completed_tasks: diff_usize(candidate.completed_tasks, baseline.completed_tasks),
        active_loops: diff_usize(candidate.active_loops, baseline.active_loops),
        tokens_used: diff_u64(candidate.tokens_used, baseline.tokens_used),
        failures_detected: diff_u64(candidate.failures_detected, baseline.failures_detected),
        failures_recovered: diff_u64(candidate.failures_recovered, baseline.failures_recovered),
        average_latency: candidate.average_latency - baseline.average_latency,
    };

    MetricsComparison {
        baseline,
        candidate,
        delta,
    }
}

pub fn write_metrics_comparison_csv<P: AsRef<Path>>(
    comparison: &MetricsComparison,
    path: P,
) -> Result<(), String> {
    let path = path.as_ref();
    if let Some(parent) = path.parent() {
        if !parent.as_os_str().is_empty() {
            std::fs::create_dir_all(parent).map_err(|error| {
                format!("no se pudo crear el directorio de comparacion {parent:?}: {error}")
            })?;
        }
    }

    let mut file = File::create(path)
        .map_err(|error| format!("no se pudo crear el archivo de comparacion {path:?}: {error}"))?;

    writeln!(file, "{METRICS_COMPARISON_CSV_HEADER}").map_err(|error| {
        format!("no se pudo escribir cabecera de comparacion {path:?}: {error}")
    })?;

    for line in comparison_csv_lines(comparison) {
        writeln!(file, "{line}").map_err(|error| {
            format!("no se pudo escribir fila de comparacion {path:?}: {error}")
        })?;
    }

    Ok(())
}

pub fn comparison_csv_lines(comparison: &MetricsComparison) -> Vec<String> {
    vec![
        format!(
            "ticks,{},{},{}",
            comparison.baseline.ticks, comparison.candidate.ticks, comparison.delta.ticks
        ),
        format!(
            "tareas_completadas,{},{},{}",
            comparison.baseline.completed_tasks,
            comparison.candidate.completed_tasks,
            comparison.delta.completed_tasks
        ),
        format!(
            "loops_activos,{},{},{}",
            comparison.baseline.active_loops,
            comparison.candidate.active_loops,
            comparison.delta.active_loops
        ),
        format!(
            "tokens_usados,{},{},{}",
            comparison.baseline.tokens_used,
            comparison.candidate.tokens_used,
            comparison.delta.tokens_used
        ),
        format!(
            "fallos_detectados,{},{},{}",
            comparison.baseline.failures_detected,
            comparison.candidate.failures_detected,
            comparison.delta.failures_detected
        ),
        format!(
            "fallos_recuperados,{},{},{}",
            comparison.baseline.failures_recovered,
            comparison.candidate.failures_recovered,
            comparison.delta.failures_recovered
        ),
        format!(
            "latencia_promedio,{:.3},{:.3},{:.3}",
            comparison.baseline.average_latency,
            comparison.candidate.average_latency,
            comparison.delta.average_latency
        ),
    ]
}

fn diff_u64(candidate: u64, baseline: u64) -> i128 {
    i128::from(candidate) - i128::from(baseline)
}

fn diff_usize(candidate: usize, baseline: usize) -> i128 {
    candidate as i128 - baseline as i128
}

#[cfg(test)]
mod tests {
    use super::{compare_metrics_rows, comparison_csv_lines};
    use crate::core::metrics::SimulationMetricsCsvRow;

    #[test]
    fn compare_metrics_rows_calculates_deltas() {
        let baseline = SimulationMetricsCsvRow {
            ticks: 10,
            completed_tasks: 2,
            active_loops: 1,
            tokens_used: 0,
            failures_detected: 1,
            failures_recovered: 0,
            average_latency: 2.0,
        };
        let candidate = SimulationMetricsCsvRow {
            ticks: 12,
            completed_tasks: 5,
            active_loops: 1,
            tokens_used: 0,
            failures_detected: 1,
            failures_recovered: 1,
            average_latency: 3.5,
        };

        let comparison = compare_metrics_rows(baseline, candidate);

        assert_eq!(comparison.delta.ticks, 2);
        assert_eq!(comparison.delta.completed_tasks, 3);
        assert_eq!(comparison.delta.failures_recovered, 1);
        assert_eq!(comparison.delta.average_latency, 1.5);
    }

    #[test]
    fn comparison_lines_use_stable_spanish_metric_names() {
        let row = SimulationMetricsCsvRow {
            ticks: 10,
            completed_tasks: 2,
            active_loops: 1,
            tokens_used: 0,
            failures_detected: 0,
            failures_recovered: 0,
            average_latency: 2.0,
        };
        let comparison = compare_metrics_rows(row.clone(), row);
        let lines = comparison_csv_lines(&comparison);

        assert!(lines[0].starts_with("ticks,"));
        assert!(lines[1].starts_with("tareas_completadas,"));
        assert!(lines[6].starts_with("latencia_promedio,"));
    }
}
