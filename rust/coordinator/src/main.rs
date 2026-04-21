use axum::{routing::post, Json, Router};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct FinalData {
    sensor_id: u32,
    value: f32,
    processed_by: String,
    timestamp: String,
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/ingest", post(handle_ingest));
    println!("🚀 [COORDINADOR] Esperando datos en el puerto 3000...");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn handle_ingest(Json(payload): Json<FinalData>) {
    println!("📥 [DATO RECIBIDO] De: {} | Valor: {:.2} | Por: {}", 
             payload.sensor_id, payload.value, payload.processed_by);
}
