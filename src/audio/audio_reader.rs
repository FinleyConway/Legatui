use std::{io, fs};
use std::path::{Path, PathBuf};

pub struct AudioReader;

impl AudioReader
{
    pub fn new() -> AudioReader
    {
        return AudioReader;
    }

    pub fn find(&self, path: &Path, cb: &mut dyn FnMut(PathBuf)) -> io::Result<()>
    {
        if path.is_dir()
        {
            for entry in fs::read_dir(path)? 
            {
                let entry = entry?.path();
                let path = entry.as_path();

                if path.is_dir() 
                {
                    self.find(&path, cb)?;
                } 
                else if path.is_file()
                {
                    cb(entry);
                }
            }
        }

        return Ok(());
    }   
}