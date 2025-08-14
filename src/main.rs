use std::io;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
    DefaultTerminal, Frame,
};

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let result = App::default().run(&mut terminal);
    ratatui::restore();
    result
}

#[derive(Debug, Default)]
pub struct App {
    counter: u8,
    message: String,
    exit: bool
}

impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()>   {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
            if self.counter == 0 {
                self.set_message(String::from("Minimum"));
            } else if self.counter == 255 {
                self.set_message(String::from("Maximum"));
            } else {
                self.set_message(String::from(""));
            }
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame)   {
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
            KeyCode::Char('q') => self.exit(),
            KeyCode::Right => {
                if self.counter < 255 {
                    self.increment();
                }
            },
            KeyCode::Left => {
                if self.counter > 0 {
                    self.decrement()
                }
            },
            _ => {}
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }

    fn increment(&mut self) {
        self.counter += 1;
    }

    fn decrement(&mut self) {
        self.counter -= 1;
    }

    fn set_message(&mut self, string: String)    {
        self.message = string;
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer)   {
        let title = Line::from(" Counter App ".bold());
        let instructions = Line::from(vec![
            " Decrement ".into(),
            "<Left>".blue().bold(),
            " Increment ".into(),
            "<Right>".blue().bold(),
            " Quit ".into(),
            "<Q>".blue().bold(),
        ]);
        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.centered())
            .border_set(border::THICK);
        
        let counter_text = Text::from(vec![
            Line::from(vec![
                "Value: ".into(),
                self.counter.to_string().yellow(),
            ]),
            Line::from(vec![
                self.message.as_str().into(),
            ]),
        ]);

        Paragraph::new(counter_text)
            .centered()
            .block(block)
            .render(area, buf);
    }
}