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

use crate::game_loader;
use crate::game_loader::GameInfo;
#[derive(Debug, Default)]
pub enum CurrentScreen {
    #[default]
    SelectGame,
    LaunchGame,
    LoadError,
}

#[derive(Debug, Default)]
pub struct App {
    games: Vec<GameInfo>,
    padded_titles: Vec<String>,
    index: usize,
    current_screen: CurrentScreen,
    error: String,
    exit: bool,
}

impl App {
    pub fn from(games: Vec<GameInfo>) -> Self {
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
            current_screen: CurrentScreen::SelectGame,
            index: 0,
            error: String::from("Error"),
            exit: false,
        }
    }


    pub fn error(mut self, error_msg: String) -> Self {
        
        self.error = error_msg;
        if !self.error.is_empty() {
            self.current_screen = CurrentScreen::LoadError;
        }
        self
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
        self.current_screen = CurrentScreen::LaunchGame;
        match game_loader::spawn_game(&self.games[self.index]) {
            Ok(_) => self.exit(),
            Err(e) => self.error = e,
        }
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

    fn render_select_game(&self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" Steam Remote Play ".bold());
        let instructions = Line::from(vec![
            " Decrement ".into(),
            "<Down/S>".blue().bold(),
            " Increment ".into(),
            "<Up/W>".blue().bold(),
            " Quit ".into(),
            "<Q/Esc>".blue().bold(),
            " Launch Game ".into(),
            "<Enter/Space> ".blue().bold(),
        ]);
        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.centered())
            .border_set(border::THICK);

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

    fn render_launch_game(&self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" Game Launch Status ".bold());
        let instructions = Line::from(vec![
            " Quit ".into(),
            "<Q/Esc> ".blue().bold(),
        ]);
        let block = Block::bordered().border_set(border::THICK)
            .title(title.centered())
            .title_bottom(instructions.centered())
            .border_set(border::THICK);
        

        let debug_string = format!("{:#?}", &self.games[self.index]);
        let lines: Vec<&str> = debug_string.lines().collect();
        let total_lines = lines.len();
        let max_width = lines.iter().map(|line| line.len()).max().unwrap_or(0);

        let mut padded_lines: Vec<Line> = lines
            .into_iter()
            .enumerate()
            .map(|(i, line)| {

                let left_pad = if i == 0 || i == total_lines-1 { "" } else { "    " };

                let right_pad_len = max_width.saturating_sub(line.len());
                let right_pad = " ".repeat(right_pad_len);

                let formatted_line = format!("{}{}{}", left_pad, line, right_pad);
                Line::from(formatted_line)
            })
            .collect();
        
        if !self.error.is_empty() {
            padded_lines.push(Line::from("\n"));
            let mut lines: Vec<Line> = self.error
                .lines()
                .map(|line| Line::from(line.red()))
                .collect();
            
            padded_lines.append(&mut lines);
        }

        let disp_text = Text::from(padded_lines);
        Paragraph::new(disp_text)
            .centered()
            .block(block)
            .render(area, buf);
    }

    fn render_load_error(&self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" Error when loading games ".bold());
        let instructions = Line::from(vec![
            " Quit ".into(),
            "<Q/Esc> ".blue().bold(),
        ]);
        let block = Block::bordered().border_set(border::THICK)
            .title(title.centered())
            .title_bottom(instructions.centered())
            .border_set(border::THICK);

        let mut lines: Vec<Line> = self.error
            .lines()
            .map(|line| Line::from(line.red()))
            .collect();
        
        lines.insert(0, Line::from("\n"));
        Paragraph::new(lines)
            .centered()
            .block(block)
            .render(area, buf);
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        match self.current_screen {
            CurrentScreen::SelectGame => self.render_select_game(area, buf),
            CurrentScreen::LaunchGame => self.render_launch_game(area, buf),
            CurrentScreen::LoadError => self.render_load_error(area, buf),
        }
    }
}

pub fn run_tui() -> Result<(), std::io::Error> {
    
    match game_loader::parse_games() {
        Ok(games) => {
            ratatui::run(|terminal| App::from(games).run(terminal))
        }
        Err(e) => {
            ratatui::run(|terminal| App::default().error(e).run(terminal))
        },
    }
}
