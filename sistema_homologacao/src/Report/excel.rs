pub fn generate_excel_report(results: Vec<AnalysisResult>) -> Result<Vec<u8>> {
    let mut workbook = Workbook::new();
    let sheet = workbook.add_worksheet();
    
    // Cabeçalhos
    sheet.write_string(0, 0, "Timestamp")?;
    sheet.write_string(0, 1, "Ação")?;
    sheet.write_string(0, 2, "Score de Risco")?;
    
    // Dados
    for (i, result) in results.iter().enumerate() {
        let row = i as u32 + 1;
        sheet.write_string(row, 0, &result.event.timestamp)?;
        sheet.write_string(row, 1, &result.event.action)?;
        sheet.write_number(row, 2, result.risk_score)?;
    }
    
    let mut buffer = Vec::new();
    workbook.save_to_buffer(&mut buffer)?;
    Ok(buffer)
}