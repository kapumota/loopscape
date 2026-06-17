### Release candidate v0.9.0-rc1

#### Objetivo

Este documento describe el cierre controlado del release candidate `v0.9.0-rc1`.

El objetivo no es publicar un producto final. El objetivo es congelar un punto experimental trazable para revisar validacion, seguridad, evidencia y artefacto web manual.

#### Alcance

El release candidate incluye:

- Simulacion determinista.
- DSL de orquestacion.
- Exportacion e importacion de grafo JSON.
- Eventos JSONL.
- Replay determinista.
- Metricas CSV.
- Comparacion de corridas.
- Supervisor multiagente.
- Fallos recuperables.
- Fallo bizantino simplificado.
- Validacion multiagente.
- Auditoria ligera de workflows.
- Auditoria Rust manual.
- Escaneo manual de secretos.
- Validacion profunda manual.
- Reportes de evidencia.
- Artefacto web manual.

#### Validacion previa

Antes de crear el tag se debe ejecutar de forma manual:

```bash
make validate-full
make validate-web
make evidence-report-release
```

Tambien se recomienda ejecutar desde GitHub Actions:

```text
Rust Security Manual
Secrets Scan Manual
Deep Validation Manual
Evidence Report Manual
Web Artifact Manual
```

#### Regla para el tag

El tag se crea solo despues de mergear el PR en `main` y actualizar la rama local:

```bash
git checkout main
git pull --ff-only origin main
git tag v0.9.0-rc1
git push origin v0.9.0-rc1
```

#### No incluido

Este release candidate no incluye:

- Publicacion automatica en GitHub Pages.
- Deploy automatico.
- Release final estable.
- Garantia de API estable.
- Sistema distribuido real.
- Consenso bizantino completo.

#### Criterio de cierre

La fase queda cerrada cuando:

- `VERSION` contiene `0.9.0-rc1`.
- `CHANGELOG.md` contiene la seccion `0.9.0-rc1`.
- La documentacion de release describe el flujo de tag.
- `make validate-full` y `make validate-web` existen como targets manuales.
- El tag se crea desde `main`, no desde una rama temporal.
