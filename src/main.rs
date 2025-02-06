mod window;
mod audio;

use audio::{audio_clip::AudioClip, audio_importer::AudioImporter};
use window::application::Application;
use ratatui;

fn main() -> std::io::Result<()>
{   
    // import clips
    let clips = AudioImporter::default().gather_clips_args()?;

    // initialse terminal and application
    let mut terminal = ratatui::init();
    let application = Application::new(clips).run(&mut terminal);

    ratatui::restore();

    return application;
}