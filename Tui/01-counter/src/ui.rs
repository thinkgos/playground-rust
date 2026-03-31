use ratatui::{
    buffer::Buffer,
    layout::Rect,
    prelude::Alignment,
    style::Stylize,
    text::{Line, Text},
    widgets::{Block, BorderType, Borders, Paragraph, Widget},
};

use crate::app::App;

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let title = Line::from(" Counter app ".bold());
        let instructions = Line::from(vec![
            " Decrement ".into(),
            "<Up>".blue().bold(),
            " Increment ".into(),
            "<Down>".blue().bold(),
            " Quit ".into(),
            "<Q> ".blue().bold(),
        ]);
        let block = Block::default()
            .title(title)
            .title_alignment(Alignment::Center)
            .title_bottom(instructions)
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded);

        let value = Text::from(vec![Line::from(vec![
            "Value: ".into(),
            self.counter.to_string().yellow(),
        ])]);

        Paragraph::new(value)
            .block(block)
            .alignment(Alignment::Center)
            .render(area, buf);
    }
}
