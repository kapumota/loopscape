### Fallo bizantino simplificado

#### Objetivo

La Fase 7.3 agrega un modelo determinista de fallo bizantino simplificado. Un worker puede devolver una respuesta falsa y el sistema puede aplicar una votacion de mayoria simple para aceptar o rechazar el resultado.

#### Componentes

```text
ByzantineFailureSpec
ByzantineFailurePlan
WorkerResponse
SimpleMajorityVoter
VotingConfig
VotingOutcome
VoteDecision
```

#### Modelo

```text
workers honestos responden el valor esperado
workers bizantinos responden un valor falso configurado
la votacion cuenta respuestas iguales
si un valor alcanza la mayoria requerida se acepta
si hay empate o respuestas insuficientes se rechaza
```

#### Alcance

Este modelo no implementa consenso bizantino completo. Es una abstraccion controlada para evaluar respuestas falsas y votacion simple dentro del core.

#### Determinismo

El fallo se define por worker y valor falso. No hay red, reloj de pared, firmas criptograficas ni aleatoriedad externa.

#### Validacion

```bash
cargo fmt
cargo test byzantine
cargo test --test fallo_bizantino
make validate-fast
git diff --check
```

### Fallo bizantino por CLI y DSL

#### CLI

```bash
cargo run -- --byzantine-vote verdadero --agents 3 --byzantine-failure 2:falso
```

#### DSL

```text
/byzantine-failure 2 falso
/byzantine-vote verdadero
```

#### Resultado esperado

El worker configurado emite una respuesta falsa. La votacion por mayoria simple acepta el valor honesto si alcanza los votos requeridos.
