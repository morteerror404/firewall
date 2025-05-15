pub fn generate_md_report(results: Vec<AnalysisResult>) -> String {
    let mut report = String::new();
    
    report.push_str("# Relatório de Ameaças Detectadas\n\n");
    report.push_str("## Resumo Executivo\n");
    report.push_str(&format!("- Total de eventos: {}\n", results.len()));
    
    // Detalhes por categoria
    report.push_str("\n## Análise Detalhada\n");
    for result in results {
        report.push_str(&format!(
            "### Evento {}\n- **Risco**: {}\n- **Ações**: {}\n\n",
            result.event.action,
            result.risk_score,
            result.recommended_actions.join(", ")
        ));
    }
    
    report
}