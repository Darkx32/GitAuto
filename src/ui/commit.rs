use std::time::Duration;

use color_eyre::eyre::Context;
use ratatui::{DefaultTerminal, Frame, crossterm::event::{self, Event, KeyCode, KeyEventKind}, layout::{Constraint, Layout}, style::{Color, Style}, widgets::{Block, Paragraph, Tabs}};

use crate::core::{input::InputHandler, tab::TabHandler};

struct CommitHandler {
    msg: InputHandler,
    tabs: TabHandler,
    commit_type: CommitType,
    is_running: bool,
    focus: Focus
}

pub enum CommitType {
    Feature,
    Docs,
    Fix,
    Improvement
}

enum Focus {
    MsgInput,
    CommitType
}

pub fn commit_app() -> color_eyre::Result<()> {
    let mut commit_handler = CommitHandler::new(CommitType::Improvement);

    ratatui::run(|terminal| commit_handler.run(terminal)).context("Error to run commit app")
}

impl CommitHandler {
    fn new(commit_type: CommitType) -> Self {
        let tabs_titles = vec!["feat", "docs", "fix", "improvement"]
            .iter()
            .map(|s| s.to_string())
            .collect();

        Self {
            msg: InputHandler::new(),
            tabs: TabHandler::new(tabs_titles),
            commit_type,
            is_running: true,
            focus: Focus::MsgInput
        }
    }

    fn run(&mut self, terminal: &mut DefaultTerminal) -> color_eyre::Result<()> {
        loop {
            if !self.is_running{
                break;
            }

            terminal.draw(|frame| self.render(frame))?;

            self.handle_events()?
        }
        Ok(())
    }

    fn render(&mut self, frame: &mut Frame) {
        let input_color = if matches!(self.focus, Focus::MsgInput) { Color::Yellow } else { Color::White };
        let tab_color = if matches!(self.focus, Focus::CommitType) { Color::Yellow } else { Color::White };

        let layout =  Layout::vertical([
            Constraint::Percentage(50),
            Constraint::Percentage(50)
        ]);

        let [area1, area2] = frame.area().layout(&layout);

        let input1 = Paragraph::new(self.msg.text.as_str())
            .block(Block::bordered().title("Input"))
            .style(Style::default().fg(input_color));

        frame.render_widget(input1, area1);

        let tab = Tabs::new(self.tabs.titles.clone())
            .block(Block::default().title("Commit Type"))
            .style(Style::default().fg(tab_color))
            .select(self.tabs.tab_index);

        frame.render_widget(tab, area2);
    }

    fn handle_events(&mut self) -> color_eyre::Result<()> {
        if event::poll(Duration::from_millis(250)).context("Event poll failed")? {
            let event = event::read()
                .context("event read failed")?;
            
            if let Event::Key(key_event) = event {
                if key_event.kind == KeyEventKind::Press {
                    if key_event.code == KeyCode::Esc {
                        self.is_running = false
                    }

                    if key_event.code == KeyCode::Tab {
                        match self.focus {
                            Focus::CommitType => self.focus = Focus::MsgInput,
                            Focus::MsgInput => self.focus = Focus::CommitType
                        }
                    }
                }

                match self.focus {
                    Focus::CommitType => self.tabs.event_handle(key_event),
                    Focus::MsgInput => self.msg.event_handle(key_event)
                }
            }
        }
        Ok(())
    }
}