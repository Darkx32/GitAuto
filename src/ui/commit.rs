use std::time::Duration;

use color_eyre::eyre::Context;
use ratatui::{DefaultTerminal, Frame, crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers}, layout::{Constraint, Layout, Position}, style::{Color, Style, Stylize}, text::{Line, Text}, widgets::{Block, Paragraph, Tabs}};

use crate::core::{git::git_controller, input::InputHandler, tab::TabHandler};

struct CommitHandler {
    msg: InputHandler,
    tabs: TabHandler,
    is_running: bool,
    is_finishing: bool,
    focus: Focus
}

enum Focus {
    MsgInput,
    CommitType
}

pub fn commit_app() -> color_eyre::Result<()> {
    let mut commit_handler = CommitHandler::new();

    ratatui::run(|terminal| commit_handler.run(terminal)).context("Error to run commit app")
}

impl CommitHandler {
    fn new() -> Self {
        Self {
            msg: InputHandler::new(),
            tabs: TabHandler::new::<&str>(["feat", "docs", "fix", "improvement"].to_vec()),
            is_running: true,
            is_finishing: false,
            focus: Focus::MsgInput
        }
    }

    fn run(&mut self, terminal: &mut DefaultTerminal) -> color_eyre::Result<()> {
        loop {
            if !self.is_running{
                break;
            }

            terminal.draw(|frame| { 
                if self.is_finishing {
                    self.finish(frame)
                        .expect("Error on commit");
                } else {
                    self.render(frame);
                }
            })?;

            self.handle_events()?;
        }
        Ok(())
    }

    fn render(&mut self, frame: &mut Frame) {
        let input_color = if matches!(self.focus, Focus::MsgInput) { Color::Yellow } else { Color::White };
        let tab_color = if matches!(self.focus, Focus::CommitType) { Color::Yellow } else { Color::White };

        let layout =  Layout::vertical([
            Constraint::Length(1),
            Constraint::Percentage(50),
            Constraint::Percentage(50)
        ]);

        let [help_area, input_area, tab_area] = frame.area().layout(&layout);


        let text_help = Text::from(
            Line::from(vec![
                "Press ".into(),
                "Esc ".bold(),
                "to cancel, ".into(),
                "Tab ".bold(),
                "to change widget and ".into(),
                "Ctrl + X ".bold(),
                "to send commit".into()
            ])
        ).patch_style(Style::default().fg(Color::Cyan));
        let help_message = Paragraph::new(text_help);
        frame.render_widget(help_message, help_area);

        let input_msg = Paragraph::new(self.msg.text.as_str())
            .block(Block::bordered().title("Message"))
            .style(Style::default().fg(input_color));

        frame.render_widget(input_msg, input_area);

        let tab = Tabs::new(self.tabs.titles.clone())
            .block(Block::default().title("Type"))
            .style(Style::default().fg(tab_color))
            .select(self.tabs.tab_index);

        frame.render_widget(tab, tab_area);

        if matches!(self.focus, Focus::MsgInput) {
            frame.set_cursor_position(
                Position::new(
                    input_area.x + self.msg.text.len() as u16 + 1,
                    input_area.y + 1
                )
            );
        }
    }

    fn handle_events(&mut self) -> color_eyre::Result<()> {
        if event::poll(Duration::from_millis(250)).context("Event poll failed")? {
            let event = event::read()
                .context("event read failed")?;
            
            if let Event::Key(key_event) = event && key_event.kind == KeyEventKind::Press {
                match (key_event.code, key_event.modifiers) {
                    (KeyCode::Esc, KeyModifiers::NONE) => self.is_running = false,
                    (KeyCode::Tab, KeyModifiers::NONE) => match self.focus {
                            Focus::CommitType => self.focus = Focus::MsgInput,
                            Focus::MsgInput => self.focus = Focus::CommitType
                        },
                    (KeyCode::Char('x'), KeyModifiers::CONTROL) => self.is_finishing = true,
                    _ => {}
                }

                match self.focus {
                    Focus::CommitType => self.tabs.event_handle(key_event),
                    Focus::MsgInput => self.msg.event_handle(key_event)
                }
            }
        }
        Ok(())
    }

    fn finish(&mut self, frame: &mut Frame) -> color_eyre::Result<()> {
        let commit_type = self.tabs.titles[self.tabs.tab_index].clone();
        let commit_msg = format!("{}: {}", commit_type, self.msg.text);

        let (text, style) = git_controller::commit(commit_msg, None)?;

        let final_text = Paragraph::new(text)
            .block(Block::bordered())
            .style(style);

        frame.render_widget(final_text, frame.area());

        
        Ok(())
    }
}