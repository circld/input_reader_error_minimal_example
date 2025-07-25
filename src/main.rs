use anyhow::Result;
use clap::Parser;
use crossterm::{
    ExecutableCommand,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    crossterm::event::{self, Event, KeyCode},
    widgets::Paragraph,
};
use std::io::{BufWriter, IsTerminal, Read, stderr, stdin};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "breeze")]
#[command(about = "A terminal-based file explorer")]
pub struct Args {
    #[arg(default_value = ".")]
    pub directory: PathBuf,
}

fn main() -> Result<()> {
    let args = if stdin().is_terminal() {
        Args::parse()
    } else {
        let mut buffer = String::new();
        buffer.push_str("prog ");
        let _ = stdin().read_to_string(&mut buffer)?;
        Args::parse_from(buffer.trim().split_whitespace())
    };

    let directory = args.directory.canonicalize()?;
    let mut should_exit = false;

    let backend = CrosstermBackend::new(BufWriter::new(stderr()));
    let mut terminal = Terminal::new(backend)?;

    enable_raw_mode()?;
    stderr().execute(EnterAlternateScreen)?;

    while !should_exit {
        terminal.draw(|f| {
            f.render_widget(
                Paragraph::new(format!("Directory: {:?}\n\n\n'q' to quit", directory)),
                f.area(),
            );
        })?;
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => should_exit = true,
                _ => {}
            }
        };
    }

    stderr().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;

    Ok(())
}
