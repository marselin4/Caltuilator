mod calculos;

use ratatui::{
    DefaultTerminal, Frame,
    widgets::{Paragraph, Borders, Block},
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
};

struct App {
    input: String,
    resultado: Option<Result<i32, String>>,
    historial: Vec<String>,
}

impl App {
    fn new() -> Self {
        App {
            input: String::new(),
            resultado: None,
            historial: Vec::new(),
        }
    }

    fn evaluar(&mut self) {
        if self.input.is_empty() { return; }
        let res = calculos::evaluar(&self.input);
        match &res {
            Ok(n) => self.historial.push(format!("{} = {}", self.input, n)),
            Err(e) => self.historial.push(format!("{} → Error: {}", self.input, e)),
        }
        self.resultado = Some(res);
        self.input.clear();
    }
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let mut app = App::new();
    ratatui::run(|terminal| run_app(terminal, &mut app))?;
    Ok(())
}

fn run_app(terminal: &mut DefaultTerminal, app: &mut App) -> std::io::Result<()> {
    loop {
        terminal.draw(|frame| render(frame, app))?;

        if let crossterm::event::Event::Key(key) = crossterm::event::read()? {
            if key.kind == crossterm::event::KeyEventKind::Press {
                match key.code {
                    crossterm::event::KeyCode::Esc => break Ok(()),
                    crossterm::event::KeyCode::Enter => app.evaluar(),
                    crossterm::event::KeyCode::Backspace => { app.input.pop(); }
                    crossterm::event::KeyCode::Char(c) => app.input.push(c),
                    _ => {}
                }
            }
        }
    }
}

fn render(frame: &mut Frame, app: &App) {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(vec![
            Constraint::Length(3),   // input
            Constraint::Length(3),   // resultado
            Constraint::Min(0),      // historial
        ])
        .split(frame.area());

    // Input
    frame.render_widget(
        Paragraph::new(app.input.as_str())
            .block(Block::new().borders(Borders::ALL).title(" Expresión (Esc para salir) ")),
        layout[0],
    );

    // Resultado
    let (texto, color) = match &app.resultado {
        None => ("Ingresá una expresión y presioná Enter".to_string(), Color::Gray),
        Some(Ok(n)) => (format!("= {}", n), Color::Green),
        Some(Err(e)) => (e.clone(), Color::Red),
    };
    frame.render_widget(
        Paragraph::new(texto)
            .style(Style::default().fg(color))
            .block(Block::new().borders(Borders::ALL).title(" Resultado ")),
        layout[1],
    );

    // Historial
    let historial_texto = if app.historial.is_empty() {
        "Sin operaciones todavía...".to_string()
    } else {
        app.historial.join("\n")
    };
    frame.render_widget(
        Paragraph::new(historial_texto)
            .block(Block::new().borders(Borders::ALL).title(" Historial ")),
        layout[2],
    );
}
