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

#### Pruebas rapidas del lexer DSL

Para cambios del lexer se puede ejecutar una prueba focalizada:

```bash
cargo test dsl::lexer
```

Esta validacion no levanta Bevy ni compila WebAssembly.

#### Pruebas rapidas del parser DSL

Para cambios del parser se puede ejecutar una prueba focalizada:

```bash
cargo test dsl::parser
```

Para validar todo el DSL sin levantar Bevy ni compilar WebAssembly:

```bash
cargo test dsl
```

#### Validacion semantica del DSL

La fase 3.4 agrega pruebas rapidas del validador semantico del DSL. Para cambios en reglas de orquestacion se debe ejecutar:

```bash
cargo test dsl::validator
make validate-fast
```

Esta validacion no levanta Bevy ni compila WebAssembly.

#### Interpretacion del DSL hacia eventos

La fase 3.5 agrega pruebas rapidas para convertir un programa DSL validado en eventos internos del nucleo.

Para cambios en el interprete se debe ejecutar:

```bash
cargo test dsl::interpreter
cargo test core
make validate-fast
```

Esta validacion no levanta Bevy ni compila WebAssembly.

#### Ejecucion de scripts DSL

La fase 3.6 agrega una comprobacion manual para scripts `.loop` desde el binario nativo.

```bash
cargo run -- --script examples/rescate.loop --seed 123 --ticks 50
make validate-fast
```

Esta validacion no compila WebAssembly y no publica artefactos. Sirve para verificar el flujo del DSL antes de conectar editor visual o replay.
