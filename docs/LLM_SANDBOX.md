### Runtime LLM sandbox seguro

#### Objetivo

La Fase 6 introduce una interfaz LLM controlada sin red, sin claves y sin proveedores externos.

El objetivo no es conectar un modelo real todavía. El objetivo es preparar el contrato interno para que futuras fases puedan evaluar decisiones asistidas por modelos sin romper determinismo ni reproducibilidad.

#### Providers permitidos en Fase 6.1

```text
MockProvider
ReplayProvider
```

#### Providers excluidos en Fase 6.1

```text
HTTP real
OpenAI
Ollama
API keys
variables de entorno con secretos
```

#### MockProvider

`MockProvider` genera respuestas deterministas a partir del prompt recibido.

Se usa para probar integración sin red y sin variabilidad externa.

#### ReplayProvider

`ReplayProvider` consume una lista fija de respuestas predefinidas.

Se usa para reproducir decisiones previamente registradas o preparar pruebas de integración controladas.

#### Contrato de seguridad

Todo provider de esta fase debe declarar:

```text
network_enabled=false
secrets_required=false
deterministic=true
```

Si un provider declara red o secretos, debe rechazarse como configuración insegura.

#### Validación

```bash
cargo test llm
cargo test --test llm_mock_provider
make validate-fast
git diff --check
```

#### Alcance

Esta fase no ejecuta un LLM real.

Esta fase no usa internet.

Esta fase no lee claves de API.

Esta fase solo agrega el contrato interno para providers simulados y reproducibles.

### Fase 6.2: limites de tokens y timeouts simulados

#### Objetivo

Modelar costo y limites antes de conectar providers reales.

Esta fase agrega limites locales para prompts, respuestas y latencia simulada. No introduce red, HTTP real, OpenAI, Ollama ni claves.

#### Limites disponibles

```text
max_prompt_tokens
max_response_tokens
timeout_ticks
```

#### Criterio de seguridad

Un provider debe rechazar solicitudes que excedan los limites definidos antes de producir respuesta.

El timeout es simulado en ticks. No mide tiempo real de pared.

#### Validación

```bash
cargo test llm
cargo test --test llm_limits
make validate-fast
git diff --check
```
