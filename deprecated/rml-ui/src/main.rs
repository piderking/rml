use std::io::{self, Error};
mod view;
mod error;
mod binds;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    buffer::Buffer,
    layout::{self, Constraint, Layout, Rect},
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
    DefaultTerminal, Frame,
};

use crate::{binds::{Bind, Keys}, view::View};

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let app_result = App::default().run(&mut terminal);
    ratatui::restore();
    app_result
}

#[derive(Default)]
pub struct App {
    input: String,
    view: View,
    exit: bool,

    // Store for Key Binds
    keys: Vec<Keys>,


    // Empty Error
    err: Option<Error>,


    commands: Vec<String>

}
impl App {
    /// runs the application's main loop until the user quits
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

    /// updates the application's state based on user input
    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            // it's important to check that the event is a key press event as
            // crossterm also emits key release and repeat events on Windows.
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {

        // Per Each Mode
        match (key_event.code, self.input.len()) {
            (KeyCode::Char(n), _) => {
                self.input.push(n);
            },
            (KeyCode::Enter, 1..) => {
                match self.input.clone()[..1].as_ref() {
                    "q" => self.exit(),
                    "v" => {
                        if self.input.len() > 2 {
                            self.handle_view(View::from_str(self.input.clone()[2..].to_string()));
                        }
                    }
                    _ => {}
                }
                self.commands.push(self.input.clone());
                
                self.input = String::new();
            },
            (KeyCode::Backspace,1..) => {
                self.input.pop();
            }

            
            _ => {}
        }
    }

    fn handle_view(&mut self, view: Option<View>)  {
        match view {
            Option::Some(view) => {self.view=view;}
            Option::None => {}
        }
    }

    fn handle_keybinds(&mut self, c: char) {
        // Bindings
        let bindings = vec![
           //Bind.defe
           Bind::new('z').activate(Keys::CTRL)
        ];

       

        
    }
    fn exit(&mut self) {
        self.exit = true;
    }

   
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::bordered()
        
                    .border_set(border::PLAIN);
        let layout = Layout::default().direction(layout::Direction::Vertical)
                .constraints(vec![
                    Constraint::Percentage(100),
                    Constraint::Length(3)
                ]).split(area);

                Paragraph::new(format!(">{}", self.input)).block(block).render(layout[1], buf);

        let view = layout[0];
        match &self.view {
            View::Home => {
                Paragraph::new("Welcome Home!").block(Block::default()).render(view, buf);
            }
        }
        
       
    }
}
