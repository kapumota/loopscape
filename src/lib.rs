//! Biblioteca minima de Loopscape para pruebas del nucleo.
//!
//! El binario mantiene la capa Bevy. Esta biblioteca expone solo modulos puros
//! para ejecutar pruebas rapidas sin levantar la aplicacion visual ni WebAssembly.

pub mod core;
pub mod dsl;
pub mod llm;
