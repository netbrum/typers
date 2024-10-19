mod words;

use crate::Args;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    layout::{Constraint, Flex, Layout, Rect},
    style::{Style, Stylize as RatatuiStylize},
    text::{Line, Span},
    widgets::{Block, BorderType, Padding, Paragraph, Widget, Wrap},
    DefaultTerminal, Frame,
};
use std::io;
use words::Words;

#[derive(PartialEq, Eq)]
enum State {
    Playing,
    Finished,
    Exit,
}

pub struct App {
    args: Args,
    state: State,
    typed: Vec<char>,
    words: Vec<&'static str>,
}

impl App {
    pub fn new(args: Args) -> Self {
        let words = Words::generate(args.words);
        let typed = Vec::with_capacity(words.len());

        Self {
            args,
            state: State::Playing,
            typed,
            words,
        }
    }

    fn exit(&mut self) {
        self.state = State::Exit;
    }

    fn reset(&mut self) {
        self.words = Words::generate(self.args.words);
        self.typed = Vec::default();
    }

    fn is_finished(&self) -> bool {
        self.typed.len() >= self.words().len()
    }

    fn words(&self) -> String {
        self.words.join(" ")
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while self.state != State::Exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }

        Ok(())
    }

    fn finish_screen(&self, frame: &mut Frame) {
        let area = center(
            frame.area(),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
        );

        let title = format!("{} words", self.words.len()).yellow();
        let block = Block::bordered()
            .title(title)
            .border_style(Style::default().yellow())
            .border_type(BorderType::Rounded)
            .padding(Padding::uniform(2));

        Paragraph::new("You did it!")
            .block(block)
            .wrap(Wrap { trim: true })
            .render(area, frame.buffer_mut());
    }

    fn playing_screen(&self, frame: &mut Frame) {
        let words = self.words();

        let area = center(
            frame.area(),
            Constraint::Length(words.len().try_into().unwrap()),
            Constraint::Percentage(100),
        );

        let block = Block::new().padding(Padding::top(area.height / 2));

        let typed: Vec<Span> = self
            .typed
            .iter()
            .zip(words.chars())
            .map(|(c, target)| {
                if target == *c {
                    Span::raw(target.to_string()).white()
                } else {
                    Span::raw(target.to_string()).red()
                }
            })
            .collect();

        Paragraph::new(words.gray())
            .block(block.clone())
            .wrap(Wrap { trim: true })
            .render(area, frame.buffer_mut());

        Paragraph::new(Line::from(typed))
            .block(block)
            .wrap(Wrap { trim: true })
            .render(area, frame.buffer_mut());
    }

    fn draw(&self, frame: &mut Frame) {
        match self.state {
            State::Playing => self.playing_screen(frame),
            State::Finished => self.finish_screen(frame),
            State::Exit => unreachable!(),
        }
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match self.state {
            State::Playing => match key_event.code {
                KeyCode::Esc => self.exit(),
                KeyCode::Tab => self.reset(),
                KeyCode::Char(c) => {
                    self.typed.push(c);

                    if self.is_finished() {
                        self.state = State::Finished;
                    }
                }
                _ => {}
            },
            State::Finished => self.exit(),
            State::Exit => unreachable!(),
        }
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event);
            }
            _ => {}
        };

        Ok(())
    }
}

fn center(area: Rect, horizontal: Constraint, vertical: Constraint) -> Rect {
    let [area] = Layout::horizontal([horizontal])
        .flex(Flex::Center)
        .areas(area);

    let [area] = Layout::vertical([vertical]).flex(Flex::Center).areas(area);

    area
}
