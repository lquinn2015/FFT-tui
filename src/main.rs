mod app;
mod event;
mod ui;

use app::App;
use event::{Event, Events};

use std::{
    f32::consts::PI, 
    io::{Write, stdout},
    io,
    error::Error, 
    time::Duration
};
use num_complex::Complex;

use termion::
{
    event::Key,
    raw::IntoRawMode,
    screen::ToMainScreen,
    input::MouseTerminal,
};
use tui::
{
    backend::TermionBackend,
    Terminal,
};



fn main() -> Result<(), Box<dyn Error>>{

    let mut app = app::App::new("FMcapture1.dat".into());
    let events = Events::new(Duration::from_millis(250));
   
    std::panic::set_hook(Box::new(move |x| {
        stdout()
            .into_raw_mode().unwrap()
            .suspend_raw_mode().unwrap();
        write!(stdout().into_raw_mode().unwrap(), "{}", ToMainScreen).unwrap();
        print!("{:?}", x);
    }));

    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    
    loop {
        terminal.draw(|f| ui::draw(f, &mut app))?;

        match events.next()? {
            Event::Input(key) => match key {
                Key::Ctrl('c') => {
                    app.should_quit = true;
                }
                _ => { 
                    App::on_input(&mut app, key);
                }
            }
            Event::Tick => {
                app::App::on_tick(&mut app);
            }
        }

        if app.should_quit {
            break;
        }
    }
    Ok(())
}



