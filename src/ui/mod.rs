// Tratamendo de telas da aplicação

// Telas acessiveis no inicio da aplicação
mod inicio;
mod busca;
mod theme;

// Deixando egui acessível
use eframe::egui;

// Definido enumeração para navegação entre telas
pub enum AcaoDaTela {
    None,
    Push(Box<dyn Tela>),
    Voltar,
}

// Definindo comportamento padrão para as telas da aplicação
pub trait Tela {

    // Exibir conteudo da tela
    fn exibir(&mut self, ctx: &egui::Context) -> Option<AcaoDaTela>;

    // Tratar ação da tela
    fn show(&mut self, ctx: &egui::Context) -> AcaoDaTela {
        if let Some(acao) = self.exibir(ctx) {
            return acao;
        };
        AcaoDaTela::None
    }
    
}

// Estrutura de dados da aplicação
pub struct AppData {
    tela_stack: Vec<Box<dyn Tela>>,
}

// Implementando métodos para a estrutura de dados da aplicação
impl AppData {

    // Método para inicializar a aplicação
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {

        // Configurando fontes e estilo do egui
        cc.egui_ctx.set_fonts(theme::load_font());
        cc.egui_ctx.set_style(theme::style());

        // Retornando a instância da aplicação com a tela inicial
        Self {
            tela_stack: vec![Box::new(inicio::Inicio::default())],
        }
    
    }
}

// Implementando a trait App para a estrutura de dados da aplicação
impl eframe::App for AppData {

    // Método chamado a cada atualização da aplicação
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        // Definindo qual tela deve ser exibida
        if let Some(tela) = self.tela_stack.last_mut() {

            // Exibindo a tela atual e tratando ações da tela
            match tela.show(ctx) {
                AcaoDaTela::None => {},
                AcaoDaTela::Push(tela_nova) => self.tela_stack.push(tela_nova),
                AcaoDaTela::Voltar => {

                    // Garantir que sempre voltará até no máximo a tela inicial
                    self.tela_stack.pop();
                    if self.tela_stack.is_empty() {
                        self.tela_stack.push(Box::new(inicio::Inicio::default()));
                    }

                },

            }

        }
        
    }

}