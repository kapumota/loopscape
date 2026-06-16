### Visor DSL

#### Objetivo

El visor DSL muestra en pantalla el programa `.loop` cargado desde la CLI. Esta fase no agrega editor visual de nodos. Solo presenta el flujo textual del DSL y su avance en un panel lateral.

#### Uso

Para abrir el visor con un script DSL se usa la opcion `--visual` junto con `--script`.

```bash
cargo run -- --script examples/rescate.loop --visual --seed 123 --ticks 50
```

El comando sin `--visual` conserva el comportamiento de validacion corta por terminal.

```bash
cargo run -- --script examples/rescate.loop --seed 123 --ticks 50
```

#### Estados visibles

El panel lateral marca cada comando con uno de estos estados:

```text
pendiente
activo
completado
error
```

El comando activo se resalta con el prefijo `>>`. El avance se calcula desde el tiempo visual de la aplicacion y no modifica todavia la logica determinista del scheduler.

#### Alcance

Esta fase permite demostrar que un script `.loop` puede llegar a la capa visual sin implementar todavia seleccion de nodos, arrastre, conexiones editables ni guardado de grafos.

#### Validacion

```bash
make validate-fast
cargo run -- --script examples/rescate.loop --seed 123 --ticks 50
```

#### Entornos remotos sin pantalla

El modo visual requiere una sesion grafica local. En servidores remotos por SSH normalmente no existen `DISPLAY`, `WAYLAND_DISPLAY` ni `WAYLAND_SOCKET`.

Si se ejecuta `--visual` sin entorno grafico, Loopscape muestra un mensaje en español y termina sin panic. Para validar en remoto se debe usar el modo sin ventana.

```bash
cargo run -- --script examples/rescate.loop --seed 123 --ticks 50
```

Para abrir el visor se necesita una sesion grafica real.

```bash
cargo run -- --script examples/rescate.loop --visual --seed 123 --ticks 50
```
