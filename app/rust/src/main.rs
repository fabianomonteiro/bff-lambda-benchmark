use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
    time::Instant,
    net::SocketAddr,
};

use axum::{
    body::{boxed, BoxBody, Full},
    http::{Request, Response, StatusCode, HeaderValue},
    response::IntoResponse,
    routing::{post},
    Router,
    extract::Json,
};
use tower::{Service, Layer};
use serde::Deserialize;
use once_cell::sync::Lazy;

// Para rodar local
#[cfg(not(feature = "lambda"))]
use axum::Server;

// Para rodar na AWS Lambda (apenas se ativar --features lambda)
#[cfg(feature = "lambda")]
use {
    lambda_http::{run as lambda_run, Error as LambdaError},
    lambda_runtime::Context,
};

#[cfg(feature = "lambda")]
use lambda_runtime::{self};

// Para usar write_image no encoder
use image::ImageEncoder;

// ======================
// MIDDLEWARE: TimingLayer
// ======================
#[derive(Clone)]
struct TimingLayer;

#[derive(Clone)]
struct TimingService<S> {
    inner: S,
}

// Implementa a criação do service via Layer
impl<S> Layer<S> for TimingLayer {
    type Service = TimingService<S>;
    fn layer(&self, inner: S) -> Self::Service {
        TimingService { inner }
    }
}

/// Precisamos especificar os tipos corretos para `Service<Request<ReqBody>>`.
/// Aqui, definimos que o `Response` esperado é `Response<BoxBody>`, que é
/// o tipo de resposta padrão do Axum quando montamos handlers.
impl<S, ReqBody> Service<Request<ReqBody>> for TimingService<S>
where
    // O service interno deve aceitar Request<ReqBody> e retornar Response<BoxBody>
    S: Service<Request<ReqBody>, Response = Response<BoxBody>> + Clone + Send + 'static,
    // O futuro do service também precisa ser Send + 'static
    S::Future: Send + 'static,
    // O body da request deve ser Send + 'static
    ReqBody: Send + 'static,
{
    type Response = Response<BoxBody>;
    type Error = S::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, S::Error>> + Send>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request<ReqBody>) -> Self::Future {
        let mut service = self.inner.clone();

        Box::pin(async move {
            let lambda_start = Instant::now();
            let endpoint_start = Instant::now();

            // processa request
            let mut response = service.call(req).await?;

            let lambda_end = Instant::now();
            let endpoint_end = Instant::now();

            let lambda_duration = lambda_end - lambda_start;
            let endpoint_duration = endpoint_end - endpoint_start;

            let headers = response.headers_mut();
            // Exemplo: podemos usar debug ou epoch
            headers.insert(
                "X-Lambda-Start-Time",
                HeaderValue::from_str(&format!("{:?}", lambda_start)).unwrap(),
            );
            headers.insert(
                "X-Lambda-End-Time",
                HeaderValue::from_str(&format!("{:?}", lambda_end)).unwrap(),
            );
            headers.insert(
                "X-Lambda-Duration",
                HeaderValue::from_str(&format!("{:?}", lambda_duration)).unwrap(),
            );
            headers.insert(
                "X-Endpoint-Start-Time",
                HeaderValue::from_str(&format!("{:?}", endpoint_start)).unwrap(),
            );
            headers.insert(
                "X-Endpoint-End-Time",
                HeaderValue::from_str(&format!("{:?}", endpoint_end)).unwrap(),
            );
            headers.insert(
                "X-Endpoint-Duration",
                HeaderValue::from_str(&format!("{:?}", endpoint_duration)).unwrap(),
            );

            Ok(response)
        })
    }
}

// ======================
// MODELOS de input
// ======================
#[derive(Deserialize)]
struct MathPayload {
    numbers: Vec<i64>,
    operation: Option<String>,
}

#[derive(Deserialize)]
struct JsonPayload {
    key: Option<String>,
    value: Option<String>,
}

#[derive(Deserialize)]
struct StringPayload {
    text: Option<String>,
    pattern: Option<String>,
}

#[derive(Deserialize)]
struct CompressPayload {
    text: Option<String>,
}

#[derive(Deserialize)]
struct ImagePayload {
    text: Option<String>,
}

// Se não estiver usando, pode comentar ou remover. Ou então silenciar com:
#[allow(dead_code)]
static REGEX_INSTANCE: Lazy<regex::Regex> = Lazy::new(|| {
    regex::Regex::new(r"hello").unwrap()
});

// Embutindo uma fonte TTF (opcional)
static FONT: Lazy<Option<rusttype::Font<'static>>> = Lazy::new(|| {
    let font_data = include_bytes!("DejaVuSans.ttf");
    rusttype::Font::try_from_bytes(font_data as &[u8])
});

// ======================
// HANDLERS
// ======================

// ------------
// math_operations
// ------------
async fn math_operations(Json(payload): Json<MathPayload>) -> Response<BoxBody> {
    if payload.numbers.is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({ "error": "No numbers provided" }))
        )
        .into_response();
    }

    let operation = payload.operation.unwrap_or_else(|| "sum".to_string());
    let result = match operation.as_str() {
        "sum" => payload.numbers.iter().sum::<i64>(),
        "product" => payload.numbers.iter().product::<i64>(),
        _ => {
            return (
                StatusCode::BAD_REQUEST,
                Json(serde_json::json!({ "error": "Unsupported operation" }))
            )
            .into_response()
        }
    };

    (StatusCode::OK, Json(serde_json::json!({ "result": result }))).into_response()
}

// ------------
// json_manipulation
// ------------
async fn json_manipulation(Json(payload): Json<JsonPayload>) -> Response<BoxBody> {
    let Some(key) = &payload.key else {
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({ "error": "Key and value are required" }))
        )
        .into_response();
    };
    let Some(value) = &payload.value else {
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({ "error": "Key and value are required" }))
        )
        .into_response();
    };

    let json_data = serde_json::json!({ key: value }).to_string();
    (StatusCode::OK, Json(serde_json::json!({ "json_data": json_data }))).into_response()
}

// ------------
// string_processing
// ------------
async fn string_processing(Json(payload): Json<StringPayload>) -> Response<BoxBody> {
    let Some(text) = &payload.text else {
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({ "error": "Text and pattern are required" }))
        )
        .into_response();
    };
    let Some(pattern) = &payload.pattern else {
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({ "error": "Text and pattern are required" }))
        )
        .into_response();
    };

    let re = match regex::Regex::new(pattern) {
        Ok(r) => r,
        Err(_) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(serde_json::json!({ "error": "Invalid regex pattern" }))
            )
            .into_response()
        }
    };
    let matches: Vec<String> = re.find_iter(text).map(|m| m.as_str().to_string()).collect();

    (StatusCode::OK, Json(serde_json::json!({ "matches": matches }))).into_response()
}

// ------------
// compress_data
// ------------
async fn compress_data(Json(payload): Json<CompressPayload>) -> Response<BoxBody> {
    let Some(text) = &payload.text else {
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({ "error": "Text is required" }))
        )
        .into_response();
    };

    use flate2::{Compression, write::GzEncoder};
    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
    std::io::Write::write_all(&mut encoder, text.as_bytes()).unwrap();
    let compressed = encoder.finish().unwrap();

    let body = boxed(Full::from(compressed));
    let mut resp = Response::new(body);
    *resp.status_mut() = StatusCode::OK;
    resp.headers_mut().insert(
        axum::http::header::CONTENT_TYPE,
        HeaderValue::from_static("application/gzip")
    );
    resp
}

// ------------
// image_processing
// ------------
async fn image_processing(Json(payload): Json<ImagePayload>) -> Response<BoxBody> {
    let text = payload.text.clone().unwrap_or_else(|| "Hello, World!".to_string());

    if FONT.is_none() {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({
                "error": "Fonte não carregada. Coloque DejaVuSans.ttf ou comente."
            }))
        ).into_response();
    }

    let width = 200;
    let height = 100;
    let mut img = image::RgbaImage::from_pixel(
        width,
        height,
        image::Rgba([73, 109, 137, 255])
    );

    use rusttype::Scale;
    use imageproc::drawing::draw_text_mut;
    use image::Rgba;

    let scale = Scale { x: 20.0, y: 20.0 };
    draw_text_mut(
        &mut img,
        Rgba([255, 255, 0, 255]),
        10,
        40,
        scale,
        FONT.as_ref().unwrap(),
        &text
    );

    // Codifica em PNG sem warnings de depreciação:
    let mut buf = Vec::new();
    let encoder = image::codecs::png::PngEncoder::new(&mut buf);
    encoder
        .write_image(
            &img,
            width,
            height,
            image::ColorType::Rgba8
        )
        .unwrap();

    // Convertemos para base64
    use base64::{Engine as _, engine::general_purpose};
    let encoded = general_purpose::STANDARD.encode(&buf);

    (StatusCode::OK, Json(serde_json::json!({ "image": encoded }))).into_response()
}

// ======================
// CRIA O ROUTER
// ======================
fn create_router() -> Router {
    use tower::layer::layer_fn;

    Router::new()
        .route("/math", post(math_operations))
        .route("/json", post(json_manipulation))
        .route("/string", post(string_processing))
        .route("/compress", post(compress_data))
        .route("/image", post(image_processing))
        .layer(layer_fn(|service| TimingLayer.layer(service)))
}

// ======================
// MAIN LOCAL
// ======================
#[cfg(not(feature = "lambda"))]
#[tokio::main]
async fn main() {
    let app = create_router();
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("Rodando local em http://127.0.0.1:3000");

    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// ======================
// MAIN LAMBDA
// ======================
#[cfg(feature = "lambda")]
#[tokio::main]
async fn main() -> Result<(), LambdaError> {
    let app = create_router();

    // Converte o Router em um Service compatível com lambda_http
    let handler = lambda_http::tower::ServiceBuilder::new()
        .layer(lambda_http::CompressionLayer::new()) // opcional
        .service(app);

    lambda_run(handler).await?;
    Ok(())
}
