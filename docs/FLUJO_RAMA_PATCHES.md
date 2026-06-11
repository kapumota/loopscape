### Flujo de trabajo con ramas y patches

#### Crear rama de fase

```bash
git checkout main
git pull origin main
git checkout -b fase-1-base-profesional
```

#### Validar antes de hacer commit

```bash
make validate
```

#### Crear commit

```bash
git add .
git commit -m "fase 1: endurece base reproducible de Loopscape"
```

#### Generar patch

```bash
mkdir -p patches
git diff main...HEAD > patches/fase-1-base-profesional.patch
```

#### Aplicar patch en una copia limpia

```bash
git checkout main
git pull origin main
git checkout -b fase-1-base-profesional
git apply patches/fase-1-base-profesional.patch
make validate
```

#### Subir rama

```bash
git push -u origin fase-1-base-profesional
```

#### Fusionar

Se recomienda Pull Request normal hacia `main`. No uses squash si quieres conservar trazabilidad por fase.

#### Limpiar rama despues del merge

```bash
git checkout main
git pull origin main
git branch -d fase-1-base-profesional
git push origin --delete fase-1-base-profesional
```
