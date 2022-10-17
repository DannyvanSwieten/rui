pub trait AppState {
    type MessageType;

    fn handle_message(&self, msg: Self::MessageType);
}
