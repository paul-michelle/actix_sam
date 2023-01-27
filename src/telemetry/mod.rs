use tracing::{subscriber::set_global_default, Subscriber};
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::{fmt::MakeWriter, layer::SubscriberExt, EnvFilter, Registry};

pub fn get_subscriber<Sink>(
    subsriber_name: String,
    filter_level: String,
    sink: Sink,
) -> Box<dyn Subscriber + Send + Sync>
where
    Sink: for<'a> MakeWriter<'a> + Send + Sync + 'static,
{
    let env_filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(filter_level));
    let fmt_layer = BunyanFormattingLayer::new(subsriber_name, sink);

    Box::new(
        Registry::default()
            .with(env_filter)
            .with(JsonStorageLayer)
            .with(fmt_layer),
    )
}

pub fn init_subscriber(subscriber: impl Subscriber + Send + Sync) {
    // Turn 3rd-party libs' logs into tracing events:
    LogTracer::init().expect("Failed to init LogTracer.");

    set_global_default(subscriber).expect("Failed to set a tracing subsriber");
}
