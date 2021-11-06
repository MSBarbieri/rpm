use crate::{
    app::App,
    event::Key,
    router::{ActiveBlock, RouteId},
};

pub fn handle_app(key: Key, app: &mut App) {
    // First handle any global event and then move to block event
    match key {
        Key::Esc => {
            handle_escape(app);
        }
        Key::Char('?') => {
            app.router
                .push_navigation_stack(RouteId::Help, ActiveBlock::HelpMenu);
        }
        Key::Char('q') => {
            app.router.pop_navigation_stack();
        }
        _ => handle_block_events(key, app),
    }
}

fn handle_block_events(key: Key, app: &mut App) {
    let current_route = app.router.get_current_route();
    match current_route.active_block {
        // ActiveBlock::HelpMenu => {
        //     help_menu::handler(key, app);
        // }
        // ActiveBlock::Home => {
        //     home::handler(key, app);
        // }
        _ => {}
    }
}

fn handle_escape(app: &mut App) {
    match app.router.get_current_route().active_block {
        ActiveBlock::HelpMenu => {
            app.router.pop_navigation_stack();
        }
        _ => {
            app.router
                .set_current_route_state(Some(ActiveBlock::Empty), None);
        }
    }
}
