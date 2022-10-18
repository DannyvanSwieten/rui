use rui::{
    app::{App, AppState, MessageCtx, UIAppDelegate, WindowRequest},
    widget::{
        button::{ButtonStyle, TextButton},
        center::Center,
        container::Container,
        flex::Flex,
        Widget,
    },
};

struct State;

impl AppState for State {
    type Message = ();

    fn handle_message(&self, _: Self::Message, _: &mut MessageCtx<Self>) {}
}

fn main() {
    let app = App::new();

    let request = WindowRequest::new("Solfege", 600, 400, |_state| {
        Box::new(Center::new(
            Flex::row()
                .with_spacing(5.0)
                .push(button("Do"))
                .push(button("Re"))
                .push(button("Mi"))
                .push(button("Fa"))
                .push(button("So"))
                .push(button("La"))
                .push(button("Ti")),
        ))
    });
    let delegate = UIAppDelegate::new(request);

    app.run(delegate, State);
}

fn button(name: &str) -> impl Widget<State> {
    Container::new(TextButton::new(name, 24.0).style(ButtonStyle::Outline))
}
