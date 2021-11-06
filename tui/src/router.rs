
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum ActiveBlock {
    Home,
    Empty,
    HelpMenu,
}

#[derive(Clone, PartialEq, Debug)]
pub enum RouteId {
    Home,
    Help
}

#[derive(Clone,Debug)]
pub struct Route {
    pub id: RouteId,
    pub active_block: ActiveBlock,
    pub hovered_block: ActiveBlock,
}

impl Default for Route {
    fn default() -> Route {
        Route {
            id: RouteId::Home,
            active_block: ActiveBlock::Empty,
            hovered_block: ActiveBlock::Empty,
        }
    }
}

impl Route {
    fn new() -> Route {
        Route::default()
    }
}

#[derive(Clone,Debug)]
pub struct Router {
    navigation_stack: Vec<Route>
}

impl Router {
    pub fn new() -> Router {
        Router {
            navigation_stack: vec![Route::new()],
        }
    }

    pub fn push_navigation_stack(
        &mut self,
        next_route_id: RouteId,
        next_active_block: ActiveBlock,
    ) {
        if !self
            .navigation_stack
            .last()
            .map(|last_route| last_route.id == next_route_id)
            .unwrap_or(false)
        {
            self.navigation_stack.push(Route {
                id: next_route_id,
                active_block: next_active_block,
                hovered_block: next_active_block,
            });
        }
    }

    pub fn pop_navigation_stack(&mut self) -> Option<Route> {
        if self.navigation_stack.len() == 1 {
            None
        } else {
            self.navigation_stack.pop()
        }
    }

    pub fn get_current_route(&self) -> &Route {
        self.navigation_stack.last().unwrap()
    }

    fn get_current_route_mut(&mut self) -> &mut Route {
        self.navigation_stack.last_mut().unwrap()
    }

    pub fn set_current_route_state(
        &mut self,
        active_block: Option<ActiveBlock>,
        hovered_block: Option<ActiveBlock>,
    ) {
        let mut current_route = self.get_current_route_mut();
        if let Some(active_block) = active_block {
            current_route.active_block = active_block;
        }
        if let Some(hovered_block) = hovered_block {
            current_route.hovered_block = hovered_block;
        }
    }
}
