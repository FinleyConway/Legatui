use std::{io, fs};
use std::path;
use std::env;

use crate::audio::audio_data::*;

#[derive(Default)]
pub struct AudioImporter;

impl AudioImporter
{
    pub fn gather_clips_args(&self) -> std::io::Result<Vec<Artist>> 
    {
        let working_dir = self.read_args();
        let path = path::Path::new(&working_dir);
        let mut artists: Vec<Artist> = Vec::new();

        self.find(path, &mut |file_path| 
        {
            let (title, artist_name, album_name) = match audiotags::Tag::new().read_from_path(&file_path) 
            {
                Ok(tag) => 
                (
                    tag.title().unwrap_or("Unknown").to_string(),
                    tag.artist().unwrap_or("Unknown Artist").to_string(),
                    tag.album_title().unwrap_or("Unknown Album").to_string(),
                ),
                Err(_) => ("Unknown".to_string(), "Unknown Artist".to_string(), "Unknown Album".to_string()), // no metadata on the audio clip
            };
        
            self.handle_clip(title, artist_name, album_name, file_path.display().to_string(), &mut artists);
        })?;

        Ok(artists)
    }

    fn read_args(&self) -> String
    {
        let args: Vec<String> = env::args().collect();

        // very simple, just checks if it has a passed in directory
        if args.len() == 2
        {
            return args[1].clone();
        }

        return args[0].clone();
    }   

    fn find(&self, path: &path::Path, func: &mut dyn FnMut(path::PathBuf) -> ()) -> io::Result<()>
    {
        // recurse through a directory and callback a audio file found
        if path.is_dir()
        {
            for entry in fs::read_dir(path)? 
            {
                let entry = entry?.path();
                let path = entry.as_path();

                if path.is_dir() 
                {
                    self.find(&path, func)?;
                } 
                else if path.is_file()
                {
                    if self.is_compatible_extension(&entry)
                    {
                        func(entry);
                    }
                }
            }
        }

        return Ok(());
    }   

    fn is_compatible_extension(&self, path: &path::PathBuf) -> bool
    {
        const COMPATIBLE_EXTENSION: [&str; 4] = ["mp3", "wav", "ogg", "flac"];

        for ext in COMPATIBLE_EXTENSION
        {
            let extension = path.extension();
            
            match extension
            {
                Some(extension) => return extension == ext,
                None => return false,
            }
        }
        
        return false;
    }

    fn handle_clip(&self, song_name: String, artist_name: String, album_name: String, file_path: String, artists: &mut Vec<Artist>) -> ()
    {
        // find or create the artist
        let artist = match artists.iter_mut().find(|a| a.get_name() == artist_name) 
        {
            Some(artist) => artist,
            None => 
            {
                artists.push(Artist::new(artist_name));
                artists.last_mut().expect("shouldn't be empty")
            }
        };

        // find or create the album within the artist
        let album = match artist.get_albums().iter_mut().find(|a| a.get_name() == album_name) 
        {
            Some(album) => album,
            None => 
            {
                artist.get_albums().push(Album::new(album_name));
                artist.get_albums().last_mut().expect("shouldn't be empty")
            }
        };

        // add the song to the album
        album.get_songs().push(Song::new(song_name, file_path));
    }
}