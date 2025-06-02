// Tratamendo de telas da 

// Telas acessiveis no inicio da aplicação
mod inicio;
mod busca;
mod theme;

// Deixando egui acessível
use eframe::egui;

// Definindo as telas da aplicação
pub enum Tela {
    Inicio,
    BuscaTime,
}

// Estrutura de dados da aplicação
pub struct AppData {
    tela: Tela,
}


// Implementando métodos para a estrutura de dados da aplicação
impl AppData {

    // Método para inicializar a aplicação
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {

        // Configurando fontes e estilo do egui
        cc.egui_ctx.set_fonts(theme::load_font());
        cc.egui_ctx.set_style(theme::style());

        // Retornando a instância da aplicação com a tela inicial
        Self { tela: Tela::Inicio }
    
    }
}

// Implementando a trait App para a estrutura de dados da aplicação
impl eframe::App for AppData {

    // Método chamado a cada atualização da aplicação
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {

        // Definindo qual tela deve ser exibida
        match self.tela {
            Tela::Inicio => { self.show_inicio(ctx, frame) }
            Tela::BuscaTime => { self.show_busca_time(ctx, frame) }
        }
        
    }
}