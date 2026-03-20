use std::time::Duration;

use color_eyre::eyre::Context;
use ratatui::{DefaultTerminal, Frame, crossterm::event::{self, Event, KeyCode, KeyEventKind}, layout::{Constraint, Layout}, widgets::{Block, Paragraph}};

use crate::core::input::InputHandler;

struct CommitHandler {
    msg: InputHandler,
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
    const fn new(commit_type: CommitType) -> Self {
        Self {
            msg: InputHandler::new(),
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
        let layout =  Layout::vertical([
            Constraint::Percentage(50),
            Constraint::Percentage(50)
        ]);

        let [area1, area2] = frame.area().layout(&layout);

        let input1 = Paragraph::new(self.msg.text.as_str())
            .block(Block::bordered().title("Input"));
        frame.render_widget(input1, area1);

        let input2 = Paragraph::new("")
            .block(Block::bordered().title("Input2"));

        frame.render_widget(input2, area2);
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
                }

                self.msg.event_handle(key_event);
            }
        }

        Ok(())
    }
}