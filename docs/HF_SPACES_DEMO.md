### Demo web en Hugging Face Spaces

#### Proposito

Este documento define como preparar una demo web publica de Loopscape antes del lanzamiento experimental.

La demo debe construirse desde el artefacto WebAssembly generado por Trunk. No debe activar despliegues automaticos desde GitHub Pages ni depender de secretos del repositorio principal.

#### Estrategia recomendada

La primera opcion es usar un Space estatico con el contenido generado en `dist/`.

La alternativa Docker solo debe usarse si el Space estatico no sirve para cargar correctamente los archivos WebAssembly, las rutas o los tipos MIME.

#### Preparacion local

```bash
rustup target add wasm32-unknown-unknown
cargo install trunk --locked
trunk build --release
```

El comando genera el directorio:

```text
dist/
```

Ese directorio contiene los archivos que deben copiarse al Space.

#### Estructura esperada del Space estatico

```text
README.md
index.html
assets/
*.js
*.wasm
```

La estructura exacta puede variar segun lo que genere Trunk. La regla practica es copiar el contenido de `dist/`, no el directorio completo como subcarpeta.

#### Validacion previa

Antes de enlazar la demo desde el README principal, revisa:

```text
la pagina abre sin consola roja critica
el archivo wasm carga correctamente
las rutas relativas funcionan
la vista inicial se muestra
los controles basicos responden
el Space no requiere credenciales para verlo
```

#### Politica de enlace publico

El README principal no debe apuntar a una URL de Hugging Face Spaces hasta que el Space exista y haya sido probado.

Mientras tanto, el badge se mantiene como estado pendiente:

```text
demo HF Spaces pendiente
```

#### GIF de demostracion

El GIF local debe agregarse solo cuando exista el archivo:

```text
docs/assets/loopscape-demo.gif
```

Si todavia no existe, se conserva la vista previa SVG:

```text
docs/assets/loopscape-demo-placeholder.svg
```

#### Criterio de fase lista

La fase queda lista cuando:

```text
los badges del README no muestran repo not found
los workflows manuales no aparecen como no status en badges dinamicos
docs/HF_SPACES_DEMO.md documenta el flujo de demo web
README.md no referencia un GIF inexistente
la vista previa local existe en docs/assets/
```
