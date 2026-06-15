### CI ligero sin GitHub Pages

#### Objetivo

El flujo principal de GitHub Actions debe validar que Loopscape sigue compilando y probandose sin convertir cada push en un despliegue web pesado. La compilacion WebAssembly queda separada como tarea manual porque consume mas tiempo, instala Trunk y genera artefactos estaticos.

#### Problema corregido

El workflow anterior ejecutaba un despliegue web en cada push a `main` y trataba de publicar `dist/` en la rama `gh-pages`. En repositorios donde el token de GitHub Actions no tiene permisos de escritura, esa publicacion falla con error 403.

Loopscape no necesita GitHub Pages como parte de la validacion diaria. Por eso la publicacion automatica fue retirada.

#### Flujo nuevo

El flujo principal es:

```text
.github/workflows/ci.yml
```

Este flujo ejecuta:

```bash
make style-check
make fmt-check
cargo check --locked --all-targets
cargo test --locked --all-targets --no-fail-fast
```

No instala Trunk, no agrega el target WebAssembly y no publica ramas remotas.

#### Build web manual

El build web queda disponible solo bajo ejecucion manual:

```text
.github/workflows/web-build.yml
```

Este flujo ejecuta:

```bash
make web-build-release
```

Luego guarda `dist/` como artefacto temporal de GitHub Actions. No publica en GitHub Pages.

#### Validacion local recomendada

Para trabajo diario:

```bash
make validate
```

Para una revision completa antes de release:

```bash
make validate-full
```

#### Criterio de aceptacion

La fase queda corregida si:

- el workflow `deploy.yml` ya no existe;
- no aparece `peaceiris/actions-gh-pages` en `.github/workflows`;
- el CI principal no compila WebAssembly en cada push;
- el build web solo corre cuando se lanza manualmente;
- la documentacion ya no indica GitHub Pages como ruta recomendada.

#### Compatibilidad con Node.js 24

Los workflows principales declaran `FORCE_JAVASCRIPT_ACTIONS_TO_NODE24` para probar la migracion de GitHub Actions hacia Node.js 24. Tambien se evita depender de acciones externas innecesarias para instalar Rust o restaurar cache.

La validacion diaria sigue siendo nativa y ligera. El build WebAssembly permanece como tarea manual.
