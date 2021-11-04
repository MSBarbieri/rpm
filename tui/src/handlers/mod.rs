mod analysis;
mod error_screen;
mod help_menu;
mod home;
mod input;

use crate::{
    app::App,
    event::Key,
    router::{ActiveBlock, RouteId},
};

pub use input::handler as input_handler;

pub fn handle_app(key: Key, app: &mut App) {
    // First handle any global event and then move to block event
    match key {
        Key::Esc => {
            handle_escape(app);
        }
        _ if key == app.config.key_bindings.manage_devices => {
            // app.dispatch(IoEvent::GetDevices);
        }
        _ if key == app.config.key_bindings.seek_backwards => {
            // app.seek_backwards();
        }
        _ if key == app.config.key_bindings.seek_forwards => {
            // app.seek_forwards();
        }
        _ if key == app.config.key_bindings.help => {
            app.router
                .set_current_route_state(Some(ActiveBlock::HelpMenu), None);
        }
        _ if key == app.config.key_bindings.search => {
            app.router
                .set_current_route_state(Some(ActiveBlock::Input), Some(ActiveBlock::Input));
        }
        _ => handle_block_events(key, app),
    }
}

// Handle event for the current active block
fn handle_block_events(key: Key, app: &mut App) {
    let current_route = app.router.get_current_route();
    match current_route.active_block {
        ActiveBlock::Analysis => {
            analysis::handler(key, app);
        }
        ActiveBlock::Input => {
            // input::handler(key, app);
        }
        ActiveBlock::HelpMenu => {
            help_menu::handler(key, app);
        }
        ActiveBlock::Error => {
            error_screen::handler(key, app);
        }
        ActiveBlock::Home => {
            home::handler(key, app);
        }
        _ => {}
    }
}

fn handle_escape(app: &mut App) {
    match app.router.get_current_route().active_block {
        ActiveBlock::Error => {
            app.router.pop_navigation_stack();
        }
        ActiveBlock::Dialog(_) => {
            app.router.pop_navigation_stack();
        }
        // These are global views that have no active/inactive distinction so do nothing
        ActiveBlock::Analysis => {}
        _ => {
            app.router
                .set_current_route_state(Some(ActiveBlock::Empty), None);
        }
    }
}
