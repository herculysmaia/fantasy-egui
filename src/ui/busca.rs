// Tela para buscar os times participantes da liga 

// Widget para a tela de busca da aplicação
use eframe::egui::{self, Color32};
use egui_extras::{Size, StripBuilder};

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
            
            // Dividindo tela em três blocos empilhados na vertical
            StripBuilder::new(ui)
                .size(Size::exact(20.0))
                .size(Size::remainder())
                .size(Size::exact(20.0))
                .vertical(|mut strip| {

                    // Compo para buscar time a ser adicionado
                    strip.cell(|ui| {

                        // Variavel auxiliar para justificar campo de entrada de texto
                        let available_width = ui.available_width();

                        // Deixar hint_text como cinza e texto digitado como preto
                        ui.style_mut().visuals.override_text_color = Some(Color32::GRAY);

                        // Adicionar campo de entrada de texto para buscar time
                        if ui.add_sized([available_width, 0.0], egui::TextEdit::singleline(&mut self.busca_time).hint_text("Nome do time").text_color(Color32::BLACK)).changed() {
                            println!("Busca iniciada: {}", self.busca_time);
                        };

                    });

                    // Espaço para a lista de times encontrados
                    strip.cell(|ui| {

                        // Deixando o fundo branco
                        ui.painter().rect_filled(
                            ui.available_rect_before_wrap(),
                            5.0,
                        Color32::WHITE);

                    });

                    // Rodapé da tela
                    strip.cell(|ui| {

                        // Centralizar o conteúdo 
                        ui.horizontal_centered(|ui| {

                            // Botão para voltar à tela inicial
                            if ui.button("Voltar").clicked() {
                                acao = Some(AcaoDaTela::Voltar);
                            }

                        });

                    });

                });

        });

        // Retornando a ação da tela
        acao
    }
}