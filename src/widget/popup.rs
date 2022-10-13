use crate::{
    app::AppState,
    canvas::{Canvas2D, Size},
    constraints::BoxConstraints,
    widget::{Action, Properties, Theme, Widget},
    window::MouseEvent,
};

pub struct PopupMenu {
    id: usize,
    name: String,
    items: Vec<PopupMenu>,
}

struct PopupMenuWidget {
    active: bool,
    children: Vec<PopupMenuWidget>,
}

impl PopupMenuWidget {
    fn new(_request: PopupMenu) -> Self {
        PopupMenuWidget {
            active: true,
            children: Vec::new(),
        }
    }
}

impl<State: AppState> Widget<State> for PopupMenuWidget {
    fn layout(&mut self, constraints: &BoxConstraints, state: &State) -> Size {
        todo!()
    }

    fn paint(&self, theme: &Theme, canvas: &mut dyn Canvas2D, size: &Size, state: &State) {
        todo!()
    }
}

impl PopupMenu {
    pub fn new(id: usize, name: &str) -> Self {
        PopupMenu {
            id,
            name: name.to_string(),
            items: Vec::new(),
        }
    }

    pub fn with_item(mut self, id: usize, name: &str) -> Self {
        self.items.push(PopupMenu::new(id, name));
        self
    }

    pub fn with_sub_menu(mut self, sub_menu: PopupMenu) -> Self {
        self.items.push(sub_menu);
        self
    }

    fn has_sub_menu_items(&self) -> bool {
        !self.items.is_empty()
    }
}

pub struct PopupRequest<State> {
    menu: PopupMenu,
    pub handler: Box<dyn FnMut(usize, usize, &mut State) -> Action<State>>,
}

impl<State: AppState + 'static> PopupRequest<State> {
    pub fn new<F>(menu: PopupMenu, handler: F) -> Self
    where
        F: FnMut(usize, usize, &mut State) -> Action<State> + 'static,
    {
        PopupRequest {
            menu,
            handler: Box::new(handler),
        }
    }

    // pub fn build(&self) -> Box<dyn Widget<State>> {
    //     let mut b = Node::new("menu").widget(VStack::new()).spacing(1.);

    //     for item in self.menu.items.iter() {
    //         let s = item.id;
    //         b.add_child(
    //             Node::new("btn")
    //                 .widget(Button::new(&item.name))
    //                 .with_mouse_event_callback(MouseEventType::MouseUp, move |_, _| {
    //                     Action::TriggerPopupMenu {
    //                         menu: 0,
    //                         sub_menu: s,
    //                     }
    //                 }),
    //         );
    //     }

    //     b.rect.set_wh(150., self.menu.items.len() as f32 * 28.);
    //     b
    // }
}
