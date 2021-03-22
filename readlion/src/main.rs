use std::io;

use mortal::{Color, Event, Key, PrepareConfig, Screen, Size, Style};

pub struct State {
    size: Size,
    log: Vec<String>,
    input_buf: String,
    quit: bool,
}

impl State {
    pub fn new(screen: &Screen) -> Self {
        Self {
            log: vec![],
            input_buf: String::new(),
            size: screen.size(),
            quit: false,
        }
    }
    pub fn handle_event(&mut self, ev: Event) {
        match ev {
            Event::Key(k) => match k {
                Key::Backspace => {}
                Key::Enter => self.log.push(self.input_buf.split_off(0)),
                Key::Escape => {}
                Key::Tab => {}
                Key::Up => {}
                Key::Down => {}
                Key::Left => {}
                Key::Right => {}
                Key::Delete => {}
                Key::Insert => {}
                Key::Home => {}
                Key::End => {}
                Key::PageUp => {}
                Key::PageDown => {}
                Key::Char(ch) => {
                    self.input_buf.push(ch);
                }
                Key::Ctrl(x) => match x {
                    'd' => self.quit = true,
                    _ => {}
                },
                Key::F(_) => {}
            },
            Event::Mouse(_) => {}
            Event::Raw(_) => {}
            Event::Resize(_) => {}
            Event::Signal(_) => {}
            Event::NoEvent => {}
        }
    }
    pub fn draw(&self, screen: &Screen) {
        screen.clear_screen();
        for (y, text) in self.log.iter().enumerate() {
            screen.write_at((y, 0), text);
        }
        screen.write_at((self.size.lines - 1, 0), &format!("> {}", self.input_buf));
    }
}

fn main() -> io::Result<()> {
    let screen = Screen::new(PrepareConfig::default())?;
    let mut state = State::new(&screen);
    while !state.quit {
        if let Some(ev) = screen.read_event(None)? {
            state.handle_event(ev);
            state.draw(&screen);
            screen.refresh()?;
        }
    }
    Ok(())
}
