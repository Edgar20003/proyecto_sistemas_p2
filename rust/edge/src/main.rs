use axum::{routing::post, Json, Router};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct SensorData {
    id: u32,
    value: f32,
    timestamp: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct FinalData {
    sensor_id: u32,
    value: f32,
    processed_by: String,
    timestamp: String,
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/data", post(handle_data));
    println!("⚙️ [EDGE] Intermediario activo en el puerto 4000...");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:4000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn handle_data(Json(payload): Json<SensorData>) {
    println!("🧐 [EDGE] Recibido de Sensor {}. Reenviando al Coordinador...", payload.id);
    let client = reqwest::Client::new();
    let final_data = FinalData {
        sensor_id: payload.id,
        value: payload.value,
        processed_by: "edge-01-vmware".to_string(),
        timestamp: payload.timestamp,
    };
    // Reenvío al Coordinador
    let _ = client.post("http://localhost:3000/ingest")
        .json(&final_data)
        .send()
        .await;
}
