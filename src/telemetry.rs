use tracing::{subscriber::set_global_default, Subscriber};
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};

pub fn init_subscriber(name: String, env_filter: String) {
    LogTracer::init().expect("Failed to set logger");
    let env_filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(env_filter));
    let formatting_layer = BunyanFormattingLayer::new(name, std::io::stdout);
    set_global_default(
        Registry::default()
            .with(env_filter)
            .with(JsonStorageLayer)
            .with(formatting_layer),
    ).expect("init subs error")
}
