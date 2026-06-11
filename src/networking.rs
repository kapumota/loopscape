use bevy::prelude::*;
use serde::{Deserialize, Serialize};

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
    use reqwest;
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
        runtime: Res<NativeRuntime>,
    ) {
        for req in requests.read() {
            let id = req.id;
            let url = req.url.clone();
            let method = req.method.clone();
            let body = req.body.clone();
            let headers = req.headers.clone();

            let rt = runtime.rt.clone();
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
    use wasm_bindgen_futures::spawn_local;
    use web_sys::{Request, RequestInit, RequestMode, Response, Headers};

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
            let url = req.url.clone();
            let method_str = match req.method {
                HTTPMethod::GET => "GET",
                HTTPMethod::POST => "POST",
                HTTPMethod::PUT => "PUT",
                HTTPMethod::DELETE => "DELETE",
            };
            let body = req.body.clone();

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
    pub async fn wasm_fetch(url: &str, method: &str, body: Option<&str>) -> Result<String, JsValue> {
        let mut opts = RequestInit::new();
        opts.method(method);
        opts.mode(RequestMode::Cors);

        if let Some(b) = body {
            opts.body(Some(&JsValue::from_str(b)));
        }

        let request = Request::new_with_str_and_init(url, &opts)?;

        let window = web_sys::window().ok_or("No window")?;
        let resp_value = wasm_bindgen_futures::JsFuture::from(window.fetch_with_request(&request)).await?;
        let resp: Response = resp_value.dyn_into()?;

        let text = wasm_bindgen_futures::JsFuture::from(resp.text()?).await?;
        Ok(text.as_string().unwrap_or_default())
    }
}

// --------- SISTEMA UNIFICADO ---------
pub fn handle_http_requests(
    #[cfg(not(target_arch = "wasm32"))]
    mut requests: EventReader<HTTPRequestEvent>,
    #[cfg(not(target_arch = "wasm32"))]
    mut responses: EventWriter<HTTPResponseEvent>,
    #[cfg(not(target_arch = "wasm32"))]
    runtime: Res<native::NativeRuntime>,
) {
    #[cfg(not(target_arch = "wasm32"))]
    native::handle_http_requests_native(requests, responses, runtime);
}

#[cfg(target_arch = "wasm32")]
pub fn handle_http_requests_wasm_system(
    mut requests: EventReader<HTTPRequestEvent>,
    mut responses: EventWriter<HTTPResponseEvent>,
) {
    wasm::handle_http_requests_wasm(requests, responses);
}
