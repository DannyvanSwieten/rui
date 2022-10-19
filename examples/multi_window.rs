use rui::{
    app::{App, AppRequest, AppState, MessageCtx, UIAppDelegate, WindowRequest},
    widget::{button::ButtonStyle, button::TextButton, container::Container, Widget},
};

struct State;

impl AppState for State {
    type Message = OpenWindow;

<<<<<<< Updated upstream
    fn handle_message(&mut self, _: Self::Message, ctx: &mut MessageCtx<Self>) {
        ctx.request(AppRequest::OpenWindow(WindowRequest::new(
            "Second window",
            600,
            400,
            |_| build_second_window(),
        )))
    }
=======
    fn handle_message(&self, _: &mut App<Self>, _: Self::MessageType) {}
>>>>>>> Stashed changes
}

#[derive(Clone)]
struct OpenWindow;

fn build_first_window() -> Box<dyn Widget<State>> {
    let button = TextButton::new("Open new window", 24.0).on_click(OpenWindow);
    Box::new(Container::new(button).with_padding(50.0))
}

fn build_second_window() -> Box<dyn Widget<State>> {
    Box::new(Container::new(TextButton::new("You opened a second window", 24.0)).with_padding(50.0))
}

fn main() {
    let app = App::new();

    let request = WindowRequest::new("Window 1", 600, 400, |_| build_first_window());
    let delegate = UIAppDelegate::new(request);

    app.run(delegate, State);
}
