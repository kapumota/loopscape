### Escaneo manual de secretos

#### Objetivo

Esta fase agrega un escaneo manual de secretos para detectar credenciales, tokens, claves privadas y valores sensibles que hayan sido agregados por error al repositorio.

El escaneo no se ejecuta en cada PR ni en cada push. Su objetivo es proteger el repositorio sin ralentizar cambios pequenos.

#### Workflow agregado

El workflow principal es:

```text
.github/workflows/secrets-scan.yml
```

Se ejecuta por:

```text
workflow_dispatch
schedule semanal
revision manual antes de release
```

No se ejecuta por:

```text
pull request
push
deploy automatico
```

#### Politica de permisos

El workflow usa permisos minimos:

```text
contents: read
```

No usa secretos del repositorio, no usa permisos de escritura y no publica artefactos por defecto.

#### Herramienta usada

La herramienta usada es `gitleaks`.

El workflow instala la herramienta durante la ejecucion manual o programada y ejecuta un escaneo sobre el repositorio.

#### Uso manual

Desde GitHub Actions:

```text
Actions
Escaneo manual de secretos
Run workflow
```

Opciones:

```text
historial
revision_actual
revision_manual
revision_release
```

#### Interpretacion

Si `gitleaks` detecta un secreto, la ejecucion falla. En ese caso se debe revisar el hallazgo, retirar el valor sensible y rotar la credencial si fue real.

Si el hallazgo es falso positivo, primero debe documentarse. Una configuracion `.gitleaks.toml` puede agregarse en una fase posterior si los falsos positivos se vuelven repetitivos.

#### Relacion con Fase 8

Este escaneo complementa:

```text
Fase 8.1: auditoria ligera de workflows
Fase 8.2: auditoria Rust manual
Fase 8.3: escaneo manual de secretos
```

La validacion profunda funcional queda separada para no mezclar seguridad de repositorio con pruebas nativas pesadas.

### Relacion con validacion profunda manual

#### Separacion de responsabilidades

El escaneo de secretos y la validacion profunda cumplen objetivos distintos.

El escaneo de secretos revisa credenciales y valores sensibles. La validacion profunda revisa comportamiento funcional, escenarios DSL, metricas y replay.

### Relacion con reportes de evidencia

#### Uso

Los resultados de escaneo de secretos pueden guardarse bajo `artifacts/secrets-scan`.

El reporte de evidencia los detecta y los lista cuando existen.
