use anyhow::Result;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::Frame;
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};

use crate::{
    action::Action,
    tui::{Event, Tui},
};

pub struct App {
    should_quit: bool,

    rx: UnboundedReceiver<Action>,
    tx: UnboundedSender<Action>,
}

impl App {
    pub fn new() -> Self {
        let (tx, rx) = unbounded_channel();
        Self {
            should_quit: false,
            tx,
            rx,
        }
    }

    pub async fn run(&mut self) -> Result<()> {
        let mut tui = Tui::new()?
            .tick_rate(4.0) // 4 ticks per second
            .frame_rate(30.0); // 30 frames per second

        tui.enter()?; // Starts event handler, enters raw mode, enters alternate screen

        loop {
            if let Some(evt) = tui.next().await {
                self.handle_event(evt).map(|action| self.tx.send(action));
                while let Ok(action) = self.rx.try_recv() {
                    self.handle_action(action.clone())
                        .map(|action| self.tx.send(action));

                    if matches!(action, Action::Quit) {
                        tui.draw(|f| {
                            self.ui(f);
                        })?;
                    }
                }
            };

            if self.should_quit {
                break;
            }
        }

        tui.exit()?;

        Ok(())
    }

    fn tick(&mut self) {}

    fn ui(&self, _frame: &mut Frame) {}

    fn handle_event(&self, event: Event) -> Option<Action> {
        match event {
            Event::Tick => Some(Action::Tick),
            Event::Render => Some(Action::Render),
            Event::Key(event) => self.handle_key_event(event),
            _ => None,
        }
    }

    fn handle_key_event(&self, event: KeyEvent) -> Option<Action> {
        match event.code {
            KeyCode::Esc => Some(Action::Quit),
            _ => None,
        }
    }

    fn handle_action(&mut self, action: Action) -> Option<Action> {
        match action {
            Action::Tick => self.tick(),
            Action::Quit => self.should_quit = true,
            Action::Render => {}
        };
        None
    }
}
