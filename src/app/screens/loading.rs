use ratatui::prelude::*;

use crate::app::App;

const TEXT: &str = "Loading";
const TEXT_SIZE: u16 = TEXT.len() as u16;

pub fn render(app: &mut App, frame: &mut Frame) {
  let center = frame
    .area()
    .centered(Constraint::Length(TEXT_SIZE), Constraint::Length(1));

  frame.render_widget(TEXT.yellow(), center);
}
