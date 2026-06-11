### Fase 1: base profesional reproducible

#### Objetivo

La Fase 1 convierte Loopscape en una base mantenible para evolucionar por ramas, Pull Requests y parches. No busca agregar nuevas eras. Busca que el proyecto sea verificable, limpiable y explicable.

#### Alcance

Esta fase cubre:

- comandos reproducibles con `make`;
- validacion local con formato, compilacion y pruebas;
- limpieza de artefactos generados;
- documentacion tecnica con estilo uniforme;
- CI separado del despliegue;
- flujo de trabajo por ramas y patches;
- pruebas unitarias iniciales para el parser ReAct.

#### Fuera de alcance

Esta fase no cubre todavia:

- multiplayer;
- editor visual de niveles;
- integracion real con streaming LLM;
- persistencia de partidas;
- consenso formal completo;
- motor Zig separado.

#### Rama sugerida

```bash
git checkout main
git pull origin main
git checkout -b fase-1-base-profesional
```

#### Validacion local

```bash
make setup
make fmt-check
make check
make test
make web-build
```

#### Commit sugerido

```bash
git add .
git commit -m "fase 1: endurece base reproducible de Loopscape"
```

#### Pull Request sugerido

Titulo:

```text
fase 1: base profesional reproducible
```

Descripcion:

```text
#### Resumen

Esta rama profesionaliza la base de Loopscape con comandos reproducibles, validacion local, CI, limpieza de artefactos y documentacion tecnica.

#### Cambios principales

- Agrega Makefile con comandos de setup, validacion, pruebas, web build y limpieza.
- Agrega scripts de validacion y limpieza.
- Agrega CI para formato, check, pruebas y build WASM.
- Reorganiza documentacion con titulos ### y subtitulos ####.
- Agrega pruebas unitarias iniciales para parsing ReAct.

#### Validacion

- make fmt-check
- make check
- make test
- make web-build
```

#### Criterio de aceptacion

La fase se acepta si:

- el proyecto compila en modo nativo;
- el build WASM genera `dist/`;
- las pruebas unitarias pasan;
- la documentacion no depende de pasos manuales ambiguos;
- `.env` y secretos no se versionan;
- `make clean` deja limpio el arbol de trabajo de artefactos generados.
