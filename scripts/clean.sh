#!/usr/bin/env bash
set -euo pipefail

# Limpieza de artefactos generados por Rust, Trunk, Node y editores.
echo "Limpiando artefactos generados"
rm -rf target dist .trunk artifacts/validation
find . -type f -name '*.log' -delete
find . -type f -name '*.tmp' -delete
find . -type f -name '*.bak' -delete
find . -type f -name '*.swp' -delete
find . -type f -name '*.swo' -delete
find . -type f -name '.DS_Store' -delete
find . -type f -name 'Thumbs.db' -delete

echo "Limpieza completa"
