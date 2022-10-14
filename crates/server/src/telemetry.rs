use crate::config::{LogFormat, SETTINGS};
use opentelemetry::{
    global, runtime::TokioCurrentThread, sdk::propagation::TraceContextPropagator,
};
use tracing::Subscriber;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::{fmt::MakeWriter, layer::SubscriberExt, EnvFilter, Registry};

pub fn get_subscriber<Sink>(
    app_name: String,
    env_filter: String,
    opentelemetry_jaeger: bool,
    sink: Sink,
) -> impl Subscriber + Sync + Send
where
    Sink: for<'a> MakeWriter<'a> + Send + Sync + 'static,
{
    let env_filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(env_filter));

    let formatting_layer_json = match (*SETTINGS).log_format() {
        LogFormat::Json => Some(BunyanFormattingLayer::new(app_name.clone(), sink)),
        LogFormat::Default => None,
    };

    let formatting_layer_default = match (*SETTINGS).log_format() {
        LogFormat::Json => None,
        LogFormat::Default => Some(tracing_subscriber::fmt::Layer::default()),
    };

    let telemetry_layer = match opentelemetry_jaeger {
        true => {
            global::set_text_map_propagator(TraceContextPropagator::new());
            let tracer = opentelemetry_jaeger::new_agent_pipeline()
                .with_service_name(app_name)
                .install_batch(TokioCurrentThread)
                .expect("Failed to install OpenTelemetry tracer.");
            let telemetry = tracing_opentelemetry::layer().with_tracer(tracer);

            Some(telemetry)
        }
        false => None,
    };

    Registry::default()
        .with(env_filter)
        .with(telemetry_layer)
        .with(JsonStorageLayer)
        .with(formatting_layer_json)
        .with(formatting_layer_default)
}

pub fn init_subscriber(subscriber: impl Subscriber + Sync + Send) {
    LogTracer::init().expect("Failed to set logger");

    tracing::subscriber::set_global_default(subscriber)
        .expect("Failed to install `tracing` subscriber.")
}

pub fn init_tracing() {
    let subscriber = get_subscriber(
        (*SETTINGS).app_name().clone(),
        "info".into(),
        *(*SETTINGS).telemetry(),
        std::io::stdout,
    );

    init_subscriber(subscriber)
}
