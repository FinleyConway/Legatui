use std::{io, fs};
use std::path;
use std::env;

use crate::AudioClip;

#[derive(Default)]
pub struct AudioImporter;

impl AudioImporter
{
    pub fn gather_clips_args(&self) -> std::io::Result<Vec<AudioClip>>
    {
        let files = self.try_load_audio(&self.read_args())?;
        let mut clips: Vec<AudioClip> = Vec::default();
    
        clips.reserve(files.len());
    
        for file in files
        {
            clips.push(AudioClip::new(&file.display().to_string()));
        }
    
        return Ok(clips);
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

    fn try_load_audio(&self, working_dir: &str) -> std::io::Result<Vec<path::PathBuf>>
    {
        let path = path::Path::new(working_dir);
        let mut files = Vec::new();
    
        self.find(path, &mut |e| files.push(e))?;
    
        return Ok(files);
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
}