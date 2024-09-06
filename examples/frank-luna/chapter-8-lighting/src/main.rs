

mod land_and_waves_sample;
mod shape_sample;

use common::run_sample;
#[allow(unused_imports)]
use land_and_waves_sample::LandAndWavesSample;
#[allow(unused_imports)]
use shape_sample::ShapesSample;
use tracing_subscriber::layer::SubscriberExt;

fn main() {
    let console_log = tracing_subscriber::fmt::Layer::new()
        .with_ansi(true)
        .with_writer(std::io::stdout);

    let subscriber = tracing_subscriber::registry().with(console_log);

    let _ = tracing::subscriber::set_global_default(subscriber);
    run_sample::<LandAndWavesSample>();
}
