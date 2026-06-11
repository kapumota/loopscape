### Seguridad

#### Secretos

No se deben versionar claves de API, tokens ni archivos `.env`.

Usa `.env.example` como plantilla y configura tus secretos localmente.

#### Integracion LLM

La version web no debe llamar directamente a proveedores externos desde el navegador si eso expone claves. Usa un proxy controlado para las peticiones LLM.

#### Reporte de problemas

Para reportar un problema de seguridad, abre un issue privado o comunica el hallazgo al mantenedor antes de publicar detalles tecnicos.

#### Validacion minima

Antes de fusionar cambios ejecuta:

```bash
make validate
```
