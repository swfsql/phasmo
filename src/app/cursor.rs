use druid::{
    kurbo::Circle, theme, BoxConstraints, Data, Env, Event, EventCtx, LayoutCtx, LifeCycle,
    LifeCycleCtx, PaintCtx, Point, Size, UpdateCtx, Widget,
};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Cursor(Point);

impl<T: Data> Widget<T> for Cursor {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, _data: &mut T, _env: &Env) {
        if let Event::MouseMove(mouse_event) = event {
            self.0 = mouse_event.window_pos;
            ctx.request_layout();
        }
    }

    fn lifecycle(&mut self, _ctx: &mut LifeCycleCtx, _event: &LifeCycle, _data: &T, _env: &Env) {}

    fn update(&mut self, _ctx: &mut UpdateCtx, _old_data: &T, _data: &T, _env: &Env) {}

    fn layout(
        &mut self,
        _layout_ctx: &mut LayoutCtx,
        _bc: &BoxConstraints,
        _data: &T,
        _env: &Env,
    ) -> Size {
        Size::ZERO
    }

    fn paint(&mut self, ctx: &mut PaintCtx, _data: &T, env: &Env) {
        use druid::RenderContext;
        let radious = 5.0;
        let circle = Circle::new(self.0, radious);
        let fore = &env.get(theme::LABEL_COLOR);

        ctx.stroke(circle, fore, 1.0);
    }
}
