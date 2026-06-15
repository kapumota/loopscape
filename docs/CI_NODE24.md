### CI compatible con Node.js 24

#### Objetivo

Esta fase prepara los workflows de Loopscape para la migracion de GitHub Actions desde Node.js 20 hacia Node.js 24. El objetivo es reducir advertencias futuras sin volver pesado el flujo de validacion diaria.

#### Alcance

Los cambios se concentran en `.github/workflows` y en la documentacion de CI. No se modifica el nucleo de simulacion, la visualizacion, el runtime del juego ni la estructura del proyecto Rust.

#### Cambios aplicados

- Se agrega `FORCE_JAVASCRIPT_ACTIONS_TO_NODE24` en los workflows.
- Se actualiza `actions/checkout` a una version compatible con Node.js 24.
- Se actualiza `actions/cache` cuando se usa cache explicito.
- Se actualiza `actions/upload-artifact` en el build web manual.
- Se reemplaza la accion externa de cache de Rust por cache oficial de GitHub Actions.
- Se reemplaza la accion externa de instalacion de Rust por comandos `rustup`.
- Se mantienen permisos minimos con `contents: read`.

#### Validacion local

```bash
grep -R "FORCE_JAVASCRIPT_ACTIONS_TO_NODE24" -n .github/workflows
grep -R "actions/cache@v4" -n .github/workflows || true
grep -R "actions/checkout@v4" -n .github/workflows || true
grep -R "actions/upload-artifact@v4" -n .github/workflows || true
make validate
```

#### Criterio de aceptacion

La fase queda cerrada si los workflows ya no usan acciones oficiales antiguas basadas en Node.js 20, si el entorno fuerza Node.js 24 durante la transicion y si `make validate` sigue pasando localmente.
