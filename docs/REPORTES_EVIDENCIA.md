### Reportes de evidencia

#### Objetivo

La Fase 8.5 agrega reportes de evidencia para resumir resultados de validacion, auditoria Rust, escaneo de secretos, metricas y replay.

El reporte no ejecuta pruebas pesadas. Solo convierte resultados existentes en archivos legibles.

#### Archivos agregados

```text
scripts/generate_evidence_report.py
.github/workflows/evidence-report.yml
docs/REPORTES_EVIDENCIA.md
```

#### Salidas generadas

```text
artifacts/evidence/reporte-evidencia.md
artifacts/evidence/reporte-evidencia.json
```

#### Uso local

```bash
make evidence-report
```

Tambien puede ejecutarse directamente:

```bash
python3 scripts/generate_evidence_report.py --input-root artifacts --output-dir artifacts/evidence
```

#### Workflow manual

El workflow manual permite generar y publicar el reporte como artifact de GitHub Actions.

```text
.github/workflows/evidence-report.yml
```

#### Politica

El workflow no se ejecuta en cada PR ni en cada push. Tampoco usa secretos, permisos de escritura ni deploy.

#### Evidencias resumidas

```text
validacion profunda manual
auditoria Rust
escaneo de secretos
metricas de fallos recuperables
fallo bizantino simplificado
replay determinista
```

#### Interpretacion

Si una evidencia aparece como pendiente, primero ejecuta el workflow manual correspondiente y luego vuelve a generar el reporte.

Esta fase convierte la Fase 8 en un ciclo mas claro: ejecutar controles manuales, generar resultados y producir evidencia legible.

#### Uso en release controlado

Los reportes de evidencia se usan como soporte previo a release.

Para un release candidate, el reporte debe generarse despues de las validaciones manuales relevantes y antes de crear el tag desde `main`.

El reporte no reemplaza la revision humana. Su funcion es dejar trazabilidad de comandos, archivos y resultados disponibles.

#### Artefacto web como evidencia

El artefacto web manual puede acompañar un reporte de evidencia previo a release.

El reporte de evidencia no versiona `dist`. El directorio `dist` debe tratarse como salida generada o artifact descargable del workflow manual.

### Uso en release candidate

#### Objetivo

Los reportes de evidencia permiten revisar el estado del release candidate antes de crear el tag `v0.9.0-rc1`.

#### Comando

```bash
make evidence-report-release
```

### Uso en revision posterior al release candidate

#### Objetivo

Los reportes de evidencia sirven como entrada para `docs/REVISION_RELEASE_CANDIDATE.md`.

#### Evidencia esperada

La revision posterior al RC debe revisar:

```text
reporte-evidencia.md
reporte-evidencia.json
resultado de validacion profunda
resultado de validacion web
resultado de auditoria Rust
resultado de escaneo de secretos
```

#### Decision

El reporte no decide por si solo si se publica `v0.9.0`. La decision queda documentada en la revision posterior al release candidate.

### Fase 10.1: escenarios comparables

#### Relacion con evidencia

Los escenarios comparables son entradas estables para reportes de evidencia, benchmarks y resultados tecnicos.

No generan evidencia por si mismos. La evidencia se genera cuando se ejecutan desde validacion profunda, benchmarks o scripts de reporte.

### Fase 10.2: benchmarks reproducibles

#### Relacion con evidencia

Los benchmarks generan salidas que pueden adjuntarse a reportes de evidencia posteriores.

Archivos relevantes:

```text
artifacts/benchmarks/resultados.csv
artifacts/benchmarks/resumen.md
```

### Fase 10.3: informe tecnico interno

#### Relacion con evidencia

El informe tecnico interno resume evidencia generada por fases previas y prepara una lectura ordenada de resultados.

Documentos principales:

```text
docs/INFORME_TECNICO.md
docs/RESULTADOS.md
```

### Fase 10.5, evidencia de usabilidad documental

#### Evidencia esperada

- README con ruta de uso.
- Guia de uso.
- Demo guiada.
- Lectura rapida.
- Enlace a Hugging Face Spaces.
- Comandos reproducibles de smoke, escenarios y benchmarks.
