use std::{
    io::stdout,
    time::{Duration, Instant},
};

use crossterm::{
    event::{
        self, Event, KeyCode, KeyEventKind, KeyboardEnhancementFlags, PopKeyboardEnhancementFlags,
        PushKeyboardEnhancementFlags,
    },
    execute,
};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Margin},
    widgets::Block,
};

use crate::{runtime::Runner, tui::display::DisplayWidget};

pub struct App<'a> {
    runner: &'a mut Runner,
    is_running: bool,
    last_frame_timestamp: Instant,
}

impl App<'_> {
    pub fn start(runner: &mut Runner) -> std::io::Result<()> {
        ratatui::run(|terminal| App::new(runner).run(terminal))?;
        Ok(())
    }

    fn run(&mut self, terminal: &mut DefaultTerminal) -> std::io::Result<()> {
        let result = execute!(
            stdout(),
            PushKeyboardEnhancementFlags(
                KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES
                    | KeyboardEnhancementFlags::REPORT_EVENT_TYPES
            )
        );

        result.expect("Kitty protocol not available!");
        while self.is_running {
            self.handle_keys()?;

            let frame_timestamp = Instant::now();
            let frame_time = frame_timestamp.duration_since(self.last_frame_timestamp);

            self.last_frame_timestamp = frame_timestamp;
            self.runner.run(frame_time);

            terminal.draw(|frame| self.draw(frame))?;
        }

        let _ = execute!(stdout(), PopKeyboardEnhancementFlags);
        Ok(())
    }

    fn draw(&mut self, frame: &mut Frame) {
        let block = Block::bordered().title("Chip-8");
        let display = DisplayWidget::new(self.runner.display());

        let block_area = frame
            .area()
            .centered(Constraint::Length(130), Constraint::Length(34));
        let display_area = block_area.inner(Margin::new(1, 1));

        frame.render_widget(block, block_area);
        frame.render_widget(display, display_area);
    }

    fn handle_keys(&mut self) -> std::io::Result<()> {
        if event::poll(Duration::from_millis(16))? {
            match event::read()? {
                Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                    if key_event.code == KeyCode::Esc {
                        self.is_running = false;
                        return Ok(());
                    }

                    let mapped_key = App::map_key(key_event.code);
                    if mapped_key >= 0 {
                        self.runner.set_key(mapped_key as u8, true);
                    }
                }
                Event::Key(key_event) if key_event.kind == KeyEventKind::Release => {
                    let mapped_key = App::map_key(key_event.code);
                    if mapped_key >= 0 {
                        self.runner.set_key(mapped_key as u8, false);
                    }
                }
                _ => {}
            };
        }

        Ok(())
    }

    fn map_key(code: KeyCode) -> i8 {
        match code {
            KeyCode::Char('1') => 0x1,
            KeyCode::Char('2') => 0x2,
            KeyCode::Char('3') => 0x3,
            KeyCode::Char('4') => 0xC,

            KeyCode::Char('q') => 0x4,
            KeyCode::Char('w') => 0x5,
            KeyCode::Char('e') => 0x6,
            KeyCode::Char('r') => 0xD,

            KeyCode::Char('a') => 0x7,
            KeyCode::Char('s') => 0x8,
            KeyCode::Char('d') => 0x9,
            KeyCode::Char('f') => 0xE,

            KeyCode::Char('z') => 0xA,
            KeyCode::Char('x') => 0x0,
            KeyCode::Char('c') => 0xB,
            KeyCode::Char('v') => 0xF,
            _ => -1,
        }
    }

    fn new(runner: &mut Runner) -> App<'_> {
        App {
            runner: runner,
            is_running: true,
            last_frame_timestamp: Instant::now(),
        }
    }
}
