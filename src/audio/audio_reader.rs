use std::{io, fs};
use std::path;

#[derive(Default)]
pub struct AudioReader;

impl AudioReader
{
    pub fn find(&self, path: &path::Path, func: &mut dyn FnMut(path::PathBuf) -> ()) -> io::Result<()>
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
}