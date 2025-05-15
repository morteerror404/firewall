use pnet::packet::{ip::IpNextHeaderProtocols, tcp::TcpPacket};

pub struct TrafficAnalyzer {
    pub triggers: Vec<Trigger>,
}

impl TrafficAnalyzer {
    pub fn new() -> Self {
        Self {
            triggers: load_triggers(),
        }
    }

    pub fn analyze_tcp(&self, packet: &TcpPacket) -> AnalysisResult {
        let mut result = AnalysisResult::default();
        
        for trigger in &self.triggers {
            if trigger.matches_tcp(packet) {
                result.risk_level += trigger.risk_score;
                result.triggers.push(trigger.name.clone());
            }
        }
        
        result
    }

    pub fn should_redirect(&self, result: &AnalysisResult) -> bool {
        result.risk_level >= self.threshold()
    }

    fn threshold(&self) -> f32 {
        // Lógica para determinar threshold dinâmico
        75.0
    }
}