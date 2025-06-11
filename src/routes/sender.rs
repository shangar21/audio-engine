use crate::utils::{launch_command, SessionMap};
use axum::extract::{Json, State};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize)]
pub struct SendRequest {
    source: String,
    device: String,
    host: String,
    port: String,
}

#[derive(Serialize)]
pub struct SendResponse {
    host: String,
    port: String,
    status: String,
    session_id: Option<String>,
}

#[derive(Deserialize)]
pub struct StopSendRequest {
    session_id: String,
}

#[derive(Serialize)]
pub struct StopSendResponse {
    status: String,
    session_id: Option<String>,
}

pub async fn send(
    State(session_store): State<SessionMap>,
    Json(payload): Json<SendRequest>,
) -> Json<SendResponse> {
    let session_id = Uuid::new_v4().to_string();
    let command = format!(
        "gst-launch-1.0 {} device={} ! audio/x-raw.format=S16LE,channels=2,rate=44100 ! audioconvert ! rtpL16pay ! \"application/x-rtp,media=audio,encoding-name=L16,clock-rate=44100,payload=96\" ! udpsink host={} port={}",
        payload.source, payload.device, payload.host, payload.port
    );

    match launch_command("sender", &command) {
        Ok(child) => {
            let mut map = session_store.lock().unwrap();
            map.insert(session_id.clone(), child);
            Json(SendResponse {
                host: payload.host,
                port: payload.port,
                status: "success".to_string(),
                session_id: Some(session_id),
            })
        }
        Err(_e) => Json(SendResponse {
            host: payload.host,
            port: payload.port,
            status: "fail".to_string(),
            session_id: None,
        }),
    }
}

pub async fn stop_send(
    State(session_store): State<SessionMap>,
    Json(payload): Json<StopSendRequest>,
) -> Json<StopSendResponse> {
    let mut map = session_store.lock().unwrap();

    if let Some(mut child) = map.remove(&payload.session_id) {
        let _ = child.kill();
        Json(StopSendResponse {
            status: "success".to_string(),
            session_id: None,
        })
    } else {
        Json(StopSendResponse {
            status: "fail".to_string(),
            session_id: Some(payload.session_id),
        })
    }
}
