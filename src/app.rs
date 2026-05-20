pub mod screens;

use std::sync::Arc;

use ratatui::{
  DefaultTerminal,
  crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
  prelude::*,
};
use serenity::{
  Client,
  all::{GatewayIntents, Http},
};
use tokio::{io, task::JoinHandle};

use crate::{app::screens::loading, handler::Handler};

pub enum ScreenType {
  Login,
  Main,
  Loading,
  Logs,
}

pub struct App {
  screen: ScreenType,
  http: Arc<Http>,
  exit: bool,
}

impl App {
  pub fn new(http: Arc<Http>) -> App {
    App {
      screen: ScreenType::Loading,
      http,
      exit: false,
    }
  }

  pub fn screen(&self) -> &ScreenType {
    &self.screen
  }

  fn render(&mut self, frame: &mut Frame) {
    match self.screen {
      ScreenType::Loading => loading::render(self, frame),
      ScreenType::Main | ScreenType::Login => {
        self.exit = true;
      }
      _ => todo!(),
    }
  }

  fn handle_events(&mut self) -> io::Result<()> {
    match event::read()? {
      Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
        self.handle_key_event(key_event);
      }
      _ => {}
    }

    Ok(())
  }

  fn handle_key_event(&mut self, key_event: KeyEvent) {
    if key_event.code == KeyCode::Char('c') && key_event.modifiers.contains(KeyModifiers::CONTROL) {
      self.exit = true;
    }
  }

  pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
    while !self.exit {
      terminal.draw(|frame| self.render(frame))?;
      self.handle_events()?;
    }
    Ok(())
  }
}
