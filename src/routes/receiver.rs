use axum::extract::Json;
use serde::Deserialize;
use crate::utils::run_command;

#[derive(Deserialize)]
pub struct ReceiveRequest {
    pub port: String,
}

pub async fn receive(Json(payload): Json<ReceiveRequest>) -> &'static str {
    let cmd = format!(
        "gst-launch-1.0 udpsrc port={} caps=\"application/x-rtp,media=audio,encoding-name=L16,clock-rate=44100,channels=2,payload=96\" \
         ! queue ! rtpL16depay ! audioconvert ! autoaudiosink",
        payload.port
    );

    run_command("receiver", &cmd);

    "Receiver started"
}


