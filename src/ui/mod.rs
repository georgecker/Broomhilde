// Set target folder
// Set Stratergy

pub mod file_table;

use ratatui::{
    prelude::{Buffer, Rect},
    style::Stylize,
    text::{Line, Span, Text},
    widgets::{Block, BorderType, Paragraph, Widget},
};

use crate::config::BroomhildeConfig;

pub struct ConfigUi<'a> {
    config: &'a BroomhildeConfig,
}

impl<'a> ConfigUi<'a> {
    pub fn new(config: &'a BroomhildeConfig) -> Self {
        Self { config }
    }
}

impl<'a> Widget for ConfigUi<'a> {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let title = Line::from("BROOMHILDE".bold());
        let instructions = Line::from(vec![
            Span::from("+Add").bold().green(),
            Span::from("~Change").bold().yellow(),
            Span::from("-Delete").bold().light_red(),
            Span::from("!Exit").bold().red(),
        ]);

        let block = Block::bordered()
            .title(title.left_aligned())
            .title_bottom(instructions.centered())
            .border_type(BorderType::Rounded);

        let text = Text::from("DEMO TEXT").centered();

        Paragraph::new(text)
            .left_aligned()
            .block(block)
            .render(area, buf);
    }
}
