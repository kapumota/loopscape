### Contribucion

#### Estilo de ramas

Usa ramas por fase:

```text
fase-1-base-profesional
fase-2-dsl-visual
fase-3-runtime-loop
```

#### Estilo de commits

Usa mensajes en espanol:

```text
fase 1: agrega validacion reproducible
corrige parser ReAct para respuestas en espanol
actualiza documentacion de arquitectura
```

#### Estilo de documentacion

- Titulo principal con `###`.
- Subtitulos con `####`.
- Sin guiones largos.
- Sin simbolos decorativos innecesarios.
- Texto tecnico claro y reproducible.

#### Estilo de codigo

- Firmas de funciones en ingles.
- Comentarios en espanol.
- Cadenas visibles en espanol.
- Pruebas para cambios de logica.

#### Validacion antes de Pull Request

```bash
make validate
```
