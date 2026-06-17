#!/usr/bin/env python3
import argparse
import csv
import json
from datetime import datetime, timezone
from pathlib import Path
from typing import Any


# Este script resume evidencias ya generadas por validaciones manuales.
def read_optional_text(path: Path, limit: int = 4000) -> str | None:
    if not path.exists() or not path.is_file():
        return None
    text = path.read_text(encoding="utf-8", errors="replace")
    return text[:limit]


# Lee la ultima fila de metricas cuando existe un CSV compatible.
def read_last_metrics_row(path: Path) -> dict[str, str] | None:
    if not path.exists() or not path.is_file():
        return None
    with path.open(newline="", encoding="utf-8") as handle:
        rows = list(csv.DictReader(handle))
    if not rows:
        return None
    return dict(rows[-1])


# Busca archivos por nombre dentro del arbol de artefactos.
def find_files(root: Path, patterns: list[str]) -> list[str]:
    results: list[str] = []
    if not root.exists():
        return results
    for pattern in patterns:
        for path in sorted(root.glob(pattern)):
            if path.is_file():
                results.append(path.as_posix())
    return results


# Devuelve estado resumido de una evidencia esperada.
def evidence_status(path: Path) -> dict[str, Any]:
    return {
        "ruta": path.as_posix(),
        "presente": path.exists(),
        "tamano_bytes": path.stat().st_size if path.exists() and path.is_file() else 0,
    }


# Construye el modelo JSON del reporte.
def build_report(input_root: Path, motivo: str, release_candidate: str | None) -> dict[str, Any]:
    deep_root = input_root / "manual-deep-validation"
    rust_root = input_root / "rust-security"
    secrets_root = input_root / "secrets-scan"

    metrics_path = deep_root / "fallos_recuperables_metrics.csv"
    replay_path = deep_root / "replay.txt"
    byzantine_path = deep_root / "fallo_bizantino.txt"
    deep_summary_path = deep_root / "resumen.txt"

    return {
        "generado_en": datetime.now(timezone.utc).isoformat(),
        "motivo": motivo,
        "release_candidate": release_candidate or "no especificado",
        "entradas": {
            "validacion_profunda": evidence_status(deep_summary_path),
            "metricas_fallos_recuperables": evidence_status(metrics_path),
            "replay_determinista": evidence_status(replay_path),
            "fallo_bizantino": evidence_status(byzantine_path),
            "auditoria_rust": find_files(rust_root, ["*.txt", "*.json", "*.sarif"]),
            "escaneo_secretos": find_files(secrets_root, ["*.txt", "*.json", "*.sarif"]),
        },
        "metricas": read_last_metrics_row(metrics_path),
        "extractos": {
            "validacion_profunda": read_optional_text(deep_summary_path),
            "replay_determinista": read_optional_text(replay_path),
            "fallo_bizantino": read_optional_text(byzantine_path),
        },
    }


# Escribe un reporte Markdown legible para revision humana.
def write_markdown(report: dict[str, Any], output_path: Path) -> None:
    lines: list[str] = []
    lines.append("### Reporte de evidencia")
    lines.append("")
    lines.append("#### Resumen")
    lines.append("")
    lines.append(f"Generado en UTC: `{report['generado_en']}`")
    lines.append("")
    lines.append(f"Motivo: `{report['motivo']}`")
    lines.append("")
    lines.append(f"Release candidate: `{report['release_candidate']}`")
    lines.append("")

    lines.append("#### Evidencias esperadas")
    lines.append("")
    for name, value in report["entradas"].items():
        if isinstance(value, dict):
            status = "presente" if value["presente"] else "pendiente"
            lines.append(f"- {name}: {status}, ruta `{value['ruta']}`")
        else:
            status = "presente" if value else "pendiente"
            lines.append(f"- {name}: {status}")
            for item in value:
                lines.append(f"  - `{item}`")
    lines.append("")

    lines.append("#### Metricas de fallos recuperables")
    lines.append("")
    metrics = report.get("metricas")
    if metrics:
        for key in sorted(metrics):
            lines.append(f"- {key}: `{metrics[key]}`")
    else:
        lines.append("No se encontraron metricas de fallos recuperables.")
    lines.append("")

    lines.append("#### Extractos")
    lines.append("")
    for name, text in report["extractos"].items():
        lines.append(f"#### {name}")
        lines.append("")
        if text:
            lines.append("```text")
            lines.append(text.strip())
            lines.append("```")
        else:
            lines.append("Evidencia no disponible.")
        lines.append("")

    lines.append("#### Interpretacion")
    lines.append("")
    lines.append("Este reporte no ejecuta validaciones pesadas. Solo resume evidencias ya generadas por workflows manuales o ejecuciones locales.")
    lines.append("")
    lines.append("Si una evidencia aparece como pendiente, primero ejecuta el workflow manual correspondiente y vuelve a generar el reporte.")

    output_path.write_text("\n".join(lines).rstrip() + "\n", encoding="utf-8")


# Escribe la version JSON para consumo automatico.
def write_json(report: dict[str, Any], output_path: Path) -> None:
    output_path.write_text(json.dumps(report, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description="Genera reportes de evidencia de Loopscape")
    parser.add_argument("--input-root", default="artifacts", help="Directorio raiz de artefactos")
    parser.add_argument("--output-dir", default="artifacts/evidence", help="Directorio de salida")
    parser.add_argument("--motivo", default="revision_manual", help="Motivo de generacion")
    parser.add_argument("--release-candidate", default=None, help="Identificador de release candidate")
    return parser.parse_args()


def main() -> None:
    args = parse_args()
    input_root = Path(args.input_root)
    output_dir = Path(args.output_dir)
    output_dir.mkdir(parents=True, exist_ok=True)

    report = build_report(input_root, args.motivo, args.release_candidate)
    write_markdown(report, output_dir / "reporte-evidencia.md")
    write_json(report, output_dir / "reporte-evidencia.json")

    print("Reporte de evidencia generado")
    print((output_dir / "reporte-evidencia.md").as_posix())
    print((output_dir / "reporte-evidencia.json").as_posix())


if __name__ == "__main__":
    main()
