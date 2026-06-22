
### Auditoria Rust manual

#### Objetivo

La auditoria Rust revisa dependencias y avisos de seguridad sin ralentizar cada pull request.

Esta fase separa el CI automatico ligero de la auditoria de seguridad de dependencias. El flujo normal de PR mantiene validaciones rapidas, mientras que la auditoria Rust se ejecuta de forma manual, semanal o antes de preparar un release.

#### Reglas de ejecucion

La auditoria Rust se ejecuta solo en estos casos:

```text
workflow manual
schedule semanal
revision previa a release
```

No se ejecuta en cada PR ni en cada push.

#### Workflow agregado

```text
.github/workflows/rust-security.yml
```

El workflow usa:

```text
workflow_dispatch
schedule semanal
permissions: contents: read
FORCE_JAVASCRIPT_ACTIONS_TO_NODE24=true
persist-credentials: false
sin deploy automatico
sin secretos en PR
sin permisos de escritura
```


#### Politica frente a avisos informativos

`cargo audit` debe fallar ante vulnerabilidades reales reportadas por RustSec. No debe fallar por todos los avisos informativos de dependencias transitivas, porque eso vuelve inestable la auditoria semanal cuando un crate de terceros cambia de estado.

El aviso `RUSTSEC-2024-0436` afecta a `paste 1.0.15` como crate no mantenido. En esta version aparece por dependencias transitivas asociadas al ecosistema grafico usado por Bevy, no por una dependencia directa del proyecto. Por eso se evita `cargo audit --deny warnings` y se deja `cargo audit` como compuerta principal.

Para `cargo deny`, la politica queda documentada en `deny.toml`: los avisos `unmaintained` fallan si afectan dependencias directas del workspace, pero no bloquean la ejecucion por dependencias transitivas.

#### Herramientas disponibles

La entrada manual `herramienta` permite seleccionar:

```text
audit
deny
ambas
```

Por defecto, el schedule semanal ejecuta `cargo audit`.

#### Uso local

Para ejecutar la auditoria principal:

```bash
make audit-rust
```

Para ejecutar cargo audit directamente:

```bash
make audit-rust-audit
```

Para ejecutar cargo deny sobre advisories:

```bash
make audit-rust-deny
```

#### Antes de release

Antes de preparar un release se recomienda ejecutar manualmente el workflow `Auditoria Rust` y revisar el resultado.

Tambien se puede ejecutar localmente:

```bash
make audit-rust
```

#### Alcance

Esta fase cubre avisos de seguridad de dependencias Rust.

No cubre:

```text
analisis estatico pesado
DAST
contenedores
firmas de artefactos
publicacion automatica
revision completa de licencias
```

#### Relacion con Fase 8.1

La Fase 8.1 mantiene el CI automatico liviano. Esta Fase 8.2 agrega auditoria Rust fuera del camino critico de PR.

### Relacion con reportes de evidencia

#### Uso

Los resultados de auditoria Rust pueden guardarse bajo `artifacts/rust-security`.

El reporte de evidencia los detecta y los lista cuando existen.
