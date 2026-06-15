### Smoke nativo

#### Objetivo

El modo smoke nativo permite verificar que el binario de Loopscape arranca y ejecuta una corrida corta del nucleo determinista sin abrir un flujo largo de validacion.

#### Comando principal

```bash
cargo run -- --smoke --seed 123 --ticks 10
```

#### Comando con Makefile

```bash
make smoke-native
```

#### Argumentos

```text
--smoke        activa la prueba de humo nativa
--headless     alias para ejecucion nativa sin ventana
--seed N       fija la semilla determinista
--ticks N      define cuantos ticks ejecutar
--agents N     define cuantos agentes iniciales crear
--tasks N      define cuantas tareas iniciales crear
```

#### Criterio de cierre

La fase queda cerrada cuando el comando de smoke termina con codigo 0 y muestra un resumen de ticks, agentes, tareas, metricas y eventos generados.

#### Relacion con la validacion por niveles

El modo smoke no reemplaza `make test-core` ni `make test-deterministic`. Su funcion es comprobar que el binario nativo puede arrancar y ejecutar el nucleo por pocos ticks antes de avanzar a pruebas o builds mas costosos.
