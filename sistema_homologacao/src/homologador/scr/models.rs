use serde::{Deserialize, Serialize};
use std::sync::Mutex;

#[derive(Debug, Serialize, Deserialize)]
pub struct HomologationRequest {
    pub packet_data: String,
    pub protocol: String,
    pub source_ip: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HomologationResult {
    pub id: u64,
    pub request: HomologationRequest,
    pub risk_score: f32,
    pub decision: String,
    pub triggers: Vec<String>,
}

pub struct HomologationData {
    results: Vec<HomologationResult>,
    next_id: u64,
}

impl HomologationData {
    pub fn new() -> Self {
        Self {
            results: Vec::new(),
            next_id: 1,
        }
    }
    
    pub fn add_result(&mut self, result: HomologationResult) {
        self.results.push(result);
    }
    
    pub fn get_all(&self) -> Vec<HomologationResult> {
        self.results.clone()
    }
}

pub fn process_homologation(request: HomologationRequest) -> HomologationResult {
    // Lógica de análise simplificada
    let risk_score = calculate_risk(&request);
    let (decision, triggers) = analyze_triggers(&request);
    
    HomologationResult {
        id: 1, // Será sobrescrito pelo HomologationData
        request,
        risk_score,
        decision,
        triggers,
    }
}

fn calculate_risk(request: &HomologationRequest) -> f32 {
    // Lógica de cálculo de risco simplificada
    let mut score = 0.0;
    
    if request.protocol == "TCP" {
        score += 30.0;
    }
    
    if request.packet_data.contains("SELECT") {
        score += 40.0;
    }
    
    if request.source_ip.starts_with("10.0.0.") {
        score += 20.0;
    }
    
    score.min(100.0)
}

fn analyze_triggers(request: &HomologationRequest) -> (String, Vec<String>) {
    let mut triggers = Vec::new();
    
    if request.packet_data.len() > 1024 {
        triggers.push("Pacote grande".to_string());
    }
    
    if request.protocol == "UDP" && request.packet_data.contains("admin") {
        triggers.push("Comando sensível em UDP".to_string());
    }
    
    let decision = match triggers.len() {
        0 => "Aceitar".to_string(),
        1..=2 => "Analisar manualmente".to_string(),
        _ => "Rejeitar".to_string(),
    };
    
    (decision, triggers)
}