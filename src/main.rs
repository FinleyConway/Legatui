use std::{fs, io, time};
use rodio;
use audiotags;

#[derive(Default)]
struct AudioRecord
{
    title: String,
    album: String,
    artist: String,
    genre: String,

    file_path: String,
}

struct JukeBox
{
    stream: rodio::OutputStream,
    track_list: rodio::Sink, // handles the currently playing audio
}

impl JukeBox
{
    fn new() -> JukeBox
    {
        let (output_stream, handle) = rodio::OutputStream::try_default().unwrap();
        let sink = rodio::Sink::try_new(&handle).unwrap();
        
        return JukeBox 
        {
            stream: output_stream,
            track_list: sink,
        }
    }

    fn add_track(&mut self, record: &AudioRecord)
    {
        let source = load_audio_source(&record.file_path);
        let source = match source
        {
            Some(source) => source,
            None => panic!("No audio source found"),
        };

        self.track_list.append(source);
    }
}

fn main() 
{
    // let mut jukebox = JukeBox::new();
    // let record = AudioRecord {
    //     file_path: "/var/home/finley/RustProjects/audio-test/target/Ruby the Hatchet - Tomorrow Never Comes.mp3".to_string(),
    // };

    // jukebox.add_track(&record);

    // jukebox.track_list.sleep_until_end();

    get_audio_info(&"/var/home/finley/RustProjects/audio-test/target/Ruby the Hatchet - Tomorrow Never Comes.mp3".to_string())
}

fn duration_to_seconds(duration: &time::Duration) -> u64
{
    return duration.as_secs() % 60;
}

fn duration_to_minutes(duration: &time::Duration) -> u64
{
    return duration.as_secs() / 60;
}

fn fmt_time(time: u64) -> String
{
    if time < 10
    {
        return format!("{:02}", time);
    }

    return time.to_string();
}

fn fmt_duration(minutes: u64, seconds: u64) -> String
{
    let minutes = fmt_time(minutes);
    let seconds = fmt_time(seconds);

    return format!("{} : {}", minutes, seconds);
}

fn get_audio_info(file_path: &String)
{
    let audio_tags = audiotags::Tag::new().read_from_path(&file_path).unwrap();

    let album = audio_tags.album().unwrap();

    let artist = album.artist.unwrap();
    let album_name = album.title;
    let song_name = audio_tags.title().unwrap();
    let song_genre = audio_tags.genre().unwrap();

    println!("Artist: {}, Album: {}, Song: {}, genre: {}", 
        artist, album_name, song_name, song_genre);
}

fn load_audio_source(file_path: &String) -> Option<rodio::Decoder<io::BufReader<fs::File>>>
{
    // attempt to retrieve the audio file.
    let file = fs::File::open(file_path).ok()?;

    // attempts and decodes the audio file.
    let source = rodio::Decoder::new(io::BufReader::new(file));
    
    match source 
    {
        Ok(source) => return Some(source),
        Err(_) => return None,
    };
}