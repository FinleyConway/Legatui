#[derive(Debug, Default)]
pub struct Artist
{
    name: String,
    albums: Vec<Album>,
}

impl Artist
{
    fn get_name(&self) -> &str
    {
        return &self.name;
    }

    fn get_albums(&self) -> &Vec<Album>
    {
        return &self.albums;
    }
}

#[derive(Debug, Default)]
pub struct Album
{
    name: String,
    songs: Vec<Song>,
}

impl Album
{
    fn get_name(&self) -> &str
    {
        return &self.name;
    }

    fn get_songs(&self) -> &Vec<Song>
    {
        return &self.songs;
    }
}

#[derive(Debug, Default)]
pub struct Song
{
    name: String,
    file_path: String,
}

impl Song
{
    fn get_name(&self) -> &str
    {
        return &self.name;
    }

    fn get_file_path(&self) -> &str
    {
        return &self.file_path;
    }
}