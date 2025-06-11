use crate::utils::{SessionMap, launch_command};
use axum::extract::{Json, State};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize)]
pub struct ReceiveRequest {
    pub port: String,
}

#[derive(Serialize)]
pub struct ReceiveResponse {
    status: String,
    session_id: Option<String>,
}

#[derive(Deserialize)]
pub struct StopReceiveRequest {
    pub session_id: String,
}

#[derive(Serialize)]
pub struct StopReceiveResponse {
    status: String,
    session_id: Option<String>,
}

pub async fn receive(
    State(session_store): State<SessionMap>,
    Json(payload): Json<ReceiveRequest>,
) -> Json<ReceiveResponse> {
    let session_id = Uuid::new_v4().to_string();

    let cmd = format!(
        "exec gst-launch-1.0 udpsrc port={} caps=\"application/x-rtp,media=audio,encoding-name=L16,clock-rate=44100,channels=2,payload=96\" \
         ! queue ! rtpL16depay ! audioconvert ! autoaudiosink",
        payload.port
    );

    match launch_command("receiver", &cmd) {
        Ok(child) => {
            let mut map = session_store.lock().unwrap();
            map.insert(session_id.clone(), child);
            Json(ReceiveResponse {
                status: "success".to_string(),
                session_id: Some(session_id),
            })
        }
        Err(_e) => Json(ReceiveResponse {
            status: "fail".to_string(),
            session_id: None,
        }),
    }
}

pub async fn stop_receive(
    State(session_store): State<SessionMap>,
    Json(payload): Json<StopReceiveRequest>,
) -> Json<StopReceiveResponse> {
    let mut map = session_store.lock().unwrap();

    if let Some(mut child) = map.remove(&payload.session_id) {
        match child.kill() {
            Ok(_) => {
                let _ = child.wait();
                Json(StopReceiveResponse {
                    status: "success".to_string(),
                    session_id: None,
                })
            }
            Err(e) => {
                eprintln!("Failed to kill GStreamer pipeline: {}", e);
                Json(StopReceiveResponse {
                    status: "fail".to_string(),
                    session_id: Some(payload.session_id),
                })
            }
        }
    } else {
        Json(StopReceiveResponse {
            status: "fail".to_string(),
            session_id: Some(payload.session_id),
        })
    }
}
