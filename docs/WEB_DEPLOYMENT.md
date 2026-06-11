### Despliegue web

#### Objetivo

Loopscape puede compilarse a WebAssembly con Trunk y publicarse como sitio estatico. El despliegue web debe mantenerse separado de la validacion principal para que un error de publicacion no bloquee pruebas locales.

#### Compilacion local

```bash
make setup
make web-build
```

El resultado queda en:

```text
dist/
```

#### Prueba local

```bash
make web-serve
```

Abre:

```text
http://localhost:8080
```

#### GitHub Pages

El workflow de despliegue compila `dist/` desde `main`. Para usarlo:

```bash
git checkout main
git pull origin main
git push origin main
```

Luego configura GitHub Pages para usar GitHub Actions.

#### Netlify

Usa estos valores:

```text
Comando de compilacion: cargo install trunk --locked && trunk build --release
Publish directory: dist
```

#### Vercel

Usa `vercel.json` como base. Si el proveedor no detecta Rust, configura el build command manualmente:

```text
cargo install trunk --locked && trunk build --release
```

#### Cloudflare Pages

Usa estos valores:

```text
Comando de compilacion: cargo install trunk --locked && trunk build --release
Directorio de salida de compilacion: dist
```

#### Recomendacion de Fase 1

Primero valida localmente:

```bash
make validate
```

Despues despliega. La validacion debe ser obligatoria; el despliegue debe ser consecuencia de una rama fusionada correctamente.
