use crossterm::event::{read, Event, KeyCode, KeyEvent, KeyModifiers};
use std::io;
use std::io::Write;
use tui::backend::CrosstermBackend;
use tui::layout::{Alignment, Constraint, Direction, Layout};
use tui::style::{Color, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders, List, ListItem, Paragraph, Tabs};
use tui::{widgets, Terminal};

pub const LOGO: &str = r#"
         r r r         
     rrstssrsstsr      
   rsurr   r   rrusr   
  rtuuuuuuuuuutr  utr  
  srrrruusrrrsuurrrss  
 rts  ruutsssuur   ttr 
  surrsuusrr suurrtus  
  rtusrrrrrr  rrrsutr  
   rsurr      rrrusr   
      rsstssstssr      
         r r r         
"#;

fn main() -> Result<(), anyhow::Error> {
    let stdout = io::stdout();
    crossterm::terminal::enable_raw_mode().unwrap();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let mut stdout = io::stdout();
    terminal.clear().unwrap();

    let mut search_block_title = "".to_string();
    let mut search_block_text = LOGO.to_string();
    let mut results_block_label = "Results".to_string();
    let mut results_current_tab = 0;

    let mut search_block_border_style = Style::default().fg(tui::style::Color::DarkGray);
    let mut results_block_border_style = Style::default().fg(tui::style::Color::DarkGray);
    let mut results_block_highlight_style = Style::default().bg(tui::style::Color::DarkGray);

    // support basic vim shortcuts like `gg` along with num prefixes
    let mut last_key = KeyEvent {
        code: KeyCode::Null,
        modifiers: KeyModifiers::NONE,
    };
    let mut previous_key = KeyEvent {
        code: KeyCode::Null,
        modifiers: KeyModifiers::NONE,
    };

    loop {
        // draw the interface
        terminal
            .draw(|f| {
                let chunks_horiz = Layout::default()
                    .direction(Direction::Horizontal)
                    .margin(2)
                    .constraints(
                        [
                            Constraint::Length(26),
                            Constraint::Max(4),
                            Constraint::Percentage(50),
                        ]
                        .as_ref(),
                    )
                    .split(f.size());
                let chunks_left = Layout::default()
                    .direction(Direction::Vertical)
                    // .margin(1)
                    .constraints([Constraint::Length(14), Constraint::Min(10)].as_ref())
                    .split(chunks_horiz[0]);
                let chunks_right = Layout::default()
                    .direction(Direction::Vertical)
                    // .margin(1)
                    .constraints([Constraint::Length(3), Constraint::Min(10)].as_ref())
                    .split(chunks_horiz[2]);
                let search_block = Block::default()
                    .title(search_block_title.as_str())
                    .borders(Borders::RIGHT)
                    .border_style(search_block_border_style);
                f.render_widget(search_block, chunks_horiz[0]);
                let paragraph = Paragraph::new(search_block_text.as_str());
                f.render_widget(paragraph, chunks_left[0]);

                let paragraph =
                    Paragraph::new("Crate termimad\n\nVersion 0.1.0").alignment(Alignment::Center);
                f.render_widget(paragraph, chunks_left[1]);

                let results_block = Block::default()
                    .title(results_block_label.as_str())
                    .borders(Borders::ALL)
                    .border_style(results_block_border_style);

                let top_bar = Block::default().title("").borders(Borders::BOTTOM);
                let spans = Spans::from(vec![
                    Span::styled("My", Style::default().fg(Color::Yellow)),
                    Span::raw(" text"),
                ]);
                let titles = ["Summary", "Readme", "Repository", "Stats"]
                    .iter()
                    .cloned()
                    .map(Spans::from)
                    .collect();
                let top_tabs = Tabs::new(titles)
                    .select(results_current_tab)
                    .block(Block::default().title("").borders(Borders::BOTTOM))
                    .style(Style::default().fg(Color::DarkGray))
                    .highlight_style(Style::default().fg(Color::White));
                // .divider(tui::symbols::);
                f.render_widget(top_tabs, chunks_right[0]);

                let details = widgets::Paragraph::new("")
                    // .style(Style::default().add_modifier())
                    // .wrap(tui::widgets::Wrap::)
                    .block(Block::default().borders(Borders::NONE));
                f.render_widget(details, chunks_right[1]);
            })
            .unwrap();

        if let Event::Key(key_event) = read().unwrap() {
            previous_key = last_key;
            last_key = key_event.clone();
            if let KeyEvent {
                code: kc,
                modifiers: mods,
            } = key_event
            {
                if mods == KeyModifiers::CONTROL {
                    match kc {
                        // KeyCode::Char('h') => show_intro = !show_intro,
                        KeyCode::Char('q') => break,
                        // KeyCode::Char('s') => current_mode = Mode::Search,
                        _ => continue,
                    }
                } else {
                    match kc {
                        KeyCode::Left | KeyCode::Char('h') => {
                            if results_current_tab > 0 {
                                results_current_tab -= 1
                            }
                        }
                        KeyCode::Right | KeyCode::Char('l') => {
                            if results_current_tab < 3 {
                                results_current_tab += 1
                            }
                        }
                        // KeyCode::Up | KeyCode::Char('k') => match num_input {
                        //     None => crates.select_previous(None),
                        //     Some(n) => crates.select_previous(Some(n as usize)),
                        // },
                        // KeyCode::Down | KeyCode::Char('j') => match num_input {
                        //     None => crates.select_next(None),
                        //     Some(n) => crates.select_next(Some(n as usize)),
                        // },
                        KeyCode::Enter => {
                            // if let Some(selected_crate) = crates.state.selected() {
                            //     webbrowser::open(&format!(
                            //         "https://crates.io/crates/{}",
                            //         crates.items.get(selected_crate).unwrap().id
                            //     ));
                            // }
                        }
                        KeyCode::Char(ch) => match ch {
                            '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                                // match num_input {
                                //     None => num_input = ch.to_digit(10),
                                //     Some(n) => num_input = Some(n * 10 + ch.to_digit(10).unwrap()),
                                // }
                                continue;
                            }
                            'g' => {
                                if let KeyCode::Char('g') = last_key.code {
                                    if let KeyCode::Char('g') = previous_key.code {
                                        // crates.select(Some(0));
                                    }
                                }
                            }
                            // 'G' => crates.select(Some(crates.items.len() - 1)),
                            'q' => break,
                            _ => (),
                        },

                        _ => (),
                    }
                }
            }
        }
        // reset combos

        stdout.flush().unwrap();
    }

    terminal.clear().unwrap();
    Ok(())
}
