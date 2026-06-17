### Validacion profunda manual

#### Objetivo

Esta fase agrega una validacion funcional profunda que se ejecuta solo cuando el mantenedor la solicita.

No se ejecuta en cada PR ni en cada push. Su objetivo es validar escenarios pesados sin ralentizar cambios pequenos.

#### Workflow agregado

```text
.github/workflows/deep-validation.yml
```

#### Ejecucion

El workflow se ejecuta mediante:

```text
workflow_dispatch
```

No se ejecuta por:

```text
pull request
push
deploy automatico
```

#### Cobertura

La validacion profunda ejecuta:

```text
make validate-fast
make validate-multiagent
escenario DSL con fallos recuperables
escenario de fallo bizantino simplificado
validacion de metricas de fallos
validacion de replay determinista
```

#### Politica de seguridad

El workflow usa permisos minimos:

```text
contents: read
```

No usa secretos del repositorio, no usa permisos de escritura y no hace deploy.

#### Artefactos locales de ejecucion

Durante la corrida se generan archivos bajo:

```text
artifacts/manual-deep-validation
```

Estos archivos son evidencia local dentro de la corrida. No se publican por defecto para evitar agregar pasos innecesarios y mantener el workflow simple.

#### Relacion con Fase 8

La Fase 8 separa los controles asi:

```text
Fase 8.1: auditoria ligera de workflows
Fase 8.2: auditoria Rust manual
Fase 8.3: escaneo manual de secretos
Fase 8.4: validacion profunda manual
```

Esta fase no reemplaza los controles de seguridad. Los complementa con una puerta funcional manual para cambios grandes o revisiones antes de release.

### Relacion con reportes de evidencia

#### Uso

La validacion profunda manual genera insumos que pueden resumirse con el reporte de evidencia.

El reporte permite revisar metricas, replay y resultados funcionales sin volver a ejecutar pruebas pesadas.
