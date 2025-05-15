pub struct EventAnalyzer {
    pub threats: Vec<ThreatPattern>,
    pub behavior_profiles: HashMap<String, BehaviorProfile>
}

impl EventAnalyzer {
    pub fn new() -> Self {
        Self {
            threats: load_threat_patterns(),
            behavior_profiles: HashMap::new()
        }
    }
    
    pub fn analyze(&mut self, event: DockerEvent) -> AnalysisResult {
        // 1. Detecção de padrões conhecidos
        let pattern_match = self.threats.iter()
            .find(|p| p.matches(&event));
        
        // 2. Análise comportamental
        let actor = event.actor.id.clone();
        let profile = self.behavior_profiles
            .entry(actor)
            .or_insert(BehaviorProfile::new());
        
        profile.update(&event);
        
        // 3. Geração de veredito
        AnalysisResult {
            event,
            pattern_match,
            risk_score: profile.risk_score(),
            recommended_actions: profile.recommend_actions()
        }
    }
}