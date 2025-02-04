mod window;
mod audio;

use audio::audio_reader::AudioReader;
use window::application::Application;
use ratatui;
use std::{path, vec::Vec};

fn main() -> std::io::Result<()>
{   
    // let args: Vec<String> = env::args().collect();
    // let path = &args[1];


    let path = path::Path::new("/home/finley/Music/");
    let mut files = Vec::new();

    AudioReader::new().find(path, &mut |e| files.push(e))?;

    dbg!(files);

    return Ok(());
    // initialse terminal and application
    let mut terminal = ratatui::init();
    let application = Application::new().run(&mut terminal);

    ratatui::restore();

    return application;
}