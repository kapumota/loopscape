
### Auditoría ligera de workflows

#### Objetivo

La Fase 8.1 endurece GitHub Actions sin convertir el CI en una tubería pesada.

El objetivo es mantener validación reproducible para pull requests y `main`, pero sin despliegues automáticos, sin secretos en PR y con permisos mínimos.

#### Política aplicada

Los workflows deben cumplir estas reglas:

```text
permissions mínimos
contents: read
sin pull_request_target
sin lectura de secrets
sin deploy automático
sin permisos de escritura
acciones JavaScript forzadas a Node 24
```

#### Workflows incluidos

```text
.github/workflows/ci.yml
.github/workflows/web-build.yml
```

`ci.yml` se ejecuta en `pull_request` y `push` hacia `main`. Valida estilo, formato, compilación nativa y pruebas nativas.

`web-build.yml` queda disponible solo con `workflow_dispatch`. Construye WebAssembly y guarda un artefacto temporal, pero no despliega automáticamente.

#### Variable de entorno

Los workflows definen:

```text
FORCE_JAVASCRIPT_ACTIONS_TO_NODE24=true
```

Esto deja explícita la intención de ejecutar acciones JavaScript con Node 24 cuando el runner lo soporte.

#### Reglas de seguridad

El job de auditoría rechaza workflows que usen:

```text
pull_request_target
secrets.
contents: write
pages: write
id-token: write
```

Estas reglas evitan que un pull request tenga acceso accidental a secretos o permisos de publicación.

#### Alcance

Esta fase no agrega despliegue, publicación de releases ni GitHub Pages.

El objetivo es preparar una base segura para las siguientes fases, donde podrán agregarse reportes o artefactos sin abrir permisos innecesarios.


### Relacion con auditoria Rust

#### Fase 8.2

La auditoria de dependencias Rust no se ejecuta en cada PR.

El proyecto usa un workflow separado para auditoria manual y semanal:

```text
.github/workflows/rust-security.yml
```

Este flujo mantiene la seguridad de dependencias sin hacer pesado el CI automatico.

### Fase 8.3: escaneo manual de secretos

#### Politica

El escaneo de secretos queda separado del CI automatico. Se ejecuta mediante `workflow_dispatch` y schedule semanal.

No debe ejecutarse en cada PR ni en cada push. Tampoco debe usar secretos, permisos de escritura ni despliegue automatico.

#### Workflow

```text
.github/workflows/secrets-scan.yml
```

El workflow usa `gitleaks` para detectar valores sensibles versionados por error.
