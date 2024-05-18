// use opentelemetry::KeyValue;
// use opentelemetry_otlp::WithExportConfig;
// use opentelemetry_sdk::runtime;
// use opentelemetry_sdk::trace;
// use opentelemetry_sdk::trace::Tracer;

// use opentelemetry_sdk::Resource;
// use opentelemetry_semantic_conventions::resource::SERVICE_NAME;
// use tracing_opentelemetry::OpenTelemetryLayer;
// use tracing_subscriber::layer::SubscriberExt;
// use tracing_subscriber::util::SubscriberInitExt;

// pub fn init_tracer() -> Tracer {
//     opentelemetry_otlp::new_pipeline()
//         .tracing()
//         .with_exporter(
//             opentelemetry_otlp::new_exporter()
//                 .tonic()
//                 .with_endpoint("http://localhost:4317"),
//         )
//         .with_trace_config(
//             trace::config()
//                 .with_resource(Resource::new(vec![KeyValue::new(SERVICE_NAME, "app_fmm")])),
//         )
//         .install_batch(runtime::Tokio)
//         .unwrap()
// }

// pub fn init_tracing_subscriber() {
//     let tracer = init_tracer();

//     let layer = OpenTelemetryLayer::new(tracer);
//     tracing_subscriber::registry()
//         // .with(tracing_subscriber::filter::LevelFilter::from_level(
//         //     Level::INFO,
//         // ))
//         .with(tracing_subscriber::fmt::layer())
//         // .with(MetricsLayer::new(meter_provider.clone()))
//         .with(tracing_opentelemetry::layer().with_tracer(tracer))
//         .init();
// }

// // fn std_trace() {
// //     // Setup LoggerProvider with a stdout exporter
// //     let exporter = opentelemetry_stdout::LogExporterBuilder::default()
// //         .with_encoder(|writer, data| {
// //             serde_json::to_writer_pretty(writer, &data).unwrap();
// //             Ok(())
// //         })
// //         .build();
// //     let logger_provider = LoggerProvider::builder()
// //         .with_config(
// //             Config::default()
// //                 .with_resource(Resource::new(vec![KeyValue::new(SERVICE_NAME, "app_fmm")])),
// //         )
// //         .with_simple_exporter(exporter)
// //         .build();

// //     // Setup Log Appender for the log crate.
// //     let otel_log_appender = OpenTelemetryLogBridge::new(&logger_provider);
// //     log::set_boxed_logger(Box::new(otel_log_appender)).unwrap();
// //     log::set_max_level(Level::Info.to_level_filter());
// // }
