mod window;
mod audio;

use std::env;
use std::path;

use audio::{audio_clip::AudioClip, audio_reader::AudioReader};
use window::application::Application;
use ratatui;

fn try_load_audio(working_dir: &str) -> std::io::Result<Vec<path::PathBuf>>
{
    let path = path::Path::new(working_dir);
    let mut files = Vec::new();

    AudioReader::default().find(path, &mut |e| files.push(e))?;

    return Ok(files);
}

fn gather_clips(working_dir: &str) -> std::io::Result<Vec<AudioClip>>
{
    let files = try_load_audio(working_dir)?;
    let mut clips: Vec<AudioClip> = Vec::default();

    clips.reserve(files.len());

    for file in files
    {
        clips.push(AudioClip::new(&file.display().to_string()));
    }

    return Ok(clips);
}

fn main() -> std::io::Result<()>
{   
    let args: Vec<String> = env::args().collect();

    if args.len() != 2
    {
        panic!("Stop"); // obv temp
    }

    let path = &args[1];

    let clips = gather_clips(&path)?;

    // initialse terminal and application
    let mut terminal = ratatui::init();
    let application = Application::new(clips).run(&mut terminal);

    ratatui::restore();

    return application;
}