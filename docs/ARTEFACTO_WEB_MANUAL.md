### Artefacto web manual

#### Objetivo

Este documento describe el flujo para generar un artefacto web descargable de Loopscape sin publicar GitHub Pages y sin ejecutar despliegues automaticos.

La Fase 9.2 conserva el release controlado iniciado en Fase 9.1. El build web se genera bajo demanda y queda disponible como artifact del workflow.

#### Workflow

El workflow principal es:

```text
.github/workflows/web-build.yml
```

El workflow se ejecuta solo mediante `workflow_dispatch`.

#### Reglas de ejecucion

El workflow mantiene estas reglas:

```text
sin pull_request
sin push
sin secrets
sin deploy automatico
sin permisos de escritura
sin GitHub Pages
```

#### Artefacto generado

El workflow genera el directorio:

```text
dist
```

Luego lo sube como artifact con un nombre basado en la etiqueta indicada al ejecutar el workflow.

Ejemplo de etiqueta:

```text
0.9.0-dev
```

#### Uso manual

Desde GitHub Actions:

```text
Actions
Artefacto web manual
Run workflow
```

Campos recomendados:

```text
version_label: 0.9.0-dev
public_url: ./
```

#### Validacion local equivalente

Para validar localmente sin publicar nada:

```bash
rm -rf dist
make web-build
ls dist
```

Si el proyecto no usa el target `web-build`, la alternativa manual es:

```bash
rustup target add wasm32-unknown-unknown
cargo install trunk --locked
trunk build --release --dist dist --public-url ./
```

#### Relacion con release

Este artefacto no crea tags ni GitHub Releases.

Para un release candidate, primero se debe mergear la fase correspondiente en `main`. El tag se crea despues desde `main` actualizado.

#### Criterio de cierre

La Fase 9.2 queda cerrada cuando:

```text
el workflow web es manual
el workflow sube dist como artifact
no existe publicacion automatica
la documentacion explica el flujo
```

### Uso en release candidate

#### Objetivo

El artefacto web manual debe ejecutarse antes de crear `v0.9.0-rc1` para confirmar que `dist/` puede generarse y descargarse como artifact.

#### Regla

La generacion del artefacto web no publica GitHub Pages y no realiza deploy automatico.
