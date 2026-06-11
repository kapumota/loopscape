### Integracion LLM

#### Objetivo

Loopscape puede funcionar con respuestas simuladas o con un proveedor LLM externo. Para una primera fase profesional, la integracion real debe tratarse como una capacidad opcional y no como requisito para ejecutar el juego.

#### Modos de ejecucion

```text
Mock: respuestas simuladas sin llamadas externas.
Nativo: llamadas HTTP desde la aplicacion de escritorio.
Web: llamadas mediante proxy para no exponer secretos en el navegador.
```

#### Configuracion local

Copia la plantilla:

```bash
cp .env.example .env
```

Edita las variables necesarias:

```text
OPENAI_API_KEY=tu_clave_local
OPENAI_BASE_URL=https://api.openai.com/v1
OPENAI_MODEL=gpt-4o-mini
LLM_MOCK_MODE=true
```

#### Proxy local

Para ejecutar el proxy local:

```bash
make proxy-install
make proxy-run
```

En otra terminal:

```bash
make web-serve
```

#### Seguridad

La version web no debe incrustar claves de API en `index.html`, JavaScript ni archivos versionados. Las claves deben vivir en variables de entorno del servidor o del proveedor de despliegue.

#### Formato ReAct soportado

El parser acepta el formato clasico:

```text
Thought: analizar la tarea
Accion: buscar
Entrada de accion: consulta
Observation: resultado observado
```

Tambien puede aceptar una variante en espanol cuando se habilite en las pruebas:

```text
Pensamiento: analizar la tarea
Accion: buscar
Entrada de accion: consulta
Observacion: resultado observado
```
