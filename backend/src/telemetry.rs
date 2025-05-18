use ::tracing_subscriber::FmtSubscriber;
use ::tracing_subscriber::util::SubscriberInitExt;

pub async fn init() {
    FmtSubscriber::builder()
        .with_file(true)
        .with_line_number(true)
        .finish()
        .init();
}
