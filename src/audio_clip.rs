use std::{fs, io};

#[derive(Debug, Default)]
pub struct AudioClip
{
    pub title: String,
    pub album: String,
    pub artist: String,
    pub genre: String,
    file_path: String,
}

impl AudioClip
{
    pub fn try_new(file_path: &str) -> Option<AudioClip>
    {
        match audiotags::Tag::new().read_from_path(file_path) 
        {
            // if it was succesfull in reading the audio tags
            Ok(audio_tags) => 
            {
                // either get or set the values to default
                let artist = audio_tags.artist().unwrap_or_default();
                let album_name = audio_tags.album_title().unwrap_or_default();
                let song_name = audio_tags.title().unwrap_or_default();
                let song_genre = audio_tags.genre().unwrap_or_default();

                Some(AudioClip 
                {
                    title: song_name.to_string(),
                    album: album_name.to_string(),
                    artist: artist.to_string(),
                    genre: song_genre.to_string(),
                    file_path: file_path.to_string(),
                })
            }
            Err(_) => None,
        }
    }

    pub fn try_load_source(&self) -> Option<rodio::Decoder<io::BufReader<fs::File>>>
    {
        // attempt to retrieve the audio file.
        let file = fs::File::open(&self.file_path).ok()?;

        // attempts and decodes the audio file.
        let source = rodio::Decoder::new(io::BufReader::new(file));
        
        match source 
        {
            Ok(source) => return Some(source),
            Err(_) => return None, // TODO: provide a better error
        };
    }
}