use std::{
    fs::OpenOptions,
    io::Write,
    time::SystemTime
};
use docker_api::Docker;

pub async fn start_monitoring() {
    let docker = Docker::unix("/var/run/docker.sock");
    let mut events = docker.events(&Default::default()).await.unwrap();
    
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("honeypot_events.csv")
        .unwrap();

    while let Some(event) = events.next().await {
        let timestamp = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        let record = format!(
            "{},{},{},{}\n",
            timestamp,
            event.action,
            event.actor.id,
            serde_json::to_string(&event).unwrap()
        );
        
        file.write_all(record.as_bytes()).unwrap();
        
        // Envia para sistema de homologação
        homologation::process_event(event).await;
    }
}