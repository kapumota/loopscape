### Build web y despliegue estatico

#### Objetivo

Loopscape puede compilarse a WebAssembly con Trunk y publicarse como sitio estatico. El build web debe mantenerse separado de la validacion principal para que un error de hosting no bloquee pruebas, revisiones ni Pull Requests.

#### Compilacion local

```bash
make setup-web
make web-build-release
```

El resultado queda en:

```text
dist/
```

#### Prueba local

```bash
make setup-web
make web-serve
```

Luego abre:

```text
http://localhost:8080
```

#### Politica del repositorio

Loopscape no publica automaticamente en GitHub Pages. El repositorio mantiene el build web como una accion manual y guarda `dist/` como artefacto temporal cuando se necesita revisar una version web.

La validacion diaria debe ser ligera:

```bash
make validate
```

El build web completo debe ejecutarse solo cuando sea necesario:

```bash
make validate-web
```

#### Netlify

Usa estos valores:

```text
Comando de compilacion: make setup-web && make web-build-release
Publish directory: dist
```

#### Vercel

Usa `vercel.json` como base. Si el proveedor no detecta Rust, configura el build command manualmente:

```text
make setup-web && make web-build-release
```

#### Cloudflare Pages

Usa estos valores:

```text
Comando de compilacion: make setup-web && make web-build-release
Directorio de salida de compilacion: dist
```

#### Validacion antes de desplegar

Antes de publicar ejecuta:

```bash
make validate
```

Despues genera el build web de forma explicita:

```bash
make validate-web
```

El despliegue debe ser una decision manual de release, no un efecto lateral de cada push a `main`.
