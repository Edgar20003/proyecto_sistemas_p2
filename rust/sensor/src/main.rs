use serde::Serialize;
use std::time::Duration;
use tokio::time::sleep;
use rand::RngExt; // <--- Este es el cambio clave para la versión 0.10

#[derive(Serialize)]
struct SensorData {
    id: u32,
    value: f32,
    timestamp: String,
}

#[tokio::main]
async fn main() {
    let client = reqwest::Client::new();
    let mut rng = rand::rng(); 
    println!("📡 [SENSOR] Iniciando generación de datos IoT...");

    loop {
        let data = SensorData {
            id: 1,
            value: rng.random_range(18.0..32.0), 
            timestamp: chrono::Utc::now().to_rfc3339(),
        };
        
        println!("📤 Enviando a Edge: {:.2}°C", data.value);
        
        // Enviamos al puerto 4000 donde escucha el Edge
        let _ = client.post("http://localhost:4000/data")
            .json(&data)
            .send()
            .await;

        sleep(Duration::from_secs(5)).await;
    }
}
