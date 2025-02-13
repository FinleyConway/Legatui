use std::time;

use rodio::Source;

use super::audio_clip::AudioClip;

pub struct AudioPlayer
{
    _stream: rodio::OutputStream, 
    stream_handler: rodio::Sink,        // handles the currently playing audio
    current_track: Option<PlayingData>, // keeps track of the playing audio data
}

impl AudioPlayer
{
    pub fn try_new() -> Result<AudioPlayer, &'static str>
    {
        let output_stream = rodio::OutputStream::try_default();
        let (output_stream, handle) = match output_stream
        {
            Ok(stream) => stream,
            Err(_) => return Err("Failed to create the output stream!"),
        };

        let sink = rodio::Sink::try_new(&handle); 
        let sink = match sink
        {
            Ok(s) => s,
            Err(_) => return Err("Failed to setup audio player!"),
        };

        return Ok(AudioPlayer 
        {
            _stream: output_stream,
            stream_handler: sink,
            current_track: None,
        });
    }

    pub fn try_play(&mut self, record: &AudioClip) -> Option<()>
    {
        // try to load the audio 
        let source = record.try_load_source()?;

        // attempt to retrieve the audio duration
        // should be safe to unwrap to default as duration is just a visual
        let source_duration = source.total_duration().unwrap_or_default();

        // stop any currently playing audio
        self.stop();

        // keep track of the playing data and add it to the sink
        self.current_track = Some(PlayingData
        {
            name_of_song: record.get_title().to_owned(),
            name_of_artist: record.get_artist().to_owned(),
            duration: source_duration,
        });
        
        // play the clip
        self.stream_handler.append(source);

        return Some(());
    }

    pub fn toggle_pause(&self) -> ()
    {
        if self.stream_handler.is_paused()
        {
            self.stream_handler.play();
        }
        else
        {
            self.stream_handler.pause();
        }
    }

    pub fn stop(&mut self) -> ()
    {
        self.stream_handler.stop();
        self.current_track = None;
    }

    pub fn is_paused(&self) -> bool
    {
        return self.stream_handler.is_paused();
    }

    pub fn is_playing(&self) -> bool
    {
        return !self.stream_handler.empty();
    }

    pub fn try_get_playing_data(&self) -> &Option<PlayingData>
    {
        return &self.current_track;
    }
}

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