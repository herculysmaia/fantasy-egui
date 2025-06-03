// Tela inicial da aplicação

// Widget para a tela inicial da aplicação
use eframe::egui::{self, Button, Color32, RichText, Vec2};

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

            // Coluna centralizada para o conteúdo
            ui.vertical_centered(|ui| {

                // Definindo tamanho padrão do botão e espaço disponível verticalmente
                let tamanho_botao = Vec2::new(100.0, 20.0);
                let altura_da_janela = ui.available_height();

                // Aproximando conteúdo do centro da tela
                ui.add_space(altura_da_janela / 4.0);

                // Adicionando título e subtítulo
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

        // Retornando a ação da tela
        acao
    }
}