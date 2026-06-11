#!/usr/bin/env bash
set -euo pipefail

# Reglas basicas de estilo del repositorio.
# Mantiene documentacion y cadenas visibles sin simbolos decorativos.

search_paths=(README.md CHANGELOG.md CONTRIBUTING.md SECURITY.md docs src proxy index.html setup.sh deploy.sh)

symbol_matches="$(perl -CSD -ne 'print "$ARGV:$.:$_" if /[\x{2013}\x{2014}\x{2190}-\x{21FF}\x{2500}-\x{257F}\x{1F000}-\x{1FAFF}]/' "${search_paths[@]}" || true)"
if [[ -n "$symbol_matches" ]]; then
    printf '%s\n' "$symbol_matches"
    echo "Se encontraron simbolos no permitidos"
    exit 1
fi

if grep -RIn '^# ' README.md CHANGELOG.md CONTRIBUTING.md SECURITY.md docs/*.md 2>/dev/null; then
    echo "La documentacion debe usar titulos ### y subtitulos ####"
    exit 1
fi

if grep -RIn '^## ' README.md CHANGELOG.md CONTRIBUTING.md SECURITY.md docs/*.md 2>/dev/null; then
    echo "La documentacion debe usar titulos ### y subtitulos ####"
    exit 1
fi

echo "Estilo del repositorio validado"
