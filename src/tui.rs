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
    input_buf: String,
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
            input_buf: String::from(""), 
            index: 0,
            error: String::from(""),
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
            _ => {}
        }

        match self.current_screen  {
            CurrentScreen::SelectGame => {
                match key_event.code {
                    KeyCode::Up | KeyCode::Char('w' | 'W') => self.decrement_index(),
                    KeyCode::Down | KeyCode::Char('s' | 'S') => self.increment_index(),
                    KeyCode::Enter | KeyCode::Char(' ') => self.spawn_game(),
                    KeyCode::Char(c) if c.is_ascii_digit() => {
                        // realistically nobody is having more than 1000 games right?
                        if self.input_buf.len() < 3 {
                            self.input_buf.push(c);
                        }
                        self.update_index()
                    }
                    KeyCode::Backspace => {self.input_buf.pop(); self.update_index()},

                    _ => {}
                }
            }
            _ => {}
        }
    }

    fn spawn_game(&mut self) {
        self.current_screen = CurrentScreen::LaunchGame;
        match game_loader::spawn_game(&self.games[self.index]) {
            Ok(_) => {},//self.exit(),
            Err(e) => self.error = e,
        }
    }

    fn update_index(&mut self) {
        if let Ok(number) = self.input_buf.parse::<usize>() {
            if self.games.len() > number {
                self.index = number;
            } else {
                self.index = self.games.len() - 1;
                self.input_buf = (self.games.len() - 1).to_string();
            }
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

        let mut items: Vec<ListItem> = self
            .padded_titles
            .iter()
            .enumerate()
            .map(|(i, name)| {
                let left_space_padding = count_digits(self.games.len()-1);
                let index_str = format!("{:width$}. ", i, width = left_space_padding);

                let is_selected = i == self.index;
                let suffix = if is_selected { " <" } else { "  " };
                let prefix = if is_selected { "> " } else { "  " };
                let line = Line::from(vec![
                    prefix.yellow().bold(),
                    index_str.into(),
                    name.as_str().into(),
                    suffix.yellow().bold(), 
                ])
                .alignment(Alignment::Center);

                ListItem::new(line)
            })
            .collect();

        // this has to be a war crime
        items.push(ListItem::new(
            Line::from(
                format!("{} {:<width$}", if self.input_buf.is_empty() {""} else {">"}, self.input_buf, width = count_digits(self.games.len()-1))
            ).centered()
        ));

        let games_list = List::new(items)
            .block(block)
            .highlight_style(
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            );


        let mut state = ListState::default();
        state.select(Some(self.index));

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

        padded_lines.push(Line::from("\n"));
        if !self.error.is_empty() {
            let mut lines: Vec<Line> = self.error
                .lines()
                .map(|line| Line::from(line.red()))
                .collect();
            
            padded_lines.append(&mut lines);
        } else {
            padded_lines.push(Line::from(format!("Sucessfully launched application: {}", &self.games[self.index].path)).green())
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





// ok guys math is hard
fn count_digits(n: usize) -> usize {
    if n == 0 {
        return 1;
    }
    (n.ilog10() + 1) as usize
}