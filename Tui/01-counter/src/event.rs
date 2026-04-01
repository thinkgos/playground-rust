use std::{
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};

use anyhow::Result;
use crossterm::event::{self, Event as OriginEvent, KeyEvent, MouseEvent};

/// Terminal events.
#[derive(Clone, Copy, Debug)]
pub enum Event {
    /// Terminal tick.
    Tick,
    /// Key press.
    Key(KeyEvent),
    /// Mouse click/scroll.
    Mouse(MouseEvent),
    /// Terminal resize.
    Resize(u16, u16),
}

/// Terminal event handler.
#[derive(Debug)]
pub struct EventHandler {
    /// Event sender channel.
    #[allow(dead_code)]
    tx: mpsc::Sender<Event>,
    /// Event receiver channel.
    rx: mpsc::Receiver<Event>,
    /// Event handler thread.
    #[allow(dead_code)]
    handler: thread::JoinHandle<()>,
}

impl EventHandler {
    /// Constructs a new instance of [`EventHandler`].
    pub fn new(tick_rate: Duration) -> Self {
        let (tx, rx) = mpsc::channel();
        let tx1 = tx.clone();
        let handler = thread::spawn(move || {
            let mut last_tick = Instant::now();
            loop {
                let timeout = tick_rate
                    .checked_sub(last_tick.elapsed())
                    .unwrap_or(tick_rate);

                if event::poll(timeout).expect("unable to poll for event") {
                    match event::read().expect("unable to read event") {
                        OriginEvent::Key(e) => {
                            if e.kind == event::KeyEventKind::Press {
                                tx1.send(Event::Key(e))
                            } else {
                                Ok(()) // ignore KeyEventKind::Release on windows
                            }
                        }
                        OriginEvent::Mouse(e) => tx1.send(Event::Mouse(e)),
                        OriginEvent::Resize(w, h) => tx1.send(Event::Resize(w, h)),
                        _ => unimplemented!(),
                    }
                    .expect("failed to send terminal event")
                }

                if last_tick.elapsed() >= tick_rate {
                    tx1.send(Event::Tick).expect("failed to send tick event");
                    last_tick = Instant::now();
                }
            }
        });

        Self { tx, rx, handler }
    }

    /// Receive the next event from the handler thread.
    ///
    /// This function will always block the current thread if
    /// there is no data available and it's possible for more data to be sent.
    pub fn next(&self) -> Result<Event> {
        Ok(self.rx.recv()?)
    }
}
