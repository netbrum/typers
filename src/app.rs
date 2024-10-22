mod words;

use crate::Args;
use crossterm::{
    cursor::SetCursorStyle,
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    execute,
};
use ratatui::{
    layout::{Constraint, Flex, Layout, Position, Rect},
    style::{Style, Stylize as RatatuiStylize},
    text::{Line, Span},
    widgets::{Block, BorderType, Padding, Paragraph, Widget, Wrap},
    DefaultTerminal, Frame,
};
use std::{io, time::Instant};
use words::Words;

#[derive(PartialEq, Eq)]
enum State {
    Playing,
    Finished,
    Exit,
}

pub struct App {
    start: Option<Instant>,
    state: State,
    first_draw: bool,
    args: Args,
    typed: Vec<char>,
    words: Vec<&'static str>,
}

impl App {
    pub fn new(args: Args) -> Self {
        let words = Words::generate(args.words);
        let typed = Vec::with_capacity(words.len());

        Self {
            start: None,
            state: State::Playing,
            first_draw: true,
            args,
            typed,
            words,
        }
    }

    fn exit(&mut self) {
        self.state = State::Exit;
    }

    fn reset(&mut self) {
        self.words = Words::generate(self.args.words);
        self.typed = Vec::with_capacity(self.words().len());
        self.first_draw = true;
    }

    fn is_finished(&self) -> bool {
        self.typed.len() >= self.words().len()
    }

    fn words(&self) -> String {
        self.words.join(" ")
    }

    #[expect(clippy::cast_precision_loss)]
    fn wpm(&self) -> f64 {
        let elapsed = self.start.unwrap().elapsed();
        (self.words().len() / 5) as f64 / elapsed.as_secs_f64() * 60.0
    }

    #[expect(clippy::cast_precision_loss)]
    fn accuracy(&self) -> f64 {
        let words = self.words();

        let correct: Vec<_> = self
            .typed
            .iter()
            .zip(words.chars())
            .filter(|(c, target)| *c == target)
            .collect();

        (correct.len() as f64 / words.len() as f64) * 100.0
    }

    fn time_ms(&self) -> u128 {
        let elapsed = self.start.unwrap().elapsed();
        elapsed.as_millis()
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        execute!(io::stdout(), SetCursorStyle::BlinkingBar)?;

        while self.state != State::Exit {
            terminal.draw(|frame| {
                self.draw(frame);
                self.first_draw = false;
            })?;

            if self.state == State::Playing {
                terminal.show_cursor()?;
            }

            self.handle_events()?;
        }

        Ok(())
    }

    fn finish_screen(&self, frame: &mut Frame) {
        let area = center(frame.area(), Constraint::Length(40), Constraint::Length(11));

        let title = format!("{} words", self.words.len()).yellow();

        let block = Block::bordered()
            .title(title)
            .border_style(Style::default().yellow())
            .border_type(BorderType::Rounded)
            .padding(Padding::uniform(2));

        let inner = block.inner(area);

        block.render(area, frame.buffer_mut());

        let layout = Layout::vertical([Constraint::Length(1); 3])
            .flex(Flex::SpaceBetween)
            .areas::<3>(inner);

        let time = format!("Time: {}ms", self.time_ms());
        Paragraph::new(time).render(layout[0], frame.buffer_mut());

        let wpm = format!("WPM: {}", self.wpm());
        Paragraph::new(wpm).render(layout[1], frame.buffer_mut());

        let accuracy = format!("Accuracy: {}%", self.accuracy());
        Paragraph::new(accuracy).render(layout[2], frame.buffer_mut());
    }

    #[expect(clippy::cast_possible_truncation)]
    fn playing_screen(&self, frame: &mut Frame) {
        let words = self.words();

        let area = center(
            frame.area(),
            Constraint::Length(words.len() as u16),
            Constraint::Percentage(100),
        );

        let block = Block::new().padding(Padding::top(area.height / 2));

        let mut typed: Vec<Span> = self
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

        typed.push(Span::raw(words[typed.len()..].to_string()));

        let typed: Vec<Span> = typed.into_iter().collect();

        Paragraph::new(Line::from(typed))
            .block(block)
            .wrap(Wrap { trim: true })
            .render(area, frame.buffer_mut());

        if self.first_draw {
            frame.set_cursor_position(Position::new(area.x, area.height / 2));
        }
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
                    if self.start.is_none() {
                        self.start = Some(Instant::now());
                    }

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
