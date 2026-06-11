### Arquitectura de Loopscape

#### Idea central

Loopscape modela cada loop como una entidad ECS. Cada comportamiento se expresa mediante componentes y sistemas. Esto evita una jerarquia rigida de clases y permite que las eras se activen como cambios de reglas sobre el mismo mundo simulado.

#### Modelo ECS

```text
Entidad: LoopAgent
Componentes:
  LoopState
  LoopVisual
  ThinkTimer
  ActTimer
  ObserveTimer
  TaskDecomposer
  RalphDna
  FormalCommand
  Supervisor
  Worker
  ConsensusVoter
```

#### Recursos globales

```text
GlobalPrompt: prompt compartido entre loops.
TaskQueue: cola de tareas entrantes.
Metrics: metricas visibles de simulacion.
EraConfig: nombres y orden de eras.
XRayMode: inspeccion visual del estado interno.
```

#### Sistemas principales

```text
react_cycle_system: avanza Think, Act y Observe.
autonomous_decomposition: parte una tarea grande en subtareas.
shared_dna_propagation: sincroniza comportamiento en la Era Ralph.
command_execution_system: ejecuta comandos formales.
latido_system: emite latidos entre supervisores y trabajadores.
consensus_voting: simula votacion entre agentes.
byzantine_detection: marca loops sospechosos.
```

#### Separacion por eras

Cada era debe mantener una responsabilidad clara:

- ReAct: ciclo secuencial con herramientas conectadas;
- Autoprompting: descomposicion y subloops temporales;
- Ralph Loop: prompt central compartido;
- Ralph formalizado: comandos formales y autoterminacion;
- Orquestacion multiagente: supervision, fallos y consenso.

#### Reglas de codigo

- Las firmas de funciones se escriben en ingles.
- Los comentarios se escriben en espanol.
- Las cadenas visibles para el usuario se escriben en espanol.
- Los separadores usan lineas simples con guion normal.
- La documentacion usa titulos `###` y subtitulos `####`.
