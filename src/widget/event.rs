use crate::window::MouseEvent;

pub enum Event {
    MouseUp(MouseEvent),
    MouseDown(MouseEvent),
}
