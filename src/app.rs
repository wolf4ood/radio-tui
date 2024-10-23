use color_eyre::Result;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::Frame;

use crate::{
    action::Action,
    tui::{Event, Tui},
};

pub struct App {
    should_quit: bool,
}

impl App {
    pub fn new() -> Self {
        Self { should_quit: false }
    }

    pub async fn run(&mut self) -> Result<()> {
        let mut tui = Tui::new()?
            .tick_rate(4.0) // 4 ticks per second
            .frame_rate(30.0); // 30 frames per second

        tui.enter()?; // Starts event handler, enters raw mode, enters alternate screen

        loop {
            tui.draw(|f| {
                self.ui(f);
            })?;

            if let Some(evt) = tui.next().await {
                // `tui.next().await` blocks till next event
                let mut maybe_action = self.handle_event(evt);
                while let Some(action) = maybe_action {
                    maybe_action = self.update(action);
                }
            };

            if self.should_quit {
                break;
            }
        }

        tui.exit()?;

        Ok(())
    }


    fn tick(&mut self)  {

    }

    fn ui(&self, _frame: &mut Frame) {}

    fn handle_event(&self, event: Event) -> Option<Action> {
        match event {
            Event::Tick => Some(Action::Tick),
            Event::Key(KeyEvent {
                code: KeyCode::Esc, ..
            }) => Some(Action::Quit),
            _ => None,
        }
    }

    fn update(&mut self, action: Action) -> Option<Action> {
        match action {
            Action::Tick => self.tick(),
            Action::Quit => self.should_quit = true,
        };
        None
    }
}
