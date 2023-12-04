use std::time::Duration;

use prometheus::{labels, register_counter};
use prometheus_push::prometheus_crate::PrometheusMetricsPusher;
use reqwest::Client;
use tokio::time::sleep;
use url::Url;

#[tokio::main]
async fn main() {
    tokio::spawn(async move {
        loop {
            sleep(Duration::from_secs(10)).await;
            push().await;
        }
    });
    let counter = register_counter!("zigglewiggle", "wiggle it a little").unwrap();

    loop {
        sleep(Duration::from_secs(1)).await;
        counter.inc();
    }
}

async fn push() {
    let gateway: Url = "http://127.0.0.1:9091".parse().unwrap();
    let client = Client::new();
    let metrics_pusher = PrometheusMetricsPusher::from(client, &gateway).unwrap();
    let _result = metrics_pusher
        .push_all(
            "vlogwort",
            &labels! { "ziggle" => "wiggle" },
            prometheus::gather(),
        )
        .await;
}
