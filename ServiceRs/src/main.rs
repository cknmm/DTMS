use std::sync::OnceLock;

use axum::extract::Request;
use axum::middleware::{self, Next};
use http_body_util::Full;
use hyper::body::Bytes;
use axum::http::Response;
use axum::{Router, Json, routing::get};
use rand::Rng;
use serde_json::{Value, json};
use tokio::net::TcpListener;

//OTel imports
use opentelemetry::global::{self, BoxedTracer};
use opentelemetry::trace::{SpanKind, Tracer};
use opentelemetry_sdk::{Resource};
use opentelemetry_sdk::trace::SdkTracerProvider;
use opentelemetry_sdk::metrics::SdkMeterProvider;
use opentelemetry_otlp::{SpanExporter, MetricExporter};

//Globals
const SERVICE_NAME: &'static str = "axumMicroserviceRs";

fn init_router() -> Router {

  Router::new()
    .route("/api", get(handle_api))
    .layer(middleware::from_fn(emit_span_from_middleware)) //Add span generation as a middleware
}

async fn handle_api() -> Result<Json<Value>, Response<Full<Bytes>>> {
  let random_float: f64 = rand::thread_rng().r#gen();
  //70% chance of success
  if random_float <= 0.7 {
    return Ok(Json(json!({
      "value": random_float
    })));
  }
  return Err(Response::builder()
    .status(400)
    .body(Full::new(Bytes::from("Failed due to chances!")))
    .unwrap());
}

//<OTel Utils>

fn get_tracer() -> &'static BoxedTracer {
  static TRACER: OnceLock<BoxedTracer> = OnceLock::new();
  TRACER.get_or_init(|| global::tracer(SERVICE_NAME))
}

fn init_tracer_and_metrics() {

  //Tracer and spans
  let span_exporter = SpanExporter::builder()
    .with_tonic()
    .build()
    .expect("Failed to initialize span exporter");
  let provider = SdkTracerProvider::builder()
    .with_resource(
      Resource::builder()
        .with_service_name(SERVICE_NAME)
        .build()
    )
    .with_batch_exporter(span_exporter)
    .build();
  global::set_tracer_provider(provider);

  //Metrics
  let metric_exporter = MetricExporter::builder()
    .with_tonic()
    .build()
    .expect("Failed to initialize metric exporter");
  let metric_provider = SdkMeterProvider::builder()
    .with_resource(
      Resource::builder()
        .with_service_name(SERVICE_NAME)
        .build()
    )
    .with_periodic_exporter(metric_exporter)
    .build();
  global::set_meter_provider(metric_provider);
}

async fn emit_span_from_middleware(request: Request, next: Next) -> Response<axum::body::Body> {
  //Start OTel instrumentation
  let tracer = get_tracer();
  let mut _span= tracer
    .span_builder(format!("{}: {}!", SERVICE_NAME, request.uri().to_string()))
    .with_kind(SpanKind::Server)
    .start(tracer);
  return next.run(request).await;
}

//</OTel Utils>

#[tokio::main]
async fn main() {

  //Initialize OTel providers
  init_tracer_and_metrics();

  let app = init_router();

  // Bind to a port using tokio::net::TcpListener
  let addr = "127.0.0.1:4000";
  let listener = TcpListener::bind(addr).await.unwrap();

  println!("Listening on {}", addr);

  // Use axum::serve with the listener and the router
  axum::serve(listener, app)
      .await
      .unwrap();
}
