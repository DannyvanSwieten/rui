use super::{App, AppRequest};

pub trait AppState: Sized {
    type Message: Clone;
    type Response: Clone;

    fn handle_message(&mut self, msg: Self::Message, ctx: &mut MessageCtx<Self>) -> Self::Response;
}

pub struct MessageCtx<'a, State: AppState> {
    app: &'a mut App<State>,
}

impl<'a, State: AppState> MessageCtx<'a, State> {
    pub fn new(app: &'a mut App<State>) -> Self {
        Self { app }
    }
}

impl<'a, State: AppState + 'static> MessageCtx<'a, State> {
    pub fn request(&mut self, request: AppRequest<State>) {
        self.app.request(request)
    }
}
