use std::{io, fs};

#[derive(Debug, Default)]
pub struct Artist
{
    pub name: String,
    pub albums: Vec<Album>,
}

impl Artist
{
    pub fn new(name: String) -> Artist
    {
        return Artist
        {
            name: name,
            albums: Vec::default(),
        } 
    }

    pub fn get_name(&self) -> &str
    {
        return &self.name;
    }

    pub fn get_albums(&mut self) -> &mut Vec<Album>
    {
        return &mut self.albums;
    }
}

#[derive(Debug, Default)]
pub struct Album
{
    pub name: String,
    pub songs: Vec<Song>,
}

impl Album
{
    pub fn new(name: String) -> Album
    {
        return Album
        {
            name: name,
            songs: Vec::default(),
        }
    }

    pub fn get_name(&self) -> &str
    {
        return &self.name;
    }

    pub fn get_songs(&mut self) -> &mut Vec<Song>
    {
        return &mut self.songs;
    }
}

#[derive(Debug, Default)]
pub struct Song
{
    pub name: String,
    pub file_path: String,
}

impl Song
{
    pub fn new(name: String, file_path: String) -> Song
    {
        return Song 
        {
            name: name,
            file_path: file_path,
        };
    }

    pub fn get_name(&self) -> &str
    {
        return &self.name;
    }

    pub fn get_file_path(&self) -> &str
    {
        return &self.file_path;
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
            Err(_) => return None,
        };
    }
}