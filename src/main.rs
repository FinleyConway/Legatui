pub mod audio_clip;

use std::time;
use audio_clip::AudioClip;
use rodio::{self, Source};

pub struct PlayingData
{
    name_of_song: String,
    name_of_artist: String,
    duration: time::Duration
    // TODO: store iterators of the audio samples
}

impl PlayingData
{
    pub fn get_song(&self) -> &str
    {
        return &self.name_of_song;
    }

    pub fn get_artist(&self) -> &str
    {
        return &self.name_of_artist;
    }

    pub fn get_duration(&self) -> (u64, u64) // maybe return a struct for readability
    {
        let seconds: u64 = self.duration.as_secs() % 60;
        let minutes: u64 = self.duration.as_secs() / 60;

        return (minutes, seconds);
    }
}

struct JukeBox
{
    stream: rodio::OutputStream,
    track_list: rodio::Sink, // handles the currently playing audio
    current_track: Option<PlayingData>, // keeps track of the playing audio data
}

impl JukeBox
{
    fn new() -> JukeBox
    {
        let (output_stream, handle) = rodio::OutputStream::try_default().unwrap();
        let sink = rodio::Sink::try_new(&handle).unwrap();

        return JukeBox 
        {
            stream: output_stream,
            track_list: sink,
            current_track: None,
        }
    }

    fn try_play_track(&mut self, record: &AudioClip) -> Result<(), ()>
    {
        // try to load the audio 
        let source = record.try_load_source();
        let source = match source
        {
            Some(source) => source,
            None => return Err(()), // TODO: provide a better error
        };

        // attempt to retrieve the audio duration
        let source_duration = source.total_duration().unwrap_or_default();

        // keep track of the playing data and add it to the sink
        self.current_track = Some(PlayingData
        {
            name_of_song: record.title.clone(),
            name_of_artist: record.artist.clone(),
            duration: source_duration,
        });

        // remove any playing clip and add new clip
        self.remove_current_playing();
        self.track_list.append(source);

        return Ok(());
    }

    fn remove_current_playing(&self) -> ()
    {
        if self.is_playing()
        {
            self.track_list.stop();
        }
    }

    fn is_playing(&self) -> bool
    {
        return !self.track_list.empty();
    }
}

fn main() 
{
    let mut jukebox = JukeBox::new();
    let record = AudioClip::try_new("/var/home/finley/RustProjects/audio_test/target/Ruby the Hatchet - Tomorrow Never Comes.mp3");
    let record = match record
    {
        Some(record) => record,
        None => panic!("No audio found"),
    };

    let _ = jukebox.try_play_track(&record);

    jukebox.track_list.sleep_until_end();
}

fn fmt_duration(minutes: u64, seconds: u64) -> String
{
    let minutes = fmt_time(minutes);
    let seconds = fmt_time(seconds);

    return format!("{} : {}", minutes, seconds);
}

fn fmt_time(time: u64) -> String
{
    if time < 10
    {
        return format!("{:02}", time);
    }

    return time.to_string();
}