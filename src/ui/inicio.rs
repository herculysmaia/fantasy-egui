// Tela inicial da aplicação

// Widget para a tela inicial da aplicação
use eframe::egui::{self, Button, Color32, RichText, Vec2};
use egui_extras::{Size, StripBuilder};

// Importando a estrutura de dados e a enumeração de telas
use super::{AcaoDaTela, Tela};
use super::busca::BuscaData;

// Estrura de dados para a tela inicial da aplicação
#[derive(Default)]
pub struct Inicio;

// Implementando comportamento padrão para a tela inicial
impl Tela for Inicio {

    // Defininindo a interface gráfica da tela inicial
    fn exibir(&mut self, ctx: &egui::Context) -> Option<AcaoDaTela> {

        // Definindo a ação padrão da tela
        let mut acao = None;

        // Definindo o painel central da tela
        egui::CentralPanel::default().show(ctx, |ui| {

            // Esboço da nova estrutura de organização de widgets 
            StripBuilder::new(ui)
                .size(Size::remainder())
                .size(Size::initial(300.0))
                .size(Size::remainder())
                .vertical(|mut strip | {
                    
                    // Espaçar conteúdo da borda
                    strip.empty();

                    // Conteúdo do centro da tela
                    strip.cell(|ui| {
                        ui.vertical_centered(|ui| {

                            // Definir espaço para receber o botão
                            let tamanho_botao = Vec2::new(100.0, 20.0);
                        
                            // Título e subtitulo
                            ui.label(RichText::new("LIGA MELHOR DA RODADA").strong().size(30.0).color(Color32::WHITE));
                            ui.label(RichText::new("Ano 9").color(Color32::WHITE));
                        
                            // Adicionando espaço entre o título e os botões
                            ui.add_space(50.0);
            
                            // Botões do menu inicial
                            if ui.add_sized(tamanho_botao, Button::new("Buscar Time")).clicked() {
                                acao = Some(AcaoDaTela::Push(Box::new(BuscaData::default())))
                            }   
                        });
                    });

                    // Espaçar conteúdo da borda
                    strip.empty();
                });
        });

        // Retornando a ação da tela
        acao
    }
}