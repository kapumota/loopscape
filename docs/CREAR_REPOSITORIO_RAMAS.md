### Crear repositorio y trabajar por ramas

#### Objetivo

Este flujo permite convertir Loopscape en un proyecto reproducible, auditable y facil de mejorar por fases. La regla principal es que `main` siempre debe representar una version estable o validada.

#### Crear repositorio local desde el ZIP

```bash
mkdir loopscape
cd loopscape
unzip ../loopscape.zip -d .
```

Si el ZIP ya contiene la carpeta `loopscape`, entra a ella:

```bash
cd loopscape
```

Inicializa Git:

```bash
git init
git branch -M main
git status --short
```

Crea el primer commit:

```bash
git add .
git commit -m "fase 0: importa base inicial de Loopscape"
```

#### Crear repositorio remoto en GitHub

Crea un repositorio vacio en GitHub, por ejemplo:

```text
loopscape
```

Luego conecta el remoto:

```bash
git remote add origin https://github.com/kapumota/loopscape.git
git push -u origin main
```

#### Rama para Fase 1

```bash
git checkout main
git pull origin main
git checkout -b fase-1-base-profesional
```

Aplica los cambios de la fase, valida y confirma:

```bash
make validate
git status --short
git add .
git commit -m "fase 1: endurece base reproducible de Loopscape"
git push -u origin fase-1-base-profesional
```

#### Pull Request recomendado

Abre un Pull Request desde:

```text
fase-1-base-profesional -> main
```

Usa merge normal, no squash, si quieres conservar la trazabilidad de cada fase.

#### Limpiar ramas despues del merge

Despues de fusionar el Pull Request:

```bash
git checkout main
git pull origin main
git branch -d fase-1-base-profesional
git push origin --delete fase-1-base-profesional
```

#### Generar patch de una fase

```bash
git diff main..fase-1-base-profesional > patches/fase-1-base-profesional.patch
```

Si quieres un patch desde cambios no confirmados:

```bash
git diff > patches/cambios-locales.patch
```

#### Aplicar patch en una copia limpia

```bash
git checkout main
git pull origin main
git checkout -b prueba-patch-fase-1
git apply --check patches/fase-1-base-profesional.patch
git apply patches/fase-1-base-profesional.patch
make validate
```

#### Convencion de ramas

```text
fase-0-importacion-inicial
fase-1-base-profesional
fase-2-core-simulacion
fase-3-dsl-orquestacion
fase-4-editor-visual
fase-5-runtime-llm-sandbox
fase-6-multiagente-fallos
fase-7-replay-benchmarks
fase-8-seguridad-ci
fase-9-release-web-desktop
fase-10-investigacion-avanzada
```

#### Convencion de commits

```text
fase 1: endurece base reproducible de Loopscape
fase 2: separa nucleo determinista de simulacion
fase 3: agrega DSL de comandos de orquestacion
fase 4: agrega editor visual de nodos y conexiones
fase 5: aisla integracion LLM mediante proxy y modo simulado
```
