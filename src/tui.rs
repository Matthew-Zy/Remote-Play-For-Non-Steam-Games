use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::{Color, Modifier, Style, Stylize},
    symbols::border,
    text::{Line, Text},
    widgets::{Block, List, ListItem, ListState, Paragraph, StatefulWidget, Widget},
};

use std::io;

use crate::game_loader::{GameInfo};
use crate::game_loader;

#[derive(Debug, Default)]
pub struct App {
    games: Vec<GameInfo>,
    padded_titles: Vec<String>,
    index: usize,
    exit: bool,
}

impl App {
    pub fn create(games: Vec<GameInfo>) -> Self {
        let titles: Vec<String> = games
            .iter()
            .map(|x| {
                if x.name.is_empty() {
                    x.path.clone()
                } else {
                    x.name.clone()
                }
            })
            .collect();

        let max_len = titles.iter().map(|s| s.len()).max().unwrap_or(0);

        let padded_titles: Vec<String> = titles
            .iter()
            .map(|x| format!("{:^width$}", x, width = max_len))
            .collect();

        App {
            games: games,
            padded_titles: padded_titles,
            index: 0,
            exit: false,
        }
    }

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
        match key_event.code {
            // don't worry capslocks users I gotchu
            KeyCode::Esc | KeyCode::Char('q' | 'Q') => self.exit(),
            KeyCode::Up | KeyCode::Char('w' | 'W') => self.decrement_index(),
            KeyCode::Down | KeyCode::Char('s' | 'S') => self.increment_index(),
            KeyCode::Enter | KeyCode::Char(' ') => self.spawn_game(),
            _ => {}
        }
    }

    fn spawn_game(&mut self) {
        let _ = game_loader::spawn_game(&self.games[self.index]);
        self.exit();
        // error handling eventually
    }

    fn exit(&mut self) {
        self.exit = true;
    }

    fn increment_index(&mut self) {
        self.index += 1;
        self.index = self.index % self.games.len();
    }

    fn decrement_index(&mut self) {
        if self.index == 0 {
            self.index = self.games.len() - 1;
        } else {
            self.index -= 1;
        }
    }


}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" Steam Remote Play ".bold());
        let instructions = Line::from(vec![
            " Decrement ".into(),
            "<Down/S>".blue().bold(),
            " Increment ".into(),
            "<Up/W>".blue().bold(),
            " Quit ".into(),
            "<Q>".blue().bold(),
            " Launch Game ".into(),
            "<Enter/Space> ".blue().bold()
        ]);
        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.centered())
            .border_set(border::THICK);

        let counter_text = Text::from(vec![Line::from(vec![
            "Value: ".into(),
            self.index.to_string().yellow(),
        ])]);

        let items: Vec<ListItem> = self
            .padded_titles
            .iter()
            .enumerate()
            .map(|(i, x)| {
                let name = x;
                let index_str = format!("{i}. ");

                let is_selected = i == self.index;
                let suffix = if is_selected { " <" } else { "  " };
                let prefix = if is_selected { "> " } else { "  " };
                // 2. Build line with optional right-hand symbol
                let line = Line::from(vec![
                    prefix.yellow().bold(),
                    index_str.into(),
                    name.as_str().into(),
                    suffix.yellow().bold(), // End pointer symbol
                ])
                .alignment(Alignment::Center);

                ListItem::new(line)
            })
            .collect();

        let games_list = List::new(items)
            .block(block)
            // Highlight current index
            .highlight_style(
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            );

        let mut state = ListState::default();
        state.select(Some(self.index));

        // 4. Render using render_stateful_widget directly to Buffer
        StatefulWidget::render(games_list, area, buf, &mut state);
    }
}

pub fn run_tui(games: Vec<GameInfo>) -> Result<(), std::io::Error> {
    ratatui::run(|terminal| App::create(games).run(terminal))
}
