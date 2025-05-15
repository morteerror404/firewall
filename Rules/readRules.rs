use std::fs;

fn main() {
    // Lê o arquivo JSON
    let json_data = fs::read_to_string("rules.json")
        .expect("Falha ao ler o arquivo JSON");

    // Desserializa para a struct FirewallConfig
    let config: FirewallConfig = serde_json::from_str(&json_data)
        .expect("Falha ao parsear JSON");

    // Itera sobre as regras
    for rule in config.rules {
        println!("Regra: {:?}", rules);
        
        // Exemplo: Aplicar regra (simulação)
        match rule.action.as_str() {
            "allow" => println!("✅ Permitir: {}", rule.name),
            "deny" => println!("❌ Bloquear: {}", rule.name),
            "log" => println!("📝 Logar: {}", rule.name),
            _ => println!("⚡ Ação desconhecida"),
        }
    }
}