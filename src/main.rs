mod cursor;
use cursor::*;
use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{Paragraph, Widget},
};
#[tokio::main] //<3
async fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let app_result = App::default().run(&mut terminal);
    ratatui::restore();
    app_result
}

#[derive(Debug, Default)]
pub struct App {
    exit: bool,
    cursor: Cursor,
    text: String,
}

impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char(c) => self.text.push(c),
            KeyCode::Enter => {
                self.text.push('\n');
                self.cursor.next_line();
            }
            KeyCode::Esc => self.exit(),
            KeyCode::Backspace => {
                let x = self.text.pop();
                if x == Some('\n') {
                    self.cursor.prev_line();
                }
            }
            KeyCode::Up => self.cursor.prev_line(),
            KeyCode::Down => self.cursor.next_line(),
            _ => {}
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let mut lines = vec![Line::from("Rascal".bold())];

        let text_lines: Vec<Line> = self
            .text
            .lines()
            .enumerate()
            .map(|(idx, line)| {
                let line_number = format!("{:>5} ", idx + 1);

                Line::from(vec![
                    Span::styled(line_number, Style::default().fg(Color::DarkGray)),
                    Span::raw(if idx == self.cursor.row {
                        format!(
                            "{}\x1b[31m|\x1b[0m{}",
                            line[..self.cursor.column].to_string(),
                            line[self.cursor.column..].to_string()
                        )
                    } else {
                        line.to_string()
                    }),
                ])
            })
            .collect();

        let height = area.height as usize;
        if self.cursor.row + 3 >= height {
            lines.extend(text_lines[self.cursor.row + 3 - height..].to_vec());
        } else {
            lines.extend(text_lines);
        }

        Paragraph::new(Text::from(lines))
            .left_aligned()
            .render(area, buf);
    }
}
