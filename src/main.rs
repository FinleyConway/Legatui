mod window;
mod audio;

use window::application::Application;
use ratatui;

fn main() -> std::io::Result<()>
{   
    // initialse terminal and application
    let mut terminal = ratatui::init();
    let application = Application::new().run(&mut terminal);

    ratatui::restore();

    return application;
}