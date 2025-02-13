mod window;
mod audio;

use std::io;

use audio::{audio_clip::AudioClip, audio_importer::AudioImporter};
use window::application::Application;
use ratatui;

fn main() -> std::io::Result<()>
{   
    // import clips
    let clips = AudioImporter::default().gather_clips_args()
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

    // initialse terminal and application
    let mut terminal = ratatui::init();
    let mut application = Application::new(clips)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

    application.run(&mut terminal)?;

    ratatui::restore();

    return Ok(());
}