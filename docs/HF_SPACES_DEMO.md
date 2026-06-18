### Demo web en Hugging Face Spaces

#### Estado de publicacion

La demo web de Loopscape fue publicada y probada en Hugging Face Spaces:

```text
https://huggingface.co/spaces/kapumota/loopscape
```

Esta URL se usa como enlace publico estable en el README principal del repositorio.

#### URL directa del runtime

Hugging Face tambien puede exponer una URL directa del runtime:

```text
https://kapumota-loopscape.hf.space
```

Esa URL puede tardar en refrescar o responder distinto segun el estado interno del Space. Para documentacion publica se prefiere la pagina estable del Space.

#### Proposito

Este documento define como preparar, publicar y validar una demo web publica de Loopscape antes del lanzamiento experimental.

La demo se construye desde el artefacto WebAssembly generado por Trunk. No activa despliegues automaticos desde GitHub Pages ni depende de secretos del repositorio principal.

#### Estrategia usada

La demo fue publicada como Space estatico con el contenido generado en `dist/`.

El archivo WebAssembly se almacena mediante Git LFS porque supera el limite de archivos ordinarios permitido por el repositorio del Space.

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

Ese directorio contiene los archivos que se copian al repositorio del Space.

#### Estructura publicada

```text
README.md
.gitattributes
index.html
*.js
*.wasm
```

La regla practica es copiar el contenido de `dist/`, no el directorio completo como subcarpeta.

#### Configuracion del Space

El `README.md` del Space debe iniciar con:

```yaml
---
title: Loopscape
sdk: static
app_file: index.html
pinned: false
---
```

#### Git LFS para WebAssembly

El archivo `.wasm` debe manejarse con Git LFS:

```bash
git lfs install
git lfs track "*.wasm"
git add .gitattributes
```

Para verificar que el commit guarda un puntero LFS y no el binario completo:

```bash
git cat-file -s HEAD:nombre_del_archivo_bg.wasm
git show HEAD:nombre_del_archivo_bg.wasm
```

El primer comando debe mostrar un tamano pequeno. El segundo debe mostrar un puntero `git-lfs`.

#### Validacion posterior

Antes de enlazar la demo desde el README principal se reviso:

```text
la pagina del Space abre sin credenciales
el archivo wasm fue aceptado por Git LFS
la vista inicial se muestra
los controles basicos aparecen en pantalla
la demo se ejecuta desde la pestana App del Space
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
README.md enlaza la demo publicada
README.md no muestra repo not found
README.md no muestra no status en badges manuales
README.md no referencia un GIF inexistente
la documentacion registra el uso de Git LFS para el wasm
la demo abre desde la pagina publica de Hugging Face Spaces
```
