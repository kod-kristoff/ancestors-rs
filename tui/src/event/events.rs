use crate::event::Key;
use crossterm::event;
use std::{sync::mpsc, thread, time::Duration};

pub enum Event<I> {
    Input(I),
    Tick,
}
pub struct EventHandler {
    rx: mpsc::Receiver<Event<Key>>,
    _tx: mpsc::Sender<Event<Key>>,
}

pub struct EventHandlerConfig {
    pub exit_key: Key,
    pub tick_rate: Duration,
}

impl Default for EventHandlerConfig {
    fn default() -> Self {
        Self {
            exit_key: Key::Ctrl('c'),
            tick_rate: Duration::from_millis(250),
        }
    }
}
impl EventHandler {
    pub fn new(tick_rate: u64) -> Self {
        Self::with_config(EventHandlerConfig {
            tick_rate: Duration::from_millis(tick_rate),
            ..Default::default()
        })
    }

    pub fn with_config(config: EventHandlerConfig) -> Self {
        let (tx, rx) = mpsc::channel();

        let event_tx = tx.clone();
        thread::spawn(move || loop {
            if event::poll(config.tick_rate).unwrap() {
                if let event::Event::Key(key) = event::read().unwrap() {
                    let key = Key::from(key);

                    event_tx.send(Event::Input(key)).unwrap();
                }
            }

            event_tx.send(Event::Tick).unwrap();
        });

        Self { rx, _tx: tx }
    }

    /// Attempts to read an event.
    /// This function will block the current thread.
    pub fn next(&self) -> Result<Event<Key>, mpsc::RecvError> {
        self.rx.recv()
    }
}
