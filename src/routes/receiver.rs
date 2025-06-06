use axum::extract::Json;
use serde::Deserialize;
use std::process::Command;


#[derive(Deserialize)]
pub struct ReceiveRequest {
    port: String,
}

pub async fn receive(Json(payload): Json<ReceiveRequest>) {
    let command = format!(
        "gst-launch-1.0 udpsrc port={} caps=\"application/x-rtp,media=audio,encoding-name=L16,clock-rate=44100,channels=2,payload=96\" ! queue ! rtpL16depay ! audioconvert ! autoaudiosink",
        payload.port
    );

    let _ = Command::new("sh")
        .arg("-c")
        .arg(&command)
        .spawn()
        .expect("Failed to launch gstream receive pipeline!");
}
