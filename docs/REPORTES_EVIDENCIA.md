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
