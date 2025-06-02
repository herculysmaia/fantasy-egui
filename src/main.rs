/*
Aplicaçao para o gerenciamento de ligas de futebol, focada na melhor pontuação da rodada.
Versão: 0.1
Desenvolvido por: Herculys S. Maia
Data de criação: 01/06/2025
Licença: MIT License
*/

// Interface gráfica, api e base de dados do aplicativo
mod ui;
mod db;

// Iniciando a aplicação
fn main() {

    // Configurações iniciais do banco de dados
    db::create_tables().unwrap_or_else(|e| { eprintln!("Erro ao criar tabelas: {:?}", e) });

    // Iniciando a interface gráfica
    interface().unwrap_or_else(|e| { eprintln!("Erro ao inicial aplicação: {:?}", e) });

}

fn interface() -> eframe::Result<()> {

    // Configurações iniciais
    eframe::run_native(
        "Liga Melhor da Rodada - Ano 9", 
        eframe::NativeOptions::default(),
        Box::new(|cc: &eframe::CreationContext<'_>| Ok(Box::new(ui::AppData::new(cc))))
    )

} 