### Pruebas rapidas del nucleo

#### Objetivo

Esta fase agrega pruebas focalizadas para el nucleo determinista de Loopscape. El objetivo es validar los modulos puros del core sin levantar la aplicacion Bevy ni ejecutar el build WebAssembly.

#### Modulos cubiertos

```text
core::agent
core::task
core::event
core::metrics
core::rng
core::scheduler
```

#### Comandos nuevos

```bash
make test-core
make test-deterministic
```

`make test-core` ejecuta pruebas del nucleo como biblioteca Rust. `make test-deterministic` ejecuta pruebas filtradas por comportamiento determinista.

#### Relacion con la validacion por niveles

Para Pull Requests diarios se mantiene:

```bash
make validate-fast
```

Para cambios que tocan el nucleo se recomienda ejecutar:

```bash
make test-core
make test-deterministic
make validate-fast
```

Para cierres de fase o cambios de alto impacto se debe ejecutar tambien:

```bash
make validate
```

#### Criterio de aceptacion

La fase queda cerrada si existen los targets `test-core` y `test-deterministic`, si las pruebas del nucleo se ejecutan desde el target de biblioteca y si la validacion rapida sigue sin activar Bevy visual ni WebAssembly.
