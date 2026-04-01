use std::time::Duration;

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::{DefaultTerminal, Frame};

use crate::event::{Event, EventHandler};

/// Application.
#[derive(Debug)]
pub struct App {
    /// the application exit or not?
    pub is_quit: bool,
    /// counter
    pub counter: u8,
    /// event handler for terminal events
    events: EventHandler,
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self {
            is_quit: false,
            counter: 0,
            events: EventHandler::new(Duration::from_millis(250)),
        }
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Set should_quit to true to quit the application.
    pub fn quit(&mut self) {
        self.is_quit = true;
    }
    pub fn increment_counter(&mut self) {
        self.counter = self.counter.checked_add(1).unwrap_or(u8::MIN);
    }
    pub fn decrement_counter(&mut self) {
        self.counter = self.counter.checked_sub(1).unwrap_or(u8::MAX);
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

impl App {
    pub fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area())
    }
    pub fn update(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Esc | KeyCode::Char('q') => self.quit(),
            KeyCode::Char('c') | KeyCode::Char('C')
                if key_event.modifiers == KeyModifiers::CONTROL =>
            {
                self.quit()
            }
            KeyCode::Up => self.increment_counter(),
            KeyCode::Down => self.decrement_counter(),
            _ => {
                // eprintln!("unsupported key event: {:?}", key_event);
            }
        };
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> anyhow::Result<()> {
        // Start the main loop.
        while !self.is_quit {
            // Render the user interface.
            terminal.draw(|frame| self.draw(frame))?;

            // Handle events.
            match self.events.next()? {
                Event::Tick => {}
                Event::Key(key_event) => self.update(key_event),
                Event::Mouse(_) => {}
                Event::Resize(_, _) => {}
            };
        }
        Ok(())
    }
}
