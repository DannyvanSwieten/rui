pub mod button;
pub mod center;
pub mod container;
pub mod flex;
pub mod flex_box;
pub mod list;
pub mod popup;
pub mod sized_box;
pub mod slider;
pub mod style;
pub mod text_editor;

mod child_slot;

use crate::application::{Application, ApplicationModel};
use crate::canvas::{Canvas2D, Point, Size};
use crate::constraints::BoxConstraints;
use crate::window::MouseEvent;
pub use child_slot::ChildSlot;
use popup::PopupRequest;
use style::Theme;
use winit::event::KeyboardInput;

pub fn map_range(x: f32, a: f32, b: f32, c: f32, d: f32) -> f32 {
    let slope = (d - c) / (b - a);
    c + slope * (x - a)
}

pub enum Action<Model> {
    None,
    Layout {
        nodes: Vec<&'static str>,
    },
    PopupRequest {
        request: PopupRequest<Model>,
        position: Point,
    },
    TriggerPopupMenu {
        menu: usize,
        sub_menu: usize,
    },
}

pub trait AppAction<Model> {
    fn undo(&self, _state: &mut Model);
    fn redo(&self, _state: &mut Model);
}

pub struct Properties {
    pub position: Point,
    pub size: Size,
}

#[allow(unused_variables)]
pub trait Widget<Model: ApplicationModel> {
    fn layout(&mut self, constraints: &BoxConstraints, model: &Model) -> Size;
    fn paint(&self, theme: &Theme, canvas: &mut dyn Canvas2D, rect: &Size, model: &Model);
    fn flex(&self) -> f32 {
        0.0
    }
    fn mouse_down(
        &mut self,
        event: &MouseEvent,
        properties: &Properties,
        application: &mut Application<Model>,
        model: &mut Model,
    ) {
    }
    fn mouse_up(&mut self, event: &MouseEvent, app: &mut Application<Model>, model: &mut Model);
    fn mouse_dragged(&mut self, event: &MouseEvent, properties: &Properties, model: &mut Model);
    fn mouse_moved(&mut self, event: &MouseEvent, model: &mut Model);
    fn mouse_entered(&mut self, event: &MouseEvent, model: &mut Model);
    fn mouse_left(&mut self, event: &MouseEvent, model: &mut Model);

    fn keyboard_event(&mut self, event: &KeyboardInput, model: &mut Model) -> bool {
        false
    }
    fn character_received(&mut self, character: char, model: &mut Model) -> bool {
        false
    }
}

// pub struct Label {
//     text: String,
//     font: Font,
//     paint: Paint,
// }

// impl Label {
//     pub fn new(text: &str) -> Self {
//         Label {
//             text: String::from(text),
//             paint: Paint::default(),
//             font: Font::default(),
//         }
//     }
// }

// impl<Model: ApplicationModel> Widget<Model> for Label {
//     fn paint(&mut self, _: &Model, rect: &Rect, canvas: &mut dyn Canvas2D, style: &StyleSheet) {
//         assert_ne!(rect.width(), 0.0);
//         assert_ne!(rect.height(), 0.0);
//         self.paint.set_color(*style.get("bg-color").unwrap());
//         self.paint.set_anti_alias(true);
//         canvas.draw_rounded_rect(rect, 15., 15., &self.paint);
//         self.paint.set_color(Color::WHITE);
//         canvas.draw_string(&self.text, &rect.center(), &self.font, &self.paint);
//     }

//     fn calculate_size(
//         &self,
//         preferred_width: Option<f32>,
//         preferred_height: Option<f32>,
//         constraints: &Constraints,
//         children: &[Node<Model>],
//     ) -> (Size, Vec<Constraints>) {
//         let w = if let Some(preferred_width) = preferred_width {
//             let width = constraints.min_width;
//             let width = width.max(preferred_width).min(constraints.max_width);
//             width
//         } else {
//             constraints.max_width
//         };

//         let h = if let Some(preferred_height) = preferred_height {
//             let height = constraints.min_height;
//             let height = height.max(preferred_height).min(constraints.max_height);
//             height
//         } else {
//             constraints.max_height
//         };

//         (Size::new(w, h), vec![Constraints::new(0.0, w, 0.0, h)])
//     }
// }

// pub trait TableDelegate<Model> {
//     fn row_selected(&mut self, id: usize, state: &mut Model) -> Action<Model>;
//     fn row_count(&self, state: &Model) -> usize;
// }

// pub struct Table<Model> {
//     paint: Paint,
//     delegate: Box<dyn TableDelegate<Model>>,
// }

// impl<Model> Table<Model> {
//     pub fn new<D>(delegate: D) -> Self
//     where
//         D: TableDelegate<Model> + 'static,
//     {
//         Table {
//             paint: Paint::default(),
//             delegate: Box::new(delegate),
//         }
//     }
// }

// impl<Model: ApplicationModel> Widget<Model> for Table<Model> {
//     fn paint(&mut self, state: &Model, rect: &Rect, canvas: &mut dyn Canvas2D, style: &StyleSheet) {
//         assert_ne!(rect.width(), 0.0);
//         assert_ne!(rect.height(), 0.0);
//         let e_color = *style.get("even").unwrap_or(&Color::CYAN);
//         let u_color = *style.get("uneven").unwrap_or(&Color::RED);

//         let row_count = self.delegate.row_count(state);
//         let height = rect.height() / row_count as f32;

//         for i in 0..row_count {
//             if i % 2 == 0 {
//                 self.paint.set_color(e_color);
//             } else {
//                 self.paint.set_color(u_color);
//             }

//             canvas.draw_rounded_rect(
//                 &Rect::from_point_and_size(
//                     (rect.left(), rect.top() + i as f32 * height),
//                     (rect.width(), height),
//                 ),
//                 1.,
//                 1.,
//                 &self.paint,
//             )
//         }
//     }

//     fn mouse_up(&mut self, state: &mut Model, rect: &Rect, event: &MouseEvent) -> Action<Model> {
//         let row_count = self.delegate.row_count(state);
//         let y = event.global_position().y - rect.top();
//         let row_size = rect.height() / row_count as f32;
//         let row = y / row_size;

//         self.delegate.row_selected(row as usize, state)
//     }

//     fn layout(
//         &mut self,
//         _state: &Model,
//         _rect: &Rect,
//         _spacing: f32,
//         _padding: f32,
//         _children: &mut [Node<Model>],
//     ) {
//     }

//     fn calculate_size(
//         &self,
//         preferred_width: Option<f32>,
//         preferred_height: Option<f32>,
//         constraints: &Constraints,
//         _children: &[Node<Model>],
//     ) -> (Size, Vec<Constraints>) {
//         let w = if let Some(preferred_width) = preferred_width {
//             let width = constraints.min_width;
//             let width = width.max(preferred_width).min(constraints.max_width);
//             width
//         } else {
//             constraints.max_width
//         };

//         let h = if let Some(preferred_height) = preferred_height {
//             let height = constraints.min_height;
//             let height = height.max(preferred_height).min(constraints.max_height);
//             height
//         } else {
//             constraints.max_height
//         };

//         (Size::new(w, h), vec![Constraints::new(0.0, w, 0.0, h)])
//     }
// }

// // pub struct Spinner<Model> {
// //     label: String,
// //     border_paint: Paint,
// //     bg_paint: Paint,
// //     fill_paint: Paint,
// //     text_paint: Paint,
// //     font: Font,
// //     min: Option<f32>,
// //     max: Option<f32>,
// //     step_size: f32,
// //     discrete: bool,
// //     current_value: f32,
// //     value_changed: Option<Box<dyn FnMut(f32, &mut Model)>>,
// // }

// // impl<Model> Spinner<Model> {
// //     pub fn new(
// //         label: &str,
// //         min: Option<f32>,
// //         max: Option<f32>,
// //         current_value: f32,
// //         discrete: bool,
// //     ) -> Self {
// //         let mut s = Spinner {
// //             label: String::from(label),
// //             border_paint: Paint::default(),
// //             bg_paint: Paint::default(),
// //             fill_paint: Paint::default(),
// //             text_paint: Paint::default(),
// //             font: Font::default(),
// //             min,
// //             max,
// //             discrete,
// //             step_size: 0.1,
// //             current_value,
// //             value_changed: None,
// //         };

// //         if discrete {
// //             s.step_size = 1.;
// //         }

// //         s
// //     }

// //     pub fn with_handler<F>(mut self, handler: F) -> Self
// //     where
// //         F: FnMut(f32, &mut Model) + 'static,
// //     {
// //         self.value_changed = Some(Box::new(handler));
// //         self
// //     }
// // }

// // impl<Model> Widget<Model> for Spinner<Model> {
// //     fn paint(&mut self, model: &mut Model, rect: &Rect, canvas: &mut dyn Canvas2D, style: &StyleSheet) {
// //         let bg_color = style.get("bg-color");
// //         let fill_color = style.get("fill-color");
// //         let border_color = style.get("border-color");

// //         self.bg_paint
// //             .set_color(bg_color.unwrap_or(&Color::new(1., 0., 0., 1.)));
// //         self.border_paint
// //             .set_color(border_color.unwrap_or(&Color::new(1., 0., 0., 1.)));
// //         self.border_paint.set_style(PaintStyle::Stroke);
// //         self.fill_paint
// //             .set_color(fill_color.unwrap_or(&Color::new(0.2, 0.2, 0.2, 1.)));
// //         self.text_paint.set_color(&Color::new(1., 1., 1., 1.));
// //         canvas.draw_rounded_rect(
// //             rect.left(),
// //             rect.bottom(),
// //             rect.width(),
// //             rect.height(),
// //             2.,
// //             2.,
// //             &self.bg_paint,
// //         );

// //         let t = self.label.to_string() + ": " + &format!("{:.4}", &self.current_value.to_string());
// //         canvas.draw_text(
// //             &t,
// //             rect,
// //             &self.text_paint,
// //             &self.font,
// //         );
// //     }

// //     fn mouse_dragged(&mut self, state: &mut Model, _: &Rect, event: &MouseEvent) {
// //         self.current_value += -event.delta_position.y * self.step_size;

// //         if self.discrete {
// //             self.current_value = self.current_value.round();
// //         }

// //         if let Some(m) = self.min {
// //             self.current_value = self.current_value.max(m);
// //         }

// //         if let Some(m) = self.max {
// //             self.current_value = self.current_value.min(m);
// //         }
// //         if let Some(l) = &mut self.value_changed {
// //             (l)(self.current_value, state);
// //         }
// //     }
// // }

// pub struct ViewPort {
//     scroll_bar_rect: Rect,
//     orientation: Orientation,
//     paint: Paint,
//     scroll_bar_paint: Paint,
//     scroll_bar_position: f32,
//     scroll_bar_ratio: f32,
// }

// impl ViewPort {
//     pub fn new(orientation: Orientation) -> Self {
//         ViewPort {
//             scroll_bar_rect: Rect::default(),
//             orientation,
//             paint: Paint::default(),
//             scroll_bar_paint: Paint::default(),
//             scroll_bar_position: 0.,
//             scroll_bar_ratio: 0.,
//         }
//     }
// }

// // impl<Model> Widget<Model> for ViewPort {
// //     fn layout(
// //         &mut self,
// //         _state: &Model,
// //         rect: &Rect,
// //         _spacing: f32,
// //         _padding: f32,
// //         children: &mut [Node<Model>],
// //     ) {
// //         assert_eq!(1, children.len());

// //         self.scroll_bar_rect = *rect;
// //         children[0].rect.set_wh(rect.size().width, rect.size().height);
// //         children[0].rect.set_wh(children[0].constraints.size(&rect.size()));

// //         match self.orientation {
// //             Orientation::Horizontal => {
// //                 self.scroll_bar_rect.bottom = 15.;
// //                 self.scroll_bar_rect.width() = rect.width();
// //                 self.scroll_bar_rect.bottom += rect.height() - 15.;

// //                 children[0].rect.height() = rect.height() - self.scroll_bar_rect.height();

// //                 self.scroll_bar_ratio = (rect.width() / children[0].rect.width()).min(1.).max(0.);
// //             }
// //             Orientation::Vertical => {
// //                 self.scroll_bar_rect.width() = 15.;

// //                 children[0].rect.left = 15.;
// //                 children[0].rect.width() = rect.width() - self.scroll_bar_rect.width();

// //                 self.scroll_bar_ratio = (rect.height() / children[0].rect.height()).min(1.).max(0.);
// //             }
// //         }
// //     }

// //     fn paint(
// //         &mut self,
// //         _state: &Model,
// //         rect: &Rect,
// //         canvas: &mut dyn Canvas2D,
// //         _style: &StyleSheet,
// //     ) {
// //         // self.paint.set_color(&Color::from((0., 0., 0.));
// //         // canvas.draw_rect(rect, &self.paint);

// //         // self.scroll_bar_paint
// //         //     .set_color(&Color::new(0.3, 0.3, 0.3, 1.));
// //         // canvas.draw_rect(self.scroll_bar_rect, &self.scroll_bar_paint);

// //         // self.scroll_bar_paint
// //         //     .set_color(&Color::new(0.2, 0.2, 0.2, 1.));

// //         // let r = Rect::from_xywh(
// //         //     self.scroll_bar_rect.left() + 1. + self.scroll_bar_position,
// //         //     self.scroll_bar_rect.bottom() + 1.,
// //         //     self.scroll_bar_rect.width(),
// //         //     self.scroll_bar_rect.height() * self.scroll_bar_ratio,
// //         // );

// //         // canvas.draw_rect(r, &self.scroll_bar_paint);
// //     }
// // }
