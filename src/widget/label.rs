use skia_safe::{Color, Paint, Size};

use crate::app::AppState;

use super::{Event, EventCtx, LayoutCtx, Widget};

pub struct Label<State> {
    text: String,
    text_provider: Option<Box<dyn Fn(&State) -> String>>,
}

impl<State: AppState> Label<State> {
    pub fn new(text: &str) -> Self {
        Self {
            text: text.to_string(),
            text_provider: None,
        }
    }

    pub fn with_provider<F: 'static>(mut self, provider: F) -> Self
    where
        F: Fn(&State) -> String,
    {
        self.text_provider = Some(Box::new(provider));
        self
    }
}

impl<State: AppState> Widget<State> for Label<State> {
    fn event(&mut self, _: &Event, _: &mut EventCtx<State::Message>, _: &State) -> bool {
        false
    }

    fn layout(
        &mut self,
        constraints: &crate::constraints::BoxConstraints,
        _: &mut LayoutCtx,
        state: &State,
    ) -> skia_safe::Size {
        assert!(constraints.max_width().is_some());
        assert!(constraints.max_height().is_some());

        if let Some(provider) = &self.text_provider {
            self.text = (*provider)(state)
        }

        Size::new(
            constraints.max_width().unwrap(),
            constraints.max_height().unwrap(),
        )
    }

    fn paint(
        &self,
        theme: &super::style::Theme,
        ctx: &super::PaintCtx,
        canvas: &mut dyn crate::canvas::Canvas2D,
        _: &State,
    ) {
        let mut paint = Paint::default();
        paint.set_anti_alias(true);
        paint.set_color(theme.label.text);
        paint.set_color(Color::from_rgb(0, 0, 0));
        canvas.draw_string(&ctx.rect(), &self.text, &theme.label.font, &paint)
    }
}
