use crate::audio::audio_data::*;
use crate::audio::{
    audio_player::AudioPlayer, audio_clip::AudioClip
};

use ratatui;
use ratatui::widgets::{self, Paragraph, Widget};
use ratatui::text;
use ratatui::symbols;

use crossterm::event;

pub struct Application
{
    pub audio_player: AudioPlayer,
    should_close: bool,

    clips: Vec<Artist>,

    current_state: usize,
    states: [widgets::ListState; 3],
}

impl Application
{
    pub fn new(clips: Vec<Artist>) -> Result<Application, &'static str>
    {
        return Ok(Application
        {
            audio_player: AudioPlayer::try_new()?,
            should_close: false,
            clips: clips,
            current_state: 0,
            states: [
                widgets::ListState::default(),
                widgets::ListState::default(),
                widgets::ListState::default(),
            ],
        });
    }

    pub fn run(&mut self, terminal: &mut ratatui::DefaultTerminal) -> std::io::Result<()>
    {
        // temp for wrapping index
        self.states[0].select(Some(0));

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

            event::KeyCode::Up        => self.previous(),
            event::KeyCode::Down      => self.next(),
            event::KeyCode::Left      => self.handle_current_state(true),
            event::KeyCode::Right     => self.handle_current_state(false),
            _ => {}
        }
    }

    fn handle_current_state(&mut self, backwards: bool)
    {
        if backwards
        {
            self.current_state = (self.current_state + self.states.len() - 1) % self.states.len();
        }
        else
        {
            self.current_state = (self.current_state + 1) % self.states.len()
        }
    }

    fn next(&mut self)
    {
        let current_state = &mut self.states[self.current_state];

        if let Some(selected) = current_state.selected() 
        {
            let next = (selected + 1) % self.clips.len(); 

            current_state.select(Some(next));
        }
    }

    fn previous(&mut self)
    {
        let current_state = &mut self.states[self.current_state];

        if let Some(selected) = current_state.selected() 
        {
            let prev = (selected + self.clips.len() - 1) % self.clips.len();

            current_state.select(Some(prev));
        }
    }
    
    fn draw(&mut self, frame: &mut ratatui::Frame) -> ()
    {
        use ratatui::widgets::{Block, Borders, List, ListItem};
        use ratatui::style::{Style, Color};

        let artist_items: Vec<ListItem> = self.clips
            .iter()
            .map(|artist| ListItem::new(artist.name.as_str()))
            .collect();

        let mut album_items: Vec<ListItem> = Vec::new();
        let mut song_items: Vec<ListItem> = Vec::new();

        if !artist_items.is_empty()
        {
            if let Some(i) = self.states[self.current_state].selected()
            {
                album_items = self.clips[i].albums.iter().map(|album| ListItem::new(album.get_name())).collect();
                song_items = self.clips[i].albums[0].songs.iter().map(|album| ListItem::new(album.get_name())).collect();
            }
            else 
            {
                album_items = self.clips[0].albums.iter().map(|album| ListItem::new(album.get_name())).collect();
                song_items = self.clips[0].albums[0].songs.iter().map(|album| ListItem::new(album.get_name())).collect();   
            }
        }

        let artist_list = List::new(artist_items)
            .block(Block::default().borders(Borders::ALL).title("Artists"))
            .style(Style::default().fg(Color::White))
            .highlight_symbol(">");
        
        let album_list = List::new(album_items)
            .block(Block::default().borders(Borders::ALL).title("Albums"))
            .style(Style::default().fg(Color::White))
            .highlight_symbol(">");
    
        let song_list = List::new(song_items)
            .block(Block::default().borders(Borders::ALL).title("Songs"))
            .style(Style::default().fg(Color::White))
            .highlight_symbol(">");
    
        let chunks = ratatui::layout::Layout::default()
            .direction(ratatui::layout::Direction::Horizontal)
            .constraints([
                ratatui::layout::Constraint::Percentage(33),
                ratatui::layout::Constraint::Percentage(33),
                ratatui::layout::Constraint::Percentage(34),
            ])
            .split(frame.area());


        frame.render_stateful_widget(artist_list, chunks[0], &mut self.states[0]);
        frame.render_stateful_widget(album_list, chunks[1], &mut self.states[1]);
        frame.render_stateful_widget(song_list, chunks[2], &mut self.states[2]);
    }
}

#[cfg(test)]
mod tests
{
    use super::*;
    use crossterm::event;

    #[test]
    fn test_key_event() -> Result<(), &'static str> 
    {
        let mut app = Application::new(Vec::default())?;

        app.handle_key_events(event::KeyCode::Char('q').into());
        assert_eq!(app.should_close, true);

        return Ok(());
    }
}