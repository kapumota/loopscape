use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

fn binary_path() -> &'static str {
    env!("CARGO_BIN_EXE_loopscape")
}

fn temp_metrics_path(name: &str) -> PathBuf {
    let mut path = std::env::temp_dir();
    path.push(format!(
        "loopscape_{name}_{}_metrics.csv",
        std::process::id()
    ));
    let _ = fs::remove_file(&path);
    path
}

fn read_last_metric_row(path: &PathBuf) -> HashMap<String, String> {
    let content = fs::read_to_string(path).expect("debe leer metrics.csv");
    let mut lines = content.lines();
    let headers = lines
        .next()
        .expect("debe tener encabezado")
        .split(',')
        .map(str::to_string)
        .collect::<Vec<_>>();
    let values = lines
        .last()
        .expect("debe tener al menos una fila")
        .split(',')
        .map(str::to_string)
        .collect::<Vec<_>>();

    headers.into_iter().zip(values).collect()
}

#[test]
fn cli_recoverable_failure_exports_consistent_metrics() {
    let metrics_path = temp_metrics_path("fallos_recuperables");
    let output = Command::new(binary_path())
        .args([
            "--headless",
            "--ticks",
            "12",
            "--agents",
            "3",
            "--tasks",
            "6",
            "--supervisor-timeout",
            "2",
            "--worker-restart-limit",
            "1",
            "--worker-failure",
            "1:3:4",
            "--metrics",
            metrics_path.to_str().expect("ruta valida"),
        ])
        .output()
        .expect("debe ejecutar loopscape");

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let row = read_last_metric_row(&metrics_path);
    let detected = row["fallos_detectados"]
        .parse::<u64>()
        .expect("fallos_detectados numerico");
    let recovered = row["fallos_recuperados"]
        .parse::<u64>()
        .expect("fallos_recuperados numerico");

    assert!(detected >= 1);
    assert!(recovered >= 1);
    assert!(recovered <= detected);

    let _ = fs::remove_file(metrics_path);
}

#[test]
fn cli_byzantine_vote_reports_accepted_majority() {
    let output = Command::new(binary_path())
        .args([
            "--byzantine-vote",
            "verdadero",
            "--agents",
            "3",
            "--byzantine-failure",
            "2:falso",
        ])
        .output()
        .expect("debe ejecutar loopscape");

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Votacion bizantina simplificada"));
    assert!(stdout.contains("Resultado aceptado: true"));
    assert!(stdout.contains("Decision: aceptada"));
}
