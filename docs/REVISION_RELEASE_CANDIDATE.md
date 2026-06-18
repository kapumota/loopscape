### Revision posterior al release candidate

#### Objetivo

Este documento define la revision posterior al release candidate `v0.9.0-rc1`.

El objetivo es registrar que se valido, que problemas quedaron abiertos y que criterios faltan antes de pasar a `v0.9.0` estable.

Esta fase no publica un nuevo release. Tampoco crea un nuevo tag. Solo ordena la revision posterior al primer release candidate.

#### Estado esperado

Antes de usar este documento se espera que:

```text
VERSION = 0.9.0-rc1
CHANGELOG.md contiene la entrada 0.9.0-rc1
docs/RELEASE_CANDIDATE.md existe
el tag v0.9.0-rc1 fue creado desde main
```

#### Entradas de revision

La revision debe usar como entradas:

```text
resultado de make validate-full
resultado de make validate-web
reporte de evidencia Markdown
reporte de evidencia JSON
resultado de auditoria Rust manual
resultado de escaneo manual de secretos
artefacto web manual
historial de PRs de Fase 8 y Fase 9
```

#### Checklist de revision

Validar los siguientes puntos:

```text
VERSION contiene 0.9.0-rc1
el tag v0.9.0-rc1 apunta a main
la validacion profunda manual no tiene bloqueantes
la validacion web manual no tiene bloqueantes
la auditoria Rust manual no reporta vulnerabilidades bloqueantes
el escaneo manual de secretos no reporta secretos reales
el reporte de evidencia se genera en Markdown y JSON
el artefacto web manual se genera como artifact y no como GitHub Pages
```

#### Criterios para pasar a v0.9.0 estable

Se puede avanzar hacia `v0.9.0` estable cuando:

```text
no hay fallos bloqueantes en validate-full
no hay fallos bloqueantes en validate-web
no hay secretos reales detectados
no hay vulnerabilidades Rust criticas conocidas sin documentar
la documentacion de release esta completa
el flujo de artifact web manual esta validado
los formatos principales estan documentados
```

#### Criterios para crear v0.9.0-rc2

Se debe crear otro release candidate si aparece alguno de estos casos:

```text
cambio funcional del nucleo
cambio en formato de eventos JSONL
cambio en formato de metricas CSV
cambio en formato de reportes de evidencia
cambio en workflows de seguridad
correccion relevante de validacion o release
```

#### Pendientes permitidos despues de v0.9.0

Estos temas pueden quedar fuera de `v0.9.0` estable:

```text
consenso bizantino completo
sistema distribuido real
publicacion automatica de web
firmado de artefactos
benchmarks grandes
matriz amplia de plataformas
coverage formal
```

#### Comandos sugeridos

```bash
git checkout main
git pull --ff-only origin main
cat VERSION
git tag --list "v0.9.0-rc1"
make validate-full
make validate-web
make evidence-report-release
```

#### Resultado de la revision

Registrar manualmente el resultado de la revision en el PR o en una nota de release.

Formato sugerido:

```text
estado: aprobado | requiere rc2 | bloqueado
validacion funcional: aprobado | revisar
validacion web: aprobado | revisar
auditoria Rust: aprobado | revisar
escaneo de secretos: aprobado | revisar
evidencia: aprobado | revisar
accion siguiente: preparar v0.9.0 | preparar v0.9.0-rc2 | corregir bloqueantes
```

### Fase 10.1: escenarios comparables

#### Relacion con release candidate

Los escenarios comparables ayudan a revisar `v0.9.0-rc1` con entradas DSL estables.

No modifican la version del proyecto ni crean nuevos tags.

### Fase 10.2: benchmarks reproducibles

#### Uso posterior al release candidate

Los benchmarks reproducibles permiten comparar el comportamiento de `v0.9.0-rc1` en escenarios basicos, delegacion DSL y fallos multiagente.

No son criterio unico de aceptacion, pero aportan evidencia para decidir si se mantiene el release candidate o se prepara un nuevo corte.
