use anyhow::Result;
use crossterm::event::{self, Event, KeyCode};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{self, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph, Wrap},
};

use crate::app::{App, CurrentScreen, CurrentlyEditing};

impl App {
    pub fn run(&mut self) -> Result<bool> {
        ratatui::run(|terminal: &mut DefaultTerminal| {
            loop {
                // Render the user interface.
                terminal.draw(|frame| self.render(frame))?;
                if let Some(t) = self.update_event()? {
                    return Ok(t);
                }
            }
        })
    }

    pub fn render(&mut self, f: &mut Frame) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(1),
                Constraint::Length(3),
            ])
            .split(f.area());

        self.render_title(f, chunks[0]);
        self.render_body(f, chunks[1]);
        self.render_footer(f, chunks[2]);

        // Render popups
        match (&self.current_screen, &self.currently_editing) {
            (_, Some(editing)) => self.render_editing_popup(f, editing),
            (CurrentScreen::Exiting, _) => self.render_exit_popup(f),
            _ => {}
        }
    }

    fn render_editing_popup(&self, f: &mut Frame, editing: &CurrentlyEditing) {
        let area = centered_rect(60, 25, f.area());

        let popup_block = Block::default()
            .title("Enter a new key-value pair")
            .borders(Borders::NONE)
            .style(Style::default().bg(Color::DarkGray));
        f.render_widget(popup_block, area);

        let popup_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .margin(1)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(area);

        let active_style = Style::default().bg(Color::LightYellow).fg(Color::Black);
        let (key_style, value_style) = match editing {
            CurrentlyEditing::Key => (active_style, Style::default()),
            CurrentlyEditing::Value => (Style::default(), active_style),
        };

        let key_block = Block::default().title("Key").borders(Borders::ALL).style(key_style);
        let value_block = Block::default().title("Value").borders(Borders::ALL).style(value_style);

        f.render_widget(Paragraph::new(self.key_input.as_str()).block(key_block), popup_chunks[0]);
        f.render_widget(Paragraph::new(self.value_input.as_str()).block(value_block), popup_chunks[1]);
    }

    fn render_exit_popup(&self, f: &mut Frame) {
        f.render_widget(Clear, f.area());

        let area = centered_rect(60, 25, f.area());
        let popup_block = Block::default()
            .title("Y/N")
            .borders(Borders::NONE)
            .style(Style::default().bg(Color::DarkGray));

        let exit_text = Text::styled(
            "Would you like to output the buffer as json? (y/n)",
            Style::default().fg(Color::Red),
        );

        let exit_paragraph = Paragraph::new(exit_text)
            .block(popup_block)
            .wrap(Wrap { trim: false });

        f.render_widget(exit_paragraph, area);
    }
    fn render_title(&self, f: &mut Frame, area: layout::Rect) {
        let title_block = Block::default()
            .borders(Borders::ALL)
            .style(Style::default());

        let title = Paragraph::new(Text::styled(
            "Create New Json",
            Style::default().fg(Color::Green),
        ))
        .block(title_block);
        f.render_widget(title, area);
    }
    fn render_body(&self, f: &mut Frame, area: layout::Rect) {
        let list_items: Vec<ListItem> = self
            .pairs
            .iter()
            .map(|(key, value)| {
                ListItem::new(Line::from(Span::styled(
                    format!("{: <25} : {}", key, value),
                    Style::default().fg(Color::Yellow),
                )))
            })
            .collect();

        f.render_widget(List::new(list_items), area);
    }
    fn render_footer(&self, f: &mut Frame, area: layout::Rect) {
        let current_navigation_text = vec![
            // The first half of the text
            match self.current_screen {
                CurrentScreen::Main => {
                    Span::styled("Normal Mode", Style::default().fg(Color::Green))
                }
                CurrentScreen::Editing => {
                    Span::styled("Editing Mode", Style::default().fg(Color::Yellow))
                }
                CurrentScreen::Exiting => {
                    Span::styled("Exiting", Style::default().fg(Color::LightRed))
                }
            }
            .to_owned(),
            // A white divider bar to separate the two sections
            Span::styled(" | ", Style::default().fg(Color::White)),
            // The final section of the text, with hints on what the user is editing
            {
                if let Some(editing) = &self.currently_editing {
                    match editing {
                        CurrentlyEditing::Key => {
                            Span::styled("Editing Json Key", Style::default().fg(Color::Green))
                        }
                        CurrentlyEditing::Value => Span::styled(
                            "Editing Json Value",
                            Style::default().fg(Color::LightGreen),
                        ),
                    }
                } else {
                    Span::styled("Not Editing Anything", Style::default().fg(Color::DarkGray))
                }
            },
        ];
        let mode_footer = Paragraph::new(Line::from(current_navigation_text))
            .block(Block::default().borders(Borders::ALL));

        let main_exit_hint = Span::styled(
            "(q) to quit / (e) to make new pair",
            Style::default().fg(Color::Red),
        );

        let current_keys_hint = match self.current_screen {
            CurrentScreen::Main | CurrentScreen::Exiting => main_exit_hint,
            CurrentScreen::Editing => Span::styled(
                "(ESC) to cancel/(Tab) to switch boxes/enter to complete",
                Style::default().fg(Color::Red),
            ),
        };

        let key_notes_footer = Paragraph::new(Line::from(current_keys_hint))
            .block(Block::default().borders(Borders::ALL));
        let footer_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(area);
        f.render_widget(mode_footer, footer_chunks[0]);
        f.render_widget(key_notes_footer, footer_chunks[1]);
    }

    pub fn update_event(&mut self) -> Result<Option<bool>> {
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Release {
                // Skip events that are not KeyEventKind::Press
                return Ok(None);
            }
            match self.current_screen {
                CurrentScreen::Main => match key.code {
                    KeyCode::Char('e') => {
                        self.current_screen = CurrentScreen::Editing;
                        self.currently_editing = Some(CurrentlyEditing::Key);
                    }
                    KeyCode::Char('q') => {
                        self.current_screen = CurrentScreen::Exiting;
                    }
                    _ => {}
                },
                CurrentScreen::Exiting => match key.code {
                    KeyCode::Char('y') => {
                        return Ok(Some(true));
                    }
                    KeyCode::Char('n') | KeyCode::Char('q') => {
                        return Ok(Some(false));
                    }
                    _ => {}
                },
                CurrentScreen::Editing => match key.code {
                    KeyCode::Enter => {
                        if let Some(editing) = &self.currently_editing {
                            match editing {
                                CurrentlyEditing::Key => {
                                    self.currently_editing = Some(CurrentlyEditing::Value);
                                }
                                CurrentlyEditing::Value => {
                                    self.save_key_value();
                                    self.current_screen = CurrentScreen::Main;
                                }
                            }
                        }
                    }
                    KeyCode::Backspace => {
                        if let Some(editing) = &self.currently_editing {
                            match editing {
                                CurrentlyEditing::Key => {
                                    self.key_input.pop();
                                }
                                CurrentlyEditing::Value => {
                                    self.value_input.pop();
                                }
                            }
                        }
                    }
                    KeyCode::Esc => {
                        self.current_screen = CurrentScreen::Main;
                        self.currently_editing = None;
                    }
                    KeyCode::Tab => {
                        self.toggle_editing();
                    }
                    KeyCode::Char(value) => {
                        if let Some(editing) = &self.currently_editing {
                            match editing {
                                CurrentlyEditing::Key => {
                                    self.key_input.push(value);
                                }
                                CurrentlyEditing::Value => {
                                    self.value_input.push(value);
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
        Ok(None)
    }
}

/// helper function to create a centered rect using up certain percentage of the available rect `r`
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    // Cut the given rectangle into three vertical pieces
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    // Then cut the middle vertical piece into three width-wise pieces
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1] // Return the middle chunk
}
