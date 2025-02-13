use std::{fs, io};

#[derive(Debug, Default)]
pub struct AudioClip
{
    title: String,
    album: String,
    artist: String,
    file_path: String,
}

impl AudioClip
{
    pub fn new(file_path: &str) -> AudioClip
    {
        const DEFAULT_SONG: &str = "Unknown Song";
        const DEFAULT_ALBUM: &str = "Unknown Album";
        const DEFAULT_ARTIST: &str = "Unknown Artist";

        match audiotags::Tag::new().read_from_path(file_path) 
        {
            // if it was succesfull in reading the audio tags
            Ok(audio_tags) => 
            {
                // either get or set the values to default
                let song_name = audio_tags.title().unwrap_or(DEFAULT_SONG);
                let album_name = audio_tags.album_title().unwrap_or(DEFAULT_ALBUM);
                let artist_name = audio_tags.artist().unwrap_or(DEFAULT_ARTIST);

                return AudioClip 
                {
                    title: song_name.to_string(),
                    album: album_name.to_string(),
                    artist: artist_name.to_string(),
                    file_path: file_path.to_string(),
                }
            }
            Err(_) => 
            {
                return AudioClip
                {
                    title: DEFAULT_SONG.to_string(),
                    album: DEFAULT_ALBUM.to_string(),
                    artist: DEFAULT_ARTIST.to_string(),
                    file_path: file_path.to_string()
                }
            }
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

    pub fn get_title(&self) -> &str
    {
        return &self.title;
    }

    pub fn get_album(&self) -> &str
    {
        return &self.album;
    }

    pub fn get_artist(&self) -> &str
    {
        return &self.artist;
    }
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_new_blank()
    {
        let clip = AudioClip::new("");

        assert_eq!(clip.get_title(), "Unknown Song");
        assert_eq!(clip.get_album(), "Unknown Album");
        assert_eq!(clip.get_artist(), "Unknown Artist");
    }

    #[test]
    fn test_new()
    {
        let clip = AudioClip::new("src/audio/Test.mp3");

        assert_eq!(clip.get_title(), "Test");
        assert_eq!(clip.get_album(), "TestAlbum");
        assert_eq!(clip.get_artist(), "TestBand");
    }

    #[test]
    fn test_try_load_source()
    {
        let clip = AudioClip::new("src/audio/Test.mp3");
        let source = clip.try_load_source();

        assert_eq!(source.is_some(), true);
    }

    #[test]
    fn test_try_load_source_blank()
    {
        let clip = AudioClip::new("");
        let source = clip.try_load_source();

        assert_eq!(source.is_some(), false);
    }
}