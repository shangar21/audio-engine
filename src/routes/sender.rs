use axum::extract::Json;
use serde::Deserialize;
use std::process::Command;

#[derive(Deserialize)]
pub struct SendRequest {
    source: String,
    device: String,
    host: String,
    port: String,
}

pub async fn send(Json(payload): Json<SendRequest>) {
    let command = format!(
        "gst-launch-1.0 {} device={} ! audio/x-raw.format=S16LE,channels=2,rate=44100 ! audioconvert ! rtpL16pay ! \"application/x-rtp,media=audio,encoding-name=L16,clock-rate=44100,payload=96\" ! udpsink host={} port={}",
        payload.source, payload.device, payload.host, payload.port
    );
    let _ = Command::new("sh")
        .arg("-c")
        .arg(&command)
        .spawn()
        .expect("Failed to launch gstream send pipeline!");
}
