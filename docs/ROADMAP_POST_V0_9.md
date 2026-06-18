### Roadmap posterior a v0.9.0

#### Proposito

Este documento registra las lineas de trabajo posteriores al cierre experimental `v0.9.0`.

No forma parte del alcance de la version de cierre. Sirve para ordenar trabajo futuro sin modificar el corte experimental.

#### Linea 1: producto experimental mas estable

Objetivo:

- Definir contratos estables de CLI.
- Versionar formatos de escenarios.
- Versionar formatos de eventos JSONL.
- Versionar formatos de metricas CSV.
- Separar mejor nucleo, visualizacion y exportacion.

#### Linea 2: experiencia de usuario

Objetivo:

- Agregar ayuda visible en pantalla.
- Agregar modo tutorial dentro de la demo.
- Agregar descripciones por era.
- Agregar indicadores para escenarios y fallos.
- Agregar una grabacion real de demo si se decide mantener un GIF local.

#### Linea 3: investigacion avanzada

Objetivo:

- Ampliar escenarios comparables.
- Agregar mas fallos multiagente.
- Agregar metricas de estabilidad.
- Agregar comparacion estadistica entre corridas.
- Agregar analisis de sensibilidad por semilla.

#### Linea 4: calidad y seguridad

Objetivo:

- Medir coverage del nucleo.
- Mejorar auditoria de dependencias.
- Evaluar `cargo deny` como politica documentada.
- Revisar permisos de workflows.
- Mantener workflows pesados como manuales.

#### Linea 5: publicacion

Objetivo:

- Crear GitHub Release manual.
- Adjuntar notas de release.
- Adjuntar artefacto web si corresponde.
- Mantener Hugging Face Spaces como demo publica.
- Documentar cambios si el Space requiere Git LFS.

#### Criterio para v1.0.0

No se recomienda `v1.0.0` hasta cumplir como minimo:

- CLI estable.
- Formatos versionados.
- Guia de compatibilidad.
- Benchmarks repetibles con resultados comparados.
- Pruebas prolongadas en entorno externo.
- Documentacion de instalacion para usuarios no desarrolladores.
- Politica de soporte y limitaciones.
