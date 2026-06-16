use serde::{Deserialize, Serialize};

use super::scheduler::SimulationState;
use super::task::TaskStatus;

/// Entrada normalizada para calcular métricas del núcleo.
///
/// La capa visual puede construir esta entrada desde Bevy, pero la fórmula de
/// métricas se mantiene centralizada en el core.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct CoreMetricsInput {
    pub tick: u64,
    pub active_loops: usize,
    pub total_tasks: usize,
    pub completed_tasks: usize,
    pub pending_tasks: usize,
    pub assigned_tasks: usize,
}

/// Métricas derivadas del estado del núcleo.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CoreMetrics {
    pub tick: u64,
    pub active_loops: usize,
    pub total_tasks: usize,
    pub completed_tasks: usize,
    pub pending_tasks: usize,
    pub assigned_tasks: usize,
    pub throughput: f32,
}

impl CoreMetrics {
    pub fn from_state(state: &SimulationState) -> Self {
        let completed_tasks = state
            .tasks
            .iter()
            .filter(|task| task.status == TaskStatus::Completed)
            .count();

        let pending_tasks = state
            .tasks
            .iter()
            .filter(|task| task.status == TaskStatus::Pending)
            .count();

        let assigned_tasks = state
            .tasks
            .iter()
            .filter(|task| task.status == TaskStatus::Assigned)
            .count();

        Self::from_input(CoreMetricsInput {
            tick: state.tick,
            active_loops: state.agents.len(),
            total_tasks: state.tasks.len(),
            completed_tasks,
            pending_tasks,
            assigned_tasks,
        })
    }

    pub fn from_input(input: CoreMetricsInput) -> Self {
        let throughput = if input.tick == 0 {
            0.0
        } else {
            input.completed_tasks as f32 / input.tick as f32
        };

        Self {
            tick: input.tick,
            active_loops: input.active_loops,
            total_tasks: input.total_tasks,
            completed_tasks: input.completed_tasks,
            pending_tasks: input.pending_tasks,
            assigned_tasks: input.assigned_tasks,
            throughput,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{CoreMetrics, CoreMetricsInput};

    #[test]
    fn metrics_from_input_are_deterministic() {
        let input = CoreMetricsInput {
            tick: 10,
            active_loops: 4,
            total_tasks: 8,
            completed_tasks: 3,
            pending_tasks: 2,
            assigned_tasks: 3,
        };

        let first = CoreMetrics::from_input(input.clone());
        let second = CoreMetrics::from_input(input);

        assert_eq!(first, second);
        assert_eq!(first.throughput, 0.3);
    }
}

#[cfg(test)]
mod focused_tests {
    use super::CoreMetrics;
    use crate::core::scheduler::{SimulationConfig, SimulationState};
    use crate::core::task::TaskStatus;

    #[test]
    fn metrics_from_state_counts_task_statuses() {
        let config = SimulationConfig::new(21).with_size(2, 5);
        let mut state = SimulationState::new(config);
        state.run_ticks(10);

        let metrics = CoreMetrics::from_state(&state);
        let completed = state
            .tasks
            .iter()
            .filter(|task| task.status == TaskStatus::Completed)
            .count();
        let assigned = state
            .tasks
            .iter()
            .filter(|task| task.status == TaskStatus::Assigned)
            .count();
        let pending = state
            .tasks
            .iter()
            .filter(|task| task.status == TaskStatus::Pending)
            .count();

        assert_eq!(metrics.completed_tasks, completed);
        assert_eq!(metrics.assigned_tasks, assigned);
        assert_eq!(metrics.pending_tasks, pending);
        assert_eq!(metrics.total_tasks, state.tasks.len());
    }
}

use std::fs::File;
use std::io::Write;
use std::path::Path;

use super::event::CoreEvent;

pub const METRICS_CSV_HEADER: &str = "ticks,tareas_completadas,loops_activos,tokens_usados,fallos_detectados,fallos_recuperados,latencia_promedio";

/// Fila estable para exportar metricas comparables de simulacion.
#[derive(Clone, Debug, PartialEq)]
pub struct SimulationMetricsCsvRow {
    pub ticks: u64,
    pub completed_tasks: usize,
    pub active_loops: usize,
    pub tokens_used: u64,
    pub failures_detected: u64,
    pub failures_recovered: u64,
    pub average_latency: f32,
}

impl SimulationMetricsCsvRow {
    pub fn from_state(state: &SimulationState) -> Self {
        let (assigned_count, assigned_duration_total) = assigned_duration_summary(&state.events);
        let average_latency = if assigned_count == 0 {
            0.0
        } else {
            assigned_duration_total as f32 / assigned_count as f32
        };

        Self {
            ticks: state.tick,
            completed_tasks: state.metrics.completed_tasks,
            active_loops: state.metrics.active_loops,
            tokens_used: 0,
            failures_detected: 0,
            failures_recovered: 0,
            average_latency,
        }
    }

    pub fn to_csv_line(&self) -> String {
        format!(
            "{},{},{},{},{},{},{:.3}",
            self.ticks,
            self.completed_tasks,
            self.active_loops,
            self.tokens_used,
            self.failures_detected,
            self.failures_recovered,
            self.average_latency
        )
    }
}

/// Escribe una fila CSV con metricas comparables de simulacion.
pub fn write_metrics_csv<P: AsRef<Path>>(state: &SimulationState, path: P) -> Result<(), String> {
    let path = path.as_ref();
    if let Some(parent) = path.parent() {
        if !parent.as_os_str().is_empty() {
            std::fs::create_dir_all(parent).map_err(|error| {
                format!("no se pudo crear el directorio de metricas {parent:?}: {error}")
            })?;
        }
    }

    let row = SimulationMetricsCsvRow::from_state(state);
    let mut file = File::create(path)
        .map_err(|error| format!("no se pudo crear el archivo CSV {path:?}: {error}"))?;

    writeln!(file, "{METRICS_CSV_HEADER}")
        .map_err(|error| format!("no se pudo escribir cabecera CSV {path:?}: {error}"))?;
    writeln!(file, "{}", row.to_csv_line())
        .map_err(|error| format!("no se pudo escribir fila CSV {path:?}: {error}"))?;

    Ok(())
}

fn assigned_duration_summary(events: &[CoreEvent]) -> (u64, u64) {
    events
        .iter()
        .fold((0, 0), |(count, total), event| match event {
            CoreEvent::TaskAssigned { duration, .. } => (count + 1, total + u64::from(*duration)),
            _ => (count, total),
        })
}

/// Lee una fila de metricas CSV desde disco y valida la cabecera estable.
pub fn read_metrics_csv<P: AsRef<Path>>(path: P) -> Result<SimulationMetricsCsvRow, String> {
    let path = path.as_ref();
    let content = std::fs::read_to_string(path)
        .map_err(|error| format!("no se pudo leer el archivo CSV {path:?}: {error}"))?;

    let mut lines = content.lines().filter(|line| !line.trim().is_empty());
    let header = lines
        .next()
        .ok_or_else(|| "el archivo CSV no contiene cabecera".to_string())?;

    if header.trim() != METRICS_CSV_HEADER {
        return Err(format!(
            "cabecera CSV invalida: se esperaba {METRICS_CSV_HEADER}, se obtuvo {header}"
        ));
    }

    let row = lines
        .next()
        .ok_or_else(|| "el archivo CSV no contiene fila de metricas".to_string())?;

    if lines.next().is_some() {
        return Err("el archivo CSV contiene mas de una fila de metricas".to_string());
    }

    parse_metrics_csv_row(row)
}

/// Convierte una fila CSV estable en metricas de simulacion.
pub fn parse_metrics_csv_row(row: &str) -> Result<SimulationMetricsCsvRow, String> {
    let columns = row.split(',').collect::<Vec<_>>();
    if columns.len() != 7 {
        return Err(format!(
            "fila CSV invalida: se esperaban 7 columnas y se obtuvieron {}",
            columns.len()
        ));
    }

    Ok(SimulationMetricsCsvRow {
        ticks: parse_column(columns[0], "ticks")?,
        completed_tasks: parse_column(columns[1], "tareas_completadas")?,
        active_loops: parse_column(columns[2], "loops_activos")?,
        tokens_used: parse_column(columns[3], "tokens_usados")?,
        failures_detected: parse_column(columns[4], "fallos_detectados")?,
        failures_recovered: parse_column(columns[5], "fallos_recuperados")?,
        average_latency: parse_column(columns[6], "latencia_promedio")?,
    })
}

fn parse_column<T: std::str::FromStr>(value: &str, name: &str) -> Result<T, String> {
    value
        .trim()
        .parse::<T>()
        .map_err(|_| format!("no se pudo interpretar la columna {name} con valor {value}"))
}
