### Indice final de Loopscape

#### Proposito

Este documento organiza la documentacion principal del proyecto antes del cierre experimental posterior a `v0.9.0-rc1`.

#### Lectura recomendada

1. `README.md`: entrada publica del repositorio.
2. `docs/ARQUITECTURA.md`: arquitectura general.
3. `docs/PLAN_FASES_AVANZADO.md`: evolucion por fases.
4. `docs/VALIDACION_POR_NIVELES.md`: niveles de validacion.
5. `docs/ESCENARIOS_COMPARABLES.md`: escenarios estables de investigacion.
6. `docs/BENCHMARKS.md`: benchmarks reproducibles.
7. `docs/INFORME_TECNICO.md`: informe tecnico interno.
8. `docs/RESULTADOS.md`: resultados preliminares.
9. `docs/RELEASE.md`: proceso de release.
10. `docs/REVISION_RELEASE_CANDIDATE.md`: revision posterior al release candidate.
11. `docs/HF_SPACES_DEMO.md`: preparacion de demo web para Hugging Face Spaces.

#### Entradas de investigacion

```text
scenarios/react_basic.loop
scenarios/dsl_delegation.loop
scenarios/multiagent_failure.loop
benchmarks/escenarios_comparables.csv
```

#### Salidas generadas

```text
artifacts/benchmarks/
artifacts/reports/
```

Las salidas generadas se conservan fuera del control de versiones, salvo archivos `.gitkeep` necesarios para mantener directorios vacios.

#### Cierre visual del repositorio

La Fase 10.4 agrega badges, graficos e indice final. No cambia el nucleo de simulacion ni altera los resultados experimentales.

#### Recursos visuales

```text
docs/assets/loopscape-flujo.svg
docs/assets/loopscape-validacion.svg
docs/assets/loopscape-demo-placeholder.svg
```

El archivo `docs/assets/loopscape-demo.gif` debe agregarse solo cuando exista una grabacion real de la interfaz.

### Demo web publicada en Hugging Face Spaces

#### Enlace publico

La demo web de Loopscape esta disponible en:

```text
https://huggingface.co/spaces/kapumota/loopscape
```

El procedimiento de publicacion y validacion esta documentado en `docs/HF_SPACES_DEMO.md`.

### Fase 10.5, guia de uso y demo guiada

#### Documentos agregados

- `docs/GUIA_USO.md`, guia practica para usar Loopscape como demo, simulador y laboratorio.
- `docs/DEMO_GUIADA.md`, recorrido sugerido para presentar la demo web y local.
- `docs/LECTURA_RAPIDA.md`, ruta corta para entender el proyecto en pocos minutos.
