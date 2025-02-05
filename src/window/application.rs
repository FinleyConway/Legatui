use crate::audio::{
    audio_player::AudioPlayer, audio_clip::AudioClip
};

use ratatui;
use ratatui::widgets;

use crossterm::event;

pub struct Application
{
    pub audio_player: AudioPlayer,
    should_close: bool,
}

impl Application
{
    pub fn new() -> Application
    {
        Application
        {
            audio_player: AudioPlayer::new(),
            should_close: false,
        }
    }

    pub fn run(&mut self, terminal: &mut ratatui::DefaultTerminal) -> std::io::Result<()>
    {
        while !self.should_close
        {
            // draw ui
            terminal.draw(|frame| self.draw(frame))?;

            // handle polling events
            self.handle_events()?;
        }

        return Ok(());
    }

    pub fn close(&mut self) -> ()
    {
        self.should_close = true;
    }

    fn temp(&mut self) -> () // test clip playing
    {
        let clip = AudioClip::new("/home/finley/Music/Ruby the Hatchet - Tomorrow Never Comes.mp3");
        self.audio_player.try_play(&clip).unwrap();
    }

    fn handle_events(&mut self) -> std::io::Result<()>
    {
        match event::read()?
        {
            // if a key was pressed
            event::Event::Key(key_event) => 
            {
                if let event::KeyEventKind::Press = key_event.kind 
                {
                    self.handle_key_events(key_event);
                }
            }

            _ => {}
        }

        return Ok(());
    }

    fn handle_key_events(&mut self, key_event: event::KeyEvent) -> ()
    {
        match key_event.code
        {
            event::KeyCode::Char('q') => self.close(),
            event::KeyCode::Char(' ') => self.temp(),
            event::KeyCode::Char('p') => self.audio_player.toggle_pause(),
            event::KeyCode::Char('s') => self.audio_player.stop(),
            _ => {}
        }
    }

    fn draw(&self, frame: &mut ratatui::Frame) -> ()
    {
        let data = self.audio_player.try_get_playing_data();

        match data
        {
            Some(data) =>
            {
                let text = widgets::Paragraph::new(format!("Playing: {}, By: {}", data.get_song(), data.get_artist()));

                frame.render_widget(text, frame.area());
            },
            None => {}
        }
    }
}

#[cfg(test)]
mod tests
{
    use super::*;
    use std::io;
    use crossterm::event;

    #[test]
    fn test_key_event() -> io::Result<()> 
    {
        let mut app = Application::new();
        app.handle_key_events(event::KeyCode::Char('q').into());
        assert_eq!(app.should_close, true);

        return Ok(());
    }
}