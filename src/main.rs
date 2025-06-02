/*
Aplicaçao para o gerenciamento de ligas de futebol, focada na melhor pontuação da rodada.
Versão: 0.1
Desenvolvido por: Herculys S. Maia
Data de criação: 01/06/2025
Licença: MIT License
*/

// Interface gráfica do aplicativo
mod ui;

// Iniciando a aplicação
fn main() -> eframe::Result<()> {

    // Configurações iniciais
    eframe::run_native(
        "Liga Melhor da Rodada - Ano 9", 
        eframe::NativeOptions::default(),
        Box::new(|cc: &eframe::CreationContext<'_>| Ok(Box::new(ui::AppData::new(cc))))
    )

}