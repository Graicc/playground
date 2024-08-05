use accesskit::Role;
use masonry::{
    paint_scene_helpers::{fill_color, stroke},
    vello::{peniko::BlendMode, Scene},
    widget::*,
    AccessCtx, AccessEvent, Affine, BoxConstraints, Color, EventCtx, LayoutCtx, LifeCycle,
    LifeCycleCtx, PaintCtx, Point, PointerEvent, Rect, Size, StatusChange, TextEvent, Widget,
    WidgetId,
};
use smallvec::SmallVec;
use tracing::{trace_span, Span};
use winit::dpi::LogicalPosition;

const ZOOM_SENSITIVITY: f64 = 0.05;

pub struct Child {
    pub position: Point,
    pub size: Size,
    pub widget: WidgetPod<Box<dyn Widget>>,
    pub background_color: Color,
}

impl Child {
    pub fn new(position: Point, widget: impl Widget + 'static) -> Self {
        Self {
            position,
            widget: WidgetPod::new(widget).boxed(),
            background_color: Color::parse("#1F1F1F").unwrap(),
            size: Size::ZERO,
        }
    }

    fn overlap(&self, position: Point) -> bool {
        let local_space = self.to_local_space(position);
        local_space.x > 0.0
            && local_space.x < self.size.width
            && local_space.y > 0.0
            && local_space.y < self.size.height
    }

    fn to_local_space(&self, position: Point) -> Point {
        let local_space = position - self.position;
        Point::new(local_space.x, local_space.y)
    }
}

enum DraggingState {
    NotDragging,
    Dragging { offset: Point, child: usize },
}

pub struct Panel {
    pub children: Vec<Child>,
    dragging_state: DraggingState,
}

impl Panel {
    pub fn new(children: Vec<Child>) -> Self {
        Self {
            children,
            dragging_state: DraggingState::NotDragging,
        }
    }

    fn logical_position_to_point(&self, ctx: &EventCtx, position: LogicalPosition<f64>) -> Point {
        let position = Point::new(position.x, position.y);
        let position = position - ctx.to_window(Point::ZERO);
        let position = Point::new(position.x, position.y);
        return position;
    }
}

// If this widget has any child widgets it should call its event, update and layout
// (and lifecycle) methods as well to make sure it works. Some things can be filtered,
// but a general rule is to just pass it through unless you really know you don't want it.
impl Widget for Panel {
    fn on_pointer_event(&mut self, ctx: &mut EventCtx, event: &PointerEvent) {
        match event {
            PointerEvent::PointerDown(masonry::PointerButton::Secondary, state) => {
                println!("Event");
                let position = self.logical_position_to_point(ctx, state.position);

                if let Some((i, child)) = self
                    .children
                    .iter()
                    .enumerate()
                    .find(|(_, c)| c.overlap(position))
                {
                    ctx.set_active(true);
                    ctx.set_handled();

                    let offset = child.to_local_space(position);
                    self.dragging_state = DraggingState::Dragging { offset, child: i }
                }
                // ctx.request_layout();
            }
            PointerEvent::PointerUp(masonry::PointerButton::Secondary, state) => {
                self.dragging_state = DraggingState::NotDragging;

                if ctx.is_active() {
                    ctx.set_handled();
                    ctx.set_active(false);
                }
            }
            PointerEvent::PointerMove(state) => {
                if ctx.is_active() {
                    let position = self.logical_position_to_point(ctx, state.position);

                    if let DraggingState::Dragging { offset, child } = self.dragging_state {
                        let mut new_position =
                            Point::new(position.x - offset.x, position.y - offset.y);

                        new_position.x = new_position
                            .x
                            .clamp(0.0, (ctx.size().width - self.children[child].size.width).max(0.0));
                        new_position.y = new_position
                            .y
                            .clamp(0.0, (ctx.size().height - self.children[child].size.height).max(0.0));

                        // println!("{position:?}");
                        self.children[child].position = new_position;

                        ctx.request_layout();
                        ctx.request_paint();
                    }
                }
            }
            _ => {}
        }

        for child in &mut self.children {
            child.widget.on_pointer_event(ctx, event);
        }
    }

    fn on_text_event(&mut self, ctx: &mut EventCtx, event: &TextEvent) {
        for child in &mut self.children {
            child.widget.on_text_event(ctx, event);
        }
    }

    fn on_access_event(&mut self, ctx: &mut EventCtx, event: &AccessEvent) {
        for child in &mut self.children {
            child.widget.on_access_event(ctx, event);
        }
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle) {
        for child in &mut self.children {
            child.widget.lifecycle(ctx, event);
        }
    }

    fn on_status_change(&mut self, _ctx: &mut LifeCycleCtx, _event: &StatusChange) {}

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints) -> Size {
        for child in &mut self.children {
            child.size = child.widget.layout(ctx, &BoxConstraints::UNBOUNDED);
            ctx.place_child(&mut child.widget, child.position);
        }

        if bc.is_width_bounded() && bc.is_height_bounded() {
            bc.max()
        } else {
            let size = Size::new(10_000.0, 10_000.0);
            bc.constrain(size)
        }
    }

    fn paint(&mut self, ctx: &mut PaintCtx, scene: &mut Scene) {
        for child in self.children.iter_mut().rev() {
            let path = Rect::from_origin_size(child.position, child.size).inflate(10., 10.);

            stroke(scene, &path, Color::WHITE, 10.0);

            fill_color(scene, &path, child.background_color);

            scene.push_layer(BlendMode::default(), 1.0, Affine::IDENTITY, &path);
            child.widget.paint(ctx, scene);
            scene.pop_layer();
        }

        for slice in self.children.windows(2) {
            if let [child1, child2] = slice {
                let path = masonry::kurbo::Line::new(child1.position, child2.position);
                stroke(scene, &path, Color::WHITE, 2.0);
            }
        }
    }

    fn accessibility(&mut self, ctx: &mut AccessCtx) {
        for child in &mut self.children {
            child.widget.accessibility(ctx);
        }
    }

    fn accessibility_role(&self) -> Role {
        Role::Window
    }

    fn children_ids(&self) -> SmallVec<[WidgetId; 16]> {
        SmallVec::new()
    }

    fn make_trace_span(&self) -> Span {
        trace_span!("Panel")
    }
}
