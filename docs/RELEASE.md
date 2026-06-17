### Release controlado

#### Objetivo

Este documento define el flujo de release controlado de Loopscape.

El objetivo es preparar releases trazables sin publicar web automaticamente, sin usar secretos en PR y sin mezclar validaciones pesadas con el CI liviano.

#### Version actual

La version de desarrollo se declara en `VERSION`.

```text
0.9.0-dev
```

La version `0.9.0-dev` prepara el camino hacia el primer release candidate `v0.9.0-rc1`.

#### Politica de publicacion

Loopscape no publica GitHub Pages automaticamente.

Los artefactos web se deben generar como artifacts manuales. La publicacion externa, si existe, debe ser una decision separada y revisada.

#### Flujo de release candidate

El flujo recomendado es:

```bash
git checkout main
git pull --ff-only origin main
git checkout -b fase-9-3-release-candidate
```

Luego se actualiza `VERSION`, `CHANGELOG.md` y la documentacion de release.

El tag se crea solo despues del merge a `main`:

```bash
git checkout main
git pull --ff-only origin main
git tag v0.9.0-rc1
git push origin v0.9.0-rc1
```

#### Validacion previa a release

Antes de preparar un release candidate se recomienda ejecutar manualmente:

```bash
make validate-full
make validate-web
make evidence-report-release
```

Si una validacion depende de red o de herramientas externas, debe ejecutarse manualmente y quedar documentada en el reporte de evidencia.

#### Artefactos esperados

Los artefactos esperados para una revision previa a release son:

```text
artifacts/evidence/reporte-evidencia.md
artifacts/evidence/reporte-evidencia.json
dist o artefacto web manual
```

Estos archivos no se versionan por defecto. Deben generarse como evidencia de una corrida o como artifact de workflow manual.

#### Reglas de seguridad

El proceso de release debe mantener estas reglas:

```text
sin deploy automatico
sin secrets en PR
sin pull_request_target
sin permisos de escritura salvo cuando una fase futura lo justifique
sin tags desde ramas no mergeadas
```

#### Criterio de cierre de Fase 9.1

La Fase 9.1 queda cerrada cuando existen:

```text
VERSION
CHANGELOG.md
docs/RELEASE.md
```

Tambien debe quedar documentado que el release candidate se crea en una fase posterior y que el tag se genera desde `main`.

#### Artefacto web manual

La Fase 9.2 agrega un workflow manual para generar `dist` como artifact descargable.

Este flujo no publica GitHub Pages y no ejecuta deploy automatico.

El artefacto web se usa como evidencia previa a release o como revision manual antes de crear un release candidate.

### Fase 9.3: release candidate v0.9.0-rc1

#### Objetivo

Cerrar el primer release candidate experimental de Loopscape sin publicar web automaticamente.

#### Validacion manual

Antes de crear el tag se debe ejecutar:

```bash
make validate-full
make validate-web
make evidence-report-release
```

#### Creacion del tag

El tag `v0.9.0-rc1` se crea solo desde `main` despues del merge:

```bash
git checkout main
git pull --ff-only origin main
git tag v0.9.0-rc1
git push origin v0.9.0-rc1
```
