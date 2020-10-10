use druid::{
    BoxConstraints, Data, Env, Event, EventCtx, LayoutCtx, LifeCycle, LifeCycleCtx, PaintCtx, Size,
    UpdateCtx, Widget,
};

pub struct Overlay<B, F> {
    background: B,
    foreground: F,
}

impl<B, F> Overlay<B, F> {
    pub fn new(background: B, foreground: F) -> Self {
        Self {
            background,
            foreground,
        }
    }
}

impl<T: Data, B, F> Widget<T> for Overlay<B, F>
where
    B: Widget<T>,
    F: Widget<T>,
{
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut T, env: &Env) {
        self.background.event(ctx, event, data, env);
        self.foreground.event(ctx, event, data, env);
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &T, env: &Env) {
        self.background.lifecycle(ctx, event, data, env);
        self.foreground.lifecycle(ctx, event, data, env);
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &T, data: &T, env: &Env) {
        self.background.update(ctx, old_data, data, env);
        self.foreground.update(ctx, old_data, data, env);
    }

    fn layout(
        &mut self,
        layout_ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        data: &T,
        env: &Env,
    ) -> Size {
        let back_size = self.background.layout(layout_ctx, bc, data, env);
        let front_size = self.foreground.layout(layout_ctx, bc, data, env);

        let max_width = if back_size.width > front_size.width {
            back_size.width
        } else {
            front_size.width
        };
        let max_height = if back_size.height > front_size.height {
            back_size.height
        } else {
            front_size.height
        };

        Size::new(max_width, max_height)
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &T, env: &Env) {
        self.background.paint(ctx, data, env);
        self.foreground.paint(ctx, data, env);
    }
}
