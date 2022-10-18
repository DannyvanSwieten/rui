use rui::{
    app::{App, AppState, UIAppDelegate, WindowRequest},
    widget::{button::ButtonStyle, button::TextButton, container::Container, Widget},
};

struct State;

impl AppState for State {
    type MessageType = ();

    fn handle_message(&self, _: Self::MessageType) {}
}

fn build_first_window() -> Box<dyn Widget<State>> {
    Box::new(
        Container::new(TextButton::new("Open new window", 24.0).on_click(|ctx| {
            // ctx.request_ui_window(WindowRequest::new("Second Window", 600, 400, |_| {
            //     build_second_window()
            // }));
        }))
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

    let delegate = UIAppDelegate::new().on_start(|app| {
        app.request(rui::app::AppRequest::OpenWindowRequest(WindowRequest::new(
            "Window 1",
            600,
            400,
            |_| build_first_window(),
        )));
    });

    app.run(delegate, State);
}
