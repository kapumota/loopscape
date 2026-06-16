use std::path::Path;

use super::trace::{read_events_jsonl, validate_trace_record, TraceRecord, TRACE_FORMAT_VERSION};

/// Resumen estable de una traza reproducida.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReplaySummary {
    pub format: String,
    pub event_count: usize,
    pub first_tick: u64,
    pub last_tick: u64,
    pub first_event_kind: String,
    pub last_event_kind: String,
}

/// Carga una traza JSONL y valida su reproduccion determinista.
pub fn replay_trace_jsonl<P: AsRef<Path>>(path: P) -> Result<ReplaySummary, String> {
    let records = read_events_jsonl(path)?;
    replay_records(&records)
}

/// Reproduce una traza ya cargada validando el contrato secuencial.
pub fn replay_records(records: &[TraceRecord]) -> Result<ReplaySummary, String> {
    if records.is_empty() {
        return Err("la traza JSONL no contiene eventos para replay".to_string());
    }

    let mut previous_tick = 0;

    for (index, record) in records.iter().enumerate() {
        validate_trace_record(record, index as u64)?;

        if index > 0 && record.tick < previous_tick {
            return Err(format!(
                "la traza no es determinista: el tick retrocede de {} a {} en la secuencia {}",
                previous_tick, record.tick, record.sequence
            ));
        }

        previous_tick = record.tick;
    }

    let first = records
        .first()
        .expect("la traza ya fue validada como no vacia");
    let last = records
        .last()
        .expect("la traza ya fue validada como no vacia");

    Ok(ReplaySummary {
        format: TRACE_FORMAT_VERSION.to_string(),
        event_count: records.len(),
        first_tick: first.tick,
        last_tick: last.tick,
        first_event_kind: first.kind.clone(),
        last_event_kind: last.kind.clone(),
    })
}

#[cfg(test)]
mod tests {
    use super::{replay_records, replay_trace_jsonl};
    use crate::core::event::CoreEvent;
    use crate::core::trace::{
        records_from_events, write_events_jsonl, TraceRecord, TRACE_FORMAT_VERSION,
    };

    #[test]
    fn replay_records_accepts_ordered_trace() {
        let events = vec![
            CoreEvent::TickAdvanced { tick: 1 },
            CoreEvent::TickAdvanced { tick: 2 },
        ];
        let records = records_from_events(&events);

        let summary = replay_records(&records).expect("debe reproducir traza ordenada");

        assert_eq!(summary.event_count, 2);
        assert_eq!(summary.first_tick, 1);
        assert_eq!(summary.last_tick, 2);
        assert_eq!(summary.first_event_kind, "TickAdvanced");
    }

    #[test]
    fn replay_records_rejects_tick_regression() {
        let records = vec![
            TraceRecord {
                format: TRACE_FORMAT_VERSION.to_string(),
                sequence: 0,
                tick: 2,
                kind: "TickAdvanced".to_string(),
                event: CoreEvent::TickAdvanced { tick: 2 },
            },
            TraceRecord {
                format: TRACE_FORMAT_VERSION.to_string(),
                sequence: 1,
                tick: 1,
                kind: "TickAdvanced".to_string(),
                event: CoreEvent::TickAdvanced { tick: 1 },
            },
        ];

        let error = replay_records(&records).expect_err("debe rechazar retroceso de tick");

        assert!(error.contains("tick retrocede"));
    }

    #[test]
    fn replay_trace_jsonl_reads_recorded_events() {
        let path = std::env::temp_dir().join("loopscape-replay-trace.jsonl");
        let events = vec![CoreEvent::TickAdvanced { tick: 1 }];

        write_events_jsonl(&events, &path).expect("debe escribir traza JSONL");
        let summary = replay_trace_jsonl(&path).expect("debe reproducir traza JSONL");
        std::fs::remove_file(&path).ok();

        assert_eq!(summary.event_count, 1);
        assert_eq!(summary.first_tick, 1);
        assert_eq!(summary.last_tick, 1);
    }
}
