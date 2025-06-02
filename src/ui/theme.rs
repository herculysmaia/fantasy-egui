// Definindo a aparência gráfica da aplicação

// Importações necessárias para definição de estilo e fontes
use eframe::egui::{style::WidgetVisuals, Color32, CornerRadius, FontData, FontDefinitions, FontFamily, Stroke, Style, Visuals};

// Retorna o estilo definido para a aplicação
pub fn style() -> Style {

    // Cria um estilo padrão para ser customizado
    let mut style = Style::default();

    // customização geral das cores da aplicação
    style.visuals = Visuals {
        dark_mode: true,
        override_text_color: Some(Color32::BLACK),
        extreme_bg_color: Color32::WHITE,
        panel_fill: Color32::from_rgb(255, 116, 0),
        ..Visuals::dark()
    };

    // Cria um estilo padrão para os widgets
    let widgets_style_inative = WidgetVisuals {
        bg_fill: Color32::from_rgb(0, 0, 0),
        weak_bg_fill: Color32::WHITE,
        bg_stroke: Stroke::new(1.0, Color32::from_rgb(210, 210, 210)),
        corner_radius: CornerRadius::same(1),
        fg_stroke: Stroke::new(1.0, Color32::from_rgb(0, 0, 0)),
        expansion: 0.0,
    };

    // Define o estilo dos widgets de acordo com o seu estado 
    style.visuals.widgets.inactive = widgets_style_inative.clone();
    style.visuals.widgets.hovered = WidgetVisuals {
        bg_stroke: Stroke::new(0.0, Color32::from_rgb(255, 116, 0)),
        ..widgets_style_inative.clone()
    };
    style.visuals.widgets.active = WidgetVisuals {
        ..widgets_style_inative.clone()
    };

    // Retorna o estilo customizado
    style

}

// Definição das fontes da aplicação
pub fn load_font() -> FontDefinitions {

    // Instacia padrão para customização de fontes
    let mut fonts = FontDefinitions::default();

    // Define Segoe UI como fonte padrão
    fonts.font_data.insert(
        "Segoe UI".to_owned(),
        std::sync::Arc::new(
            FontData::from_static(include_bytes!("C:/Windows/Fonts/segoeui.ttf"))
        ),
    );

    // Define a fonte Segoe UI como fonte proporcional
    fonts.families.get_mut(&FontFamily::Proportional).unwrap()
        .insert(0, "Segoe UI".into());

    // Define a fonte Segoe UI como fonte monoespaçada
    fonts.families.get_mut(&FontFamily::Monospace).unwrap()
        .push("Segoe UI".into());

    // Retorna as fontes definidas
    fonts
    
}