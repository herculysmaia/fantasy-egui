// Tela para buscar os times participantes da liga 

// Widget para a tela de busca da aplicação
use eframe::egui;

// Importando a estrutura de dados e a enumeração de telas
use super::{AppData, Tela};

// Implementando métodos para exibir a tela de busca a estrutura de dados da aplicação
impl AppData {

    // Método para inicializar a tela de busca
    pub fn show_busca_time(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        self.busca(ctx, frame);
    }

    // Método que define o conteúdo da tela de busca
    fn busca(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        // Definindo o painel central da tela
        egui::CentralPanel::default().show(ctx, |ui| {

            // Identificando a tela de busca
            ui.label("Conteúdo da Busca");

            // Botão para retornar à tela inicial
            if ui.button("Voltar").clicked() {
                self.tela = Tela::Inicio;
            }
            
        });
    }
}