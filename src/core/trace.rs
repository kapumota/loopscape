use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

use serde::{Deserialize, Serialize};

use super::event::CoreEvent;

pub const TRACE_FORMAT_VERSION: &str = "loopscape.core.events.v1";

/// Registro JSONL estable para un evento del nucleo.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TraceRecord {
    #[serde(rename = "formato")]
    pub format: String,
    #[serde(rename = "secuencia")]
    pub sequence: u64,
    pub tick: u64,
    #[serde(rename = "tipo")]
    pub kind: String,
    #[serde(rename = "evento")]
    pub event: CoreEvent,
}

/// Construye registros JSONL a partir de eventos del nucleo.
pub fn records_from_events(events: &[CoreEvent]) -> Vec<TraceRecord> {
    events
        .iter()
        .enumerate()
        .map(|(index, event)| TraceRecord {
            format: TRACE_FORMAT_VERSION.to_string(),
            sequence: index as u64,
            tick: event_tick(event),
            kind: event_kind(event).to_string(),
            event: event.clone(),
        })
        .collect()
}

/// Escribe eventos del nucleo en formato JSONL.
pub fn write_events_jsonl<P: AsRef<Path>>(events: &[CoreEvent], path: P) -> Result<(), String> {
    let path = path.as_ref();
    if let Some(parent) = path.parent() {
        if !parent.as_os_str().is_empty() {
            std::fs::create_dir_all(parent).map_err(|error| {
                format!("no se pudo crear el directorio de trazas {parent:?}: {error}")
            })?;
        }
    }

    let mut file = File::create(path)
        .map_err(|error| format!("no se pudo crear el archivo JSONL {path:?}: {error}"))?;

    for record in records_from_events(events) {
        let line = serde_json::to_string(&record).map_err(|error| {
            format!("no se pudo serializar un evento del nucleo como JSONL: {error}")
        })?;
        writeln!(file, "{line}").map_err(|error| {
            format!("no se pudo escribir un evento en el archivo JSONL {path:?}: {error}")
        })?;
    }

    Ok(())
}

/// Lee registros JSONL previamente exportados.
pub fn read_events_jsonl<P: AsRef<Path>>(path: P) -> Result<Vec<TraceRecord>, String> {
    let path = path.as_ref();
    let file = File::open(path)
        .map_err(|error| format!("no se pudo abrir el archivo JSONL {path:?}: {error}"))?;
    let reader = BufReader::new(file);
    let mut records = Vec::new();

    for (index, line) in reader.lines().enumerate() {
        let line = line.map_err(|error| {
            format!("no se pudo leer la linea {} del JSONL: {error}", index + 1)
        })?;
        if line.trim().is_empty() {
            return Err(format!(
                "la linea {} del JSONL esta vacia y no es valida",
                index + 1
            ));
        }

        let record = serde_json::from_str::<TraceRecord>(&line)
            .map_err(|error| format!("la linea {} del JSONL no es valida: {error}", index + 1))?;
        validate_trace_record(&record, index as u64)?;
        records.push(record);
    }

    if records.is_empty() {
        return Err("la traza JSONL no contiene eventos".to_string());
    }

    Ok(records)
}

/// Valida un registro individual del contrato JSONL.
pub fn validate_trace_record(record: &TraceRecord, expected_sequence: u64) -> Result<(), String> {
    if record.format != TRACE_FORMAT_VERSION {
        return Err(format!("formato de traza no soportado: {}", record.format));
    }

    if record.sequence != expected_sequence {
        return Err(format!(
            "secuencia de evento fuera de contrato: esperada {} recibida {}",
            expected_sequence, record.sequence
        ));
    }

    if record.kind.trim().is_empty() {
        return Err("el tipo de evento esta vacio".to_string());
    }

    if record.tick != event_tick(&record.event) {
        return Err("el tick del registro no coincide con el evento".to_string());
    }

    if record.kind != event_kind(&record.event) {
        return Err("el tipo del registro no coincide con el evento".to_string());
    }

    Ok(())
}

/// Devuelve el tick asociado a un evento del nucleo.
pub fn event_tick(event: &CoreEvent) -> u64 {
    match event {
        CoreEvent::TickAdvanced { tick }
        | CoreEvent::TaskAssigned { tick, .. }
        | CoreEvent::AgentStateChanged { tick, .. }
        | CoreEvent::TaskCompleted { tick, .. }
        | CoreEvent::GoalCreated { tick, .. }
        | CoreEvent::PlanStepCreated { tick, .. }
        | CoreEvent::DelegationRequested { tick, .. }
        | CoreEvent::VerificationRequested { tick, .. }
        | CoreEvent::TerminationPolicySet { tick, .. } => *tick,
    }
}

/// Devuelve el nombre estable del tipo de evento.
pub fn event_kind(event: &CoreEvent) -> &'static str {
    match event {
        CoreEvent::TickAdvanced { .. } => "TickAdvanced",
        CoreEvent::TaskAssigned { .. } => "TaskAssigned",
        CoreEvent::AgentStateChanged { .. } => "AgentStateChanged",
        CoreEvent::TaskCompleted { .. } => "TaskCompleted",
        CoreEvent::GoalCreated { .. } => "GoalCreated",
        CoreEvent::PlanStepCreated { .. } => "PlanStepCreated",
        CoreEvent::DelegationRequested { .. } => "DelegationRequested",
        CoreEvent::VerificationRequested { .. } => "VerificationRequested",
        CoreEvent::TerminationPolicySet { .. } => "TerminationPolicySet",
    }
}

#[cfg(test)]
mod tests {
    use super::{read_events_jsonl, records_from_events, write_events_jsonl, TRACE_FORMAT_VERSION};
    use crate::core::event::CoreEvent;

    #[test]
    fn records_keep_stable_sequence_and_kind() {
        let events = vec![
            CoreEvent::TickAdvanced { tick: 1 },
            CoreEvent::GoalCreated {
                tick: 1,
                goal: "rescatar_victimas".to_string(),
            },
        ];

        let records = records_from_events(&events);

        assert_eq!(records.len(), 2);
        assert_eq!(records[0].format, TRACE_FORMAT_VERSION);
        assert_eq!(records[0].sequence, 0);
        assert_eq!(records[0].tick, 1);
        assert_eq!(records[0].kind, "TickAdvanced");
        assert_eq!(records[1].kind, "GoalCreated");
    }

    #[test]
    fn jsonl_writer_and_reader_roundtrip_events() {
        let path = std::env::temp_dir().join("loopscape-trace-test.jsonl");
        let events = vec![CoreEvent::TickAdvanced { tick: 1 }];

        write_events_jsonl(&events, &path).expect("debe escribir JSONL");
        let records = read_events_jsonl(&path).expect("debe leer JSONL");
        std::fs::remove_file(&path).ok();

        assert_eq!(records.len(), 1);
        assert_eq!(records[0].kind, "TickAdvanced");
    }
}
