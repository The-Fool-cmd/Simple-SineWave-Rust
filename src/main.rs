use crossterm::event::KeyCode;
use ratatui::{
    Frame,
    crossterm::event::{self, Event},
    layout::{Constraint, Layout},
    style::{Color, Style, Stylize},
    symbols::Marker,
    text::Line,
    widgets::{Axis, Block, Chart, Dataset, Paragraph},
};
use std::time::{Duration, Instant};

// Stores state of app
struct App {
    freq: f64,
    amp: f64,
    started_at: Instant,
    should_quit: bool,
    plot: Vec<(f64, f64)>,
}

// bougee
impl Default for App {
    fn default() -> Self {
        App {
            freq: 1.0,
            amp: 1.0,
            started_at: Instant::now(),
            should_quit: false,
            plot: Vec::new(),
        }
    }
}

impl App {
    // Handles all the inputs and changes App state accordingly
    fn process_input(&mut self, key: KeyCode) {
        match key {
            KeyCode::Char('q') => {
                self.should_quit = true;
            }
            KeyCode::Left => {
                self.freq = (self.freq - 0.1).max(1.0);
                self.update_plot();
            }
            KeyCode::Right => {
                self.freq = (self.freq + 0.1).min(10.0);
                self.update_plot();
            }
            KeyCode::Down => {
                self.amp = (self.amp - 0.1).max(0.0);
                self.update_plot();
            }
            KeyCode::Up => {
                self.amp = (self.amp + 0.1).min(10.0);
                self.update_plot();
            }
            KeyCode::Char('r') => {
                self.amp = 1.0;
                self.freq = 1.0;
                self.update_plot();
            }
            _ => {}
        }
    }

    fn ui(&self, frame: &mut Frame) {
        // Split layout vertically into smol rect and beeg rect
        let [top, bottom] =
            Layout::vertical(vec![Constraint::Min(3), Constraint::default()]).areas(frame.area());
        // The stuff we would like to know
        let debug_text = Line::from(vec![
            "Time elapsed: ".into(),
            format!("{:?}", self.started_at.elapsed()).into(),
            " Amplitude: ".into(),
            self.amp.to_string().into(),
            " Frequency: ".into(),
            self.freq.to_string().into(),
        ]);
        // Create debug paragraph from 'debug_text'
        let debug_line: Paragraph = Paragraph::new(debug_text)
            .block(Block::bordered().title("Debug Line".bold().into_centered_line()));

        // Create dataset using the vector of coordinates
        let dataset = Dataset::default()
            .marker(Marker::Braille)
            .style(Style::default().fg(Color::Cyan))
            .data(&self.plot);

        // Create chart from with the dataset
        let graph: Chart = Chart::new(vec![dataset])
            .x_axis(
                Axis::default()
                    .title("X Axis")
                    .style(Style::default().fg(Color::LightBlue))
                    .bounds([-10.0, 10.0]),
            )
            .y_axis(
                Axis::default()
                    .title("Y Axis")
                    .style(Style::default().fg(Color::LightRed))
                    .bounds([-10.0, 10.0]),
            )
            .block(Block::bordered().title("Sine Wave".bold().into_centered_line()));

        // Render both blocks
        frame.render_widget(debug_line, top);
        frame.render_widget(graph, bottom);
    }

    fn update_plot(&mut self) {
        self.plot.clear();
        let n = 10000;
        let x_min = -10.0;
        let x_max = 10.0;
        let span = x_max - x_min;

        for i in 0..n {
            let t = i as f64 / (n - 1) as f64;
            let x = x_min + t * span;
            let y = self.amp * (self.freq * x).sin();
            self.plot.push((x, y));
        }
    }
}

fn main() -> Result<(), std::io::Error> {
    let mut terminal = ratatui::init();

    let mut app = App::default();
    app.update_plot();

    loop {
        if app.should_quit {
            break;
        }
        // Some fucking black magic this is
        terminal.draw(|f| app.ui(f))?;
        // **Blocking** waiting for input
        if event::poll(Duration::from_millis(16))? {
            if let Event::Key(key) = event::read()? {
                app.process_input(key.code);
            }
        }
    }
    // If app crashes before this, too bad
    ratatui::restore();
    return Ok(());
}
