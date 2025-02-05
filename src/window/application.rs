use crate::audio::{
    audio_player::AudioPlayer, audio_clip::AudioClip
};

use ratatui;
use ratatui::widgets;
use ratatui::text;
use ratatui::symbols;

use crossterm::event;

pub struct Application
{
    pub audio_player: AudioPlayer,
    should_close: bool,

    clips: Vec<AudioClip>,
    state: widgets::ListState,
}

impl Application
{
    pub fn new(clips: Vec<AudioClip>) -> Application
    {
        Application
        {
            audio_player: AudioPlayer::new(),
            should_close: false,
            clips: clips,
            state: widgets::ListState::default(),
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
            event::KeyCode::Char('p') => self.audio_player.toggle_pause(),
            event::KeyCode::Char('s') => self.audio_player.stop(),
            event::KeyCode::Down      => self.state.select_next(),
            event::KeyCode::Up        => self.state.select_previous(),
            event::KeyCode::Enter     => self.handle_selected(),
            _ => {}
        }
    }

    fn handle_selected(&mut self)
    {
        if let Some(i) = self.state.selected() 
        {
            let clip = &self.clips[i];

            self.audio_player.try_play(&clip).unwrap();
        }
    }
    
    fn draw(&mut self, frame: &mut ratatui::Frame) -> ()
    {
        let data = self.audio_player.try_get_playing_data();
        let data = match data {
            Some(data) => format!("Playing: {} - {}", data.get_song(), data.get_artist()),
            None => "No Song Playing".to_string(),
        };

        let block = widgets::Block::new()
            .title(text::Line::raw(data).centered())
            .borders(widgets::Borders::TOP)
            .border_set(symbols::border::EMPTY);

        
        let mut songs: Vec<&str> = Vec::default();
        songs.reserve(self.clips.len());

        for clip in self.clips.iter()
        {
            songs.push(clip.get_title());
        }

        let list = widgets::List::new(songs)
            .block(block)
            .highlight_symbol(">")
            .highlight_spacing(widgets::HighlightSpacing::Always);

        widgets::StatefulWidget::render(list, frame.area(), &mut frame.buffer_mut(), &mut self.state);
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
        let mut app = Application::new(Vec::default());
        app.handle_key_events(event::KeyCode::Char('q').into());
        assert_eq!(app.should_close, true);

        return Ok(());
    }
}