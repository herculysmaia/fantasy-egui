// Tela para buscar os times participantes da liga 

// Widget para a tela de busca da aplicação
use eframe::egui;

// Importando a estrutura de dados e a enumeração de telas
use super::{AcaoDaTela, Tela};

// Estrura de dados para a tela de busca da aplicação
#[derive(Default)]
pub struct BuscaData {
    pub busca_time: String,
}

// Implementando comportamento padrão para a tela de busca
impl Tela for BuscaData {

    // Defininindo a interface gráfica da tela inicial
    fn exibir(&mut self, ctx: &egui::Context) -> Option<AcaoDaTela> {

        // Definindo a ação padrão da tela
        let mut acao = None;

        // Definindo o painel central da tela
        egui::CentralPanel::default().show(ctx, |ui| {

            // Título da tela
            ui.heading("Buscar Time");

            // Campo para entrada de texto para busca
            ui.text_edit_singleline(&mut self.busca_time);

            // Botão para voltar à tela inicial
            if ui.button("Voltar").clicked() {
                acao = Some(AcaoDaTela::Voltar);
            }
        });

        // Retornando a ação da tela
        acao
    }
}