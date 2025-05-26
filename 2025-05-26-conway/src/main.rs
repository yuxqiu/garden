use conway::Conway;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, MouseEventKind},
    execute,
};
use ratatui::{
    Frame,
    style::Color,
    widgets::canvas::{Canvas, Line},
};

mod conway;

fn main() -> std::io::Result<()> {
    execute!(std::io::stdout(), EnableMouseCapture)?;

    let mut terminal = ratatui::init();
    let size = terminal.size()?;
    let mut conway = Conway::new(size.width, size.height);

    'main: loop {
        terminal.draw(|frame| {
            draw(frame, &conway);
        })?;

        loop {
            match event::read()? {
                Event::Key(key_event) => match key_event.code {
                    event::KeyCode::Char('n') => {
                        conway.next();
                        break;
                    }
                    event::KeyCode::Char('q') => break 'main,
                    _ => (),
                },
                Event::Resize(width, height) => {
                    conway.resize(width, height);
                    break;
                }
                Event::Mouse(m) if m.kind == MouseEventKind::Down(event::MouseButton::Left) => {
                    conway.set(m.row, m.column);
                    break;
                }
                _ => (),
            }
        }
    }

    execute!(std::io::stdout(), DisableMouseCapture)?;
    ratatui::restore();

    Ok(())
}

fn draw(frame: &mut Frame, conway: &Conway) {
    let area = frame.area();

    let canvas = Canvas::default()
        .x_bounds([0., area.width as f64])
        .y_bounds([0., area.height as f64])
        .paint(|ctx| {
            let state = conway.state();
            for i in 0..conway.height() {
                for j in 0..conway.width() {
                    let index = i * conway.width() + j;
                    if state[index as usize] {
                        let i = area.height - i;

                        ctx.draw(&Line {
                            x1: j as f64,
                            y1: i as f64,
                            x2: j as f64,
                            y2: i as f64,
                            color: Color::White,
                        });
                    }
                }
            }
        });

    frame.render_widget(canvas, area);
}
