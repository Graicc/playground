use accesskit::Role;
use masonry::{
    app_driver::{AppDriver, DriverCtx},
    kurbo::{BezPath, Stroke},
    paint_scene_helpers::{fill_color, stroke},
    parley::{
        self,
        style::{FontFamily, FontStack},
    },
    vello::{
        glyph::skrifa::raw::types::FixedSize,
        peniko::{Brush, Fill},
        Scene,
    },
    widget::{FillStrat, Label, RootWidget, *},
    AccessCtx, AccessEvent, Action, Affine, BoxConstraints, Color, EventCtx, LayoutCtx, LifeCycle,
    LifeCycleCtx, PaintCtx, Point, PointerEvent, Rect, Size, StatusChange, TextEvent, Vec2, Widget,
    WidgetId,
};
use parley::layout::Alignment;
use parley::style::StyleProperty;
use smallvec::SmallVec;
use tracing::{trace_span, Span};
use winit::{
    dpi::{LogicalPosition, LogicalSize},
    window::Window,
};

const MAX_SIZE: masonry::Size = Size::new(400.0, 4000.0);

pub struct Child {
    pub position: Point,
    pub widget: WidgetPod<Box<dyn Widget>>,
    pub background_color: Color,
}

impl Child {
    pub fn new(position: Point, widget: impl Widget + 'static) -> Self {
        Self {
            position,
            widget: WidgetPod::new(widget).boxed(),
            background_color: Color::GRAY,
        }
    }

    fn overlap(&self, position: Point) -> bool {
        let local_space = self.to_local_space(position);
        local_space.x > 0.0
            && local_space.x < MAX_SIZE.width
            && local_space.y > 0.0
            && local_space.y < MAX_SIZE.height
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
}

// If this widget has any child widgets it should call its event, update and layout
// (and lifecycle) methods as well to make sure it works. Some things can be filtered,
// but a general rule is to just pass it through unless you really know you don't want it.
impl Widget for Panel {
    fn on_pointer_event(&mut self, ctx: &mut EventCtx, event: &PointerEvent) {
        match event {
            PointerEvent::PointerDown(masonry::PointerButton::Secondary, state) => {
                let position = Point::new(state.position.x, state.position.y);
                let position = position - ctx.to_window(Point::ZERO);
                let position = Point::new(position.x, position.y);
                println!("{position:?}");

                if let Some((i, child)) = self
                    .children
                    .iter()
                    .enumerate()
                    .find(|(_, c)| c.overlap(position))
                {
                    ctx.set_active(true);
                    ctx.set_handled();
                    println!("CHILD FOUND");

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
                    let position = Point::new(state.position.x, state.position.y);
                    let position = position - ctx.to_window(Point::ZERO);
                    let position = Point::new(position.x, position.y);

                    if let DraggingState::Dragging { offset, child } = self.dragging_state {
                        let mut new_position =
                            Point::new(position.x - offset.x, position.y - offset.y);

                        new_position.x = new_position
                            .x
                            .clamp(0.0, (ctx.size().width - MAX_SIZE.width).max(0.0));
                        new_position.y = new_position
                            .y
                            .clamp(0.0, (ctx.size().height - MAX_SIZE.height).max(0.0));

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
        let child_bc = BoxConstraints::new(Size::new(0.0, 0.0), MAX_SIZE.into());
        for child in &mut self.children {
            child.widget.layout(ctx, &child_bc);
            ctx.place_child(&mut child.widget, child.position);
        }

        if bc.is_width_bounded() && bc.is_height_bounded() {
            bc.max()
        } else {
            let size = Size::new(100.0, 100.0);
            bc.constrain(size)
        }
    }

    fn paint(&mut self, ctx: &mut PaintCtx, scene: &mut Scene) {
        for child in self.children.iter_mut().rev() {
            let path = Rect::from_origin_size(child.position, MAX_SIZE).inflate(10., 10.);

            stroke(scene, &path, Color::WHITE, 10.0);

            fill_color(scene, &path, child.background_color);
            child.widget.paint(ctx, scene);
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
