//! Screen rendering coordination
//!
//! This module handles the coordination of screen rendering for different
//! application states (List, Show, Dialog, etc.).

use std::io::Write;
use crate::status::Status;
use crate::App;

/// Print the appropriate screen based on current application state
///
/// # Arguments
/// * `app` - Mutable reference to the application state
///
/// # Renders
/// - Index view when in List status
/// - Thread view when in Show status
/// - Dialog views when in Dialog or ChannelDialog status
pub fn print_screen(app: &mut App) {
    match app.state_manager.get_state() {
        Status::Startup => {}
        Status::List => {
            app.index.print(&mut app.stdout, &app.list_topic_items);
        }
        Status::Show => {
            app.show.print(&mut app.stdout, &app.show_item);
        }
        Status::Dialog => {
            // Print the underlying screen (List or Show)
            match app.prev_state {
                Status::List => {
                    app.index.print(&mut app.stdout, &app.list_topic_items);
                }
                Status::Show => {
                    app.show.print(&mut app.stdout, &app.show_item);
                }
                _ => {}
            }
            // Print dialog on top
            app.dialog.print(&mut app.stdout);
        }
        Status::ChannelDialog => {
            // Print the underlying screen (List or Show)
            match app.prev_state {
                Status::List => {
                    app.index.print(&mut app.stdout, &app.list_topic_items);
                }
                Status::Show => {
                    app.show.print(&mut app.stdout, &app.show_item);
                }
                _ => {}
            }
            // Print channel dialog on top
            app.channel_dialog.print(&mut app.stdout);
        }
        Status::PageDialog => {
            // Print the underlying screen (should only be Show)
            match app.prev_state {
                Status::Show => {
                    app.show.print(&mut app.stdout, &app.show_item);
                }
                _ => {}
            }
            // Print page dialog on top
            app.page_dialog.print(&mut app.stdout);
        }
    }

    app.status_bar.print(&app.screen_manager);

    if let Err(e) = app.stdout.flush() {
        log::error!("Failed to flush stdout: {}", e);
    }
}
