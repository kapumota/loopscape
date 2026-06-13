#![allow(dead_code)]
#![allow(clippy::upper_case_acronyms)]

// Capa de red preparatoria para proxy y sandbox LLM.
// La conexion real se endurecera en Fase 5 y Fase 8.
use bevy::prelude::*;

// --------- ABSTRACCIÓN DE HTTP ---------

#[derive(Resource)]
pub struct HTTPClient {
    pub base_url: String,
    pub api_key: Option<String>,
}

impl Default for HTTPClient {
    fn default() -> Self {
        Self {
            base_url: "https://api.openai.com/v1".to_string(),
            api_key: None,
        }
    }
}

#[derive(Event)]
pub struct HTTPRequestEvent {
    pub id: u64,
    pub method: HTTPMethod,
    pub url: String,
    pub body: Option<String>,
    pub headers: Vec<(String, String)>,
}

#[derive(Event)]
pub struct HTTPResponseEvent {
    pub id: u64,
    pub status: u16,
    pub body: String,
    pub success: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HTTPMethod {
    GET,
    POST,
    PUT,
    DELETE,
}

// --------- IMPLEMENTACION NATIVA (Tokio) ---------
#[cfg(not(target_arch = "wasm32"))]
pub mod native {
    use super::*;
    use std::sync::Arc;
    use tokio::runtime::Runtime;

    #[derive(Resource)]
    pub struct NativeRuntime {
        pub rt: Arc<Runtime>,
    }

    impl Default for NativeRuntime {
        fn default() -> Self {
            Self {
                rt: Arc::new(Runtime::new().unwrap()),
            }
        }
    }

    pub fn handle_http_requests_native(
        mut requests: EventReader<HTTPRequestEvent>,
        mut responses: EventWriter<HTTPResponseEvent>,
        _runtime: Res<NativeRuntime>,
    ) {
        for req in requests.read() {
            let id = req.id;
            // En un sistema real, usaríamos bevy_tasks::AsyncComputeTaskPool
            // Aquí simplificamos para el ejemplo

            // Simulación de respuesta para demo
            responses.send(HTTPResponseEvent {
                id,
                status: 200,
                body: r#"{"choices":[{"message":{"content":"Pensamiento: Analizando tarea. Accion: buscar Entrada de accion: consulta Observacion: resultados encontrados"}}]}"#.to_string(),
                success: true,
            });
        }
    }
}

// --------- IMPLEMENTACION WASM (web-sys) ---------
#[cfg(target_arch = "wasm32")]
pub mod wasm {
    use super::*;
    use wasm_bindgen::prelude::*;
    use web_sys::{Request, RequestInit, RequestMode, Response};

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = console)]
        fn log(s: &str);
    }

    pub fn handle_http_requests_wasm(
        mut requests: EventReader<HTTPRequestEvent>,
        mut responses: EventWriter<HTTPResponseEvent>,
    ) {
        for req in requests.read() {
            let id = req.id;
            let _method_str = match req.method {
                HTTPMethod::GET => "GET",
                HTTPMethod::POST => "POST",
                HTTPMethod::PUT => "PUT",
                HTTPMethod::DELETE => "DELETE",
            };

            // En WASM, usamos spawn_local para async
            // Nota: en Bevy WASM, esto requiere manejo cuidadoso del event loop

            // Simulación para demo (en producción usar fetch real)
            responses.send(HTTPResponseEvent {
                id,
                status: 200,
                body: r#"{"choices":[{"message":{"content":"Pensamiento: Procesando en WASM. Accion: finalizar Entrada de accion: listo Observacion: tarea completa"}}]}"#.to_string(),
                success: true,
            });
        }
    }

    /// Función real de fetch para WASM (para uso futuro)
    pub async fn wasm_fetch(
        url: &str,
        method: &str,
        body: Option<&str>,
    ) -> Result<String, JsValue> {
        let opts = RequestInit::new();
        opts.set_method(method);
        opts.set_mode(RequestMode::Cors);

        if let Some(b) = body {
            opts.set_body(&JsValue::from_str(b));
        }

        let request = Request::new_with_str_and_init(url, &opts)?;

        let window = web_sys::window().ok_or("No window")?;
        let resp_value =
            wasm_bindgen_futures::JsFuture::from(window.fetch_with_request(&request)).await?;
        let resp: Response = resp_value.dyn_into()?;

        let text = wasm_bindgen_futures::JsFuture::from(resp.text()?).await?;
        Ok(text.as_string().unwrap_or_default())
    }
}

// --------- SISTEMA UNIFICADO ---------
pub fn handle_http_requests(
    #[cfg(not(target_arch = "wasm32"))] requests: EventReader<HTTPRequestEvent>,
    #[cfg(not(target_arch = "wasm32"))] responses: EventWriter<HTTPResponseEvent>,
    #[cfg(not(target_arch = "wasm32"))] runtime: Res<native::NativeRuntime>,
) {
    #[cfg(not(target_arch = "wasm32"))]
    native::handle_http_requests_native(requests, responses, runtime);
}

#[cfg(target_arch = "wasm32")]
pub fn handle_http_requests_wasm_system(
    requests: EventReader<HTTPRequestEvent>,
    responses: EventWriter<HTTPResponseEvent>,
) {
    wasm::handle_http_requests_wasm(requests, responses);
}
