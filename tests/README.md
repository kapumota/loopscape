### Pruebas de Loopscape

#### Criterio actual

Las pruebas rapidas del nucleo viven junto a los modulos de `src/core`. Esta decision permite ejecutar `cargo test --lib core` sin depender del arranque visual de Bevy.

#### Integracion futura

Cuando Loopscape tenga un crate de biblioteca mas amplio, esta carpeta puede alojar pruebas de integracion para DSL, replay y escenarios comparables.
