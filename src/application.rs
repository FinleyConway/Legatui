use ratatui;
use ratatui::widgets;

use crossterm::event;

#[derive(Default)]
pub struct Application
{
    should_close: bool,
}

impl Application
{
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
            _ => {}
        }
    }

    fn draw(&self, frame: &mut ratatui::Frame) -> ()
    {
        let text = widgets::Paragraph::new("Hello, World");

        frame.render_widget(text, frame.area());
    }

    fn close(&mut self) -> ()
    {
        self.should_close = true;
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
        let mut app = Application::default();
        app.handle_key_events(event::KeyCode::Char('q').into());
        assert_eq!(app.should_close, true);

        return Ok(());
    }
}