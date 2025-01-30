use std::{fs, io};
use rodio::{Decoder, OutputStream, Sink, Source};

fn main() 
{
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    let source = load_audio_source("/var/home/finley/RustProjects/audio-test/target/GOBLINSMOKER - Toad King EP [FULL ALBUM] 2018 4.mp3".to_string());
    let source = match source
    {
        Some(source) => source,
        None => panic!("No audio source found"),
    };

    let total_duration = source.total_duration();
    let mut total_minutes: u64 = 0;
    let mut total_seconds: u64 = 0;

    match total_duration 
    {
        Some(dur) => 
        {
            total_minutes = dur.as_secs() / 60;
            total_seconds = dur.as_secs() % 60;
        },
        None => println!("No duration found!"),
    }

    sink.append(source);

    while !sink.empty()
    {
        let current_minutes = sink.get_pos().as_secs() / 60;
        let current_seconds = sink.get_pos().as_secs() % 60;

        println!("[{current_minutes} : {current_seconds}] - [{total_minutes} : {total_seconds}]");
    }
}

fn load_audio_source(file_path: String) -> Option<Decoder<io::BufReader<fs::File>>>
{
    // attempt to retrieve the audio file.
    let file = fs::File::open(file_path);
    let file = match file 
    {
        Ok(file) => file,
        Err(_) => return None,
    };

    // attempts and decodes the audio file.
    let source = Decoder::new(io::BufReader::new(file));
    
    match source 
    {
        Ok(source) => return Some(source),
        Err(_) => return None,
    };
}