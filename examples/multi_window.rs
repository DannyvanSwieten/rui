use rui::{
    app::{App, AppState, UIAppDelegate, WindowRequest},
    widget::{button::ButtonStyle, button::TextButton, container::Container, Widget},
};

struct State;

impl AppState for State {
    type Message = OpenWindow;

    fn handle_message(&self, _message: Self::Message) {
        //
    }
}

#[derive(Clone)]
struct OpenWindow;

fn build_first_window() -> Box<dyn Widget<State>> {
    Box::new(
        Container::new(TextButton::new("Open new window", 24.0).on_click(OpenWindow))
            .with_padding(50.0),
    )
}
fn build_second_window() -> Box<dyn Widget<State>> {
    Box::new(
        Container::new(
            TextButton::new("You opened a second window", 24.0).style(ButtonStyle::Outline),
        )
        .with_padding(50.0),
    )
}

fn main() {
    let app = App::new();

    let request = WindowRequest::new("Window 1", 600, 400, |_| build_first_window());
    let delegate = UIAppDelegate::new(request);

    app.run(delegate, State);
}
