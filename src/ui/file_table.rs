use ratatui::{
    Frame,
    layout::Constraint,
    prelude::Rect,
    style::Color,
    widgets::{Cell, HighlightSpacing, Row, Table, TableState},
};

use crate::config::FolderConfig;

const COLOR_SELECTED: Color = Color::Green;

pub struct FileTable {
    state: TableState,
    items: Vec<FolderConfig>,
}

impl FileTable {
    pub fn new(state: TableState, items: Vec<FolderConfig>) -> Self {
        Self { state, items }
    }

    pub fn render(&mut self, frame: &mut Frame, area: Rect) {
        let header = ["Path", "Stratergy"]
            .into_iter()
            .map(|h| Cell::from(h))
            .collect::<Row>()
            .height(1);
        let rows = self.items.iter().map(|item| {
            let path = Cell::from(item.path.clone());
            let stratergy = Cell::from(
                item.stratergy
                    .as_ref()
                    .map_or("".to_string(), |s| s.to_string()),
            );
            Row::new(vec![path, stratergy]).height(3)
        });

        let table = Table::new(
            rows,
            [Constraint::Percentage(75), Constraint::Percentage(25)],
        )
        .header(header)
        .cell_highlight_style(COLOR_SELECTED)
        .highlight_spacing(HighlightSpacing::Always);

        frame.render_stateful_widget(table, area, &mut self.state);
    }
}

impl Default for FileTable {
    fn default() -> Self {
        Self {
            state: TableState::default(),
            items: Vec::new(),
        }
    }
}
