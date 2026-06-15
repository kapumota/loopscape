### Validacion por niveles

#### Objetivo

Esta fase separa la validacion de Loopscape en niveles para reducir el costo de los Pull Requests normales y reservar las comprobaciones pesadas para ejecuciones importantes o manuales.

#### Niveles

```text
validate-fast: estilo y formato
validate: validacion rapida, compilacion nativa y pruebas nativas
validate-web: build WebAssembly manual
validate-full: validacion media, build web y Clippy estricto
```

#### Uso recomendado

Para cambios pequeños de documentacion, workflows o ajustes no criticos:

```bash
make validate-fast
```

Para cambios de codigo Rust o cambios que afectan el nucleo:

```bash
make validate
```

Para revisar el build de navegador:

```bash
make validate-web
```

Para cierres de fase, release candidates o cambios de alto impacto:

```bash
make validate-full
```

#### Criterio de aceptacion

La fase queda cerrada si existen los targets `validate-fast`, `validate`, `validate-web` y `validate-full`, si los scripts correspondientes estan separados y si el workflow principal puede usar validacion rapida para Pull Requests normales.

#### Pruebas rapidas del nucleo

Para cambios que afectan `src/core`, se agregan dos comandos focalizados:

```bash
make test-core
make test-deterministic
```

Estos comandos permiten revisar el nucleo determinista sin ejecutar el build WebAssembly ni abrir la aplicacion visual.

#### Smoke nativo

Para revisar que el binario nativo arranca sin abrir un flujo largo se agrega el comando:

```bash
make smoke-native
```

El comando ejecuta una corrida corta del nucleo determinista:

```bash
cargo run -- --smoke --seed 123 --ticks 10
```

Este paso no reemplaza las pruebas del nucleo. Sirve como comprobacion rapida de arranque del binario nativo.

#### Pruebas rapidas del DSL

Para cambios que afectan `src/dsl`, se puede ejecutar una validacion focalizada sin levantar Bevy ni compilar WebAssembly:

```bash
cargo test dsl
```

Este comando valida el AST y el modelo de comandos del lenguaje de orquestacion.
