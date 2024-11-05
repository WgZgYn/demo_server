use std::time::Duration;
use log::error;
use rumqttc::{AsyncClient, Event, EventLoop, Incoming, MqttOptions};
use crate::utils::config::MqttConfig;

pub async fn handle_mqtt_message(mut event_loop: EventLoop) {
    loop {
        match event_loop.poll().await {
            Ok(e) => {
                if let Event::Incoming(Incoming::Publish(publish)) = e {
                    println!("mqtt publish: {:?}", publish);
                }
            }
            Err(e) => {
                error!("{}", e);
            }
        }
    }
}

pub fn mqtt(cfg: &MqttConfig) -> (AsyncClient, EventLoop) {
    let mut options = MqttOptions::new(&cfg.id, &cfg.host, cfg.port);
    options.set_keep_alive(Duration::from_secs(cfg.keep_alive));
    let (client, event_loop) = AsyncClient::new(options, cfg.cap);
    (client, event_loop)
}