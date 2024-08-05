use accesskit::Role;
use masonry::{
    paint_scene_helpers::fill_color,
    paint_scene_helpers::stroke,
    vello::{peniko::BlendMode, Scene},
    AccessCtx, AccessEvent, Affine, BoxConstraints, Color, CursorIcon, EventCtx, LayoutCtx,
    LifeCycle, LifeCycleCtx, PaintCtx, Point, PointerEvent, PointerState, Rect, Size, StatusChange,
    TextEvent, Vec2, Widget, WidgetId, WidgetPod,
};
use smallvec::{smallvec, SmallVec};
use tracing::{trace_span, Span};
use winit::dpi::{LogicalPosition, PhysicalPosition};

const ZOOM_SENSITIVITY: f64 = 0.05;

enum DraggingState {
    NotDragging,
    Dragging {
        previous_screen_position: LogicalPosition<f64>,
    },
}

pub struct Canvas<W: Widget> {
    child: WidgetPod<W>,
    transform: Affine,
    dragging_state: DraggingState,
}

impl<W: Widget> Canvas<W> {
    pub fn new(child: W) -> Self {
        Self {
            child: WidgetPod::new(child),
            transform: Affine::IDENTITY,
            dragging_state: DraggingState::NotDragging,
        }
    }

    /// Where this point would be if the canvas had no transforms (used for passing to children)
    fn point_to_faux_point(&self, ctx: &EventCtx, position: Point) -> Point {
        let position = position - ctx.to_window(Point::ZERO);
        let position = Point::new(position.x, position.y);
        let position = self.transform.inverse() * position;
        let position = position.to_vec2() + ctx.to_window(Point::ZERO).to_vec2();
        position.to_point()
    }

    /// Where a screen point is in the local space of the canvas
    fn point_to_local_space(&self, ctx: &EventCtx, position: Point) -> Point {
        let position = position - ctx.to_window(Point::ZERO);
        let position = Point::new(position.x, position.y);
        let position = self.transform.inverse() * position;
        position
    }
}

impl<W: Widget> Widget for Canvas<W> {
    fn on_pointer_event(&mut self, ctx: &mut EventCtx, event: &PointerEvent) {
        match event {
            PointerEvent::PointerDown(masonry::PointerButton::Auxiliary, state) => {
                ctx.set_active(true);
                ctx.set_handled();
                ctx.set_cursor(&CursorIcon::Grabbing);

                self.dragging_state = DraggingState::Dragging {
                    previous_screen_position: state.position,
                };
            }
            PointerEvent::PointerUp(masonry::PointerButton::Auxiliary, state) => {
                self.dragging_state = DraggingState::NotDragging;

                if ctx.is_active() {
                    ctx.set_handled();
                    ctx.set_active(false);
                    ctx.clear_cursor();
                }
            }
            PointerEvent::PointerMove(state) => {
                if ctx.is_active() {
                    if let DraggingState::Dragging {
                        previous_screen_position: previous_position,
                    } = self.dragging_state
                    {
                        let delta = Vec2::new(
                            state.position.x - previous_position.x,
                            state.position.y - previous_position.y,
                        );

                        self.transform = self.transform.then_translate(delta);
                        self.dragging_state = DraggingState::Dragging {
                            previous_screen_position: state.position,
                        };

                        ctx.request_layout();
                        ctx.request_paint();
                    }
                }
            }
            PointerEvent::MouseWheel(delta, state) => {
                // TODO: this has error buildup because of the multiplication

                let scale_focus = self
                    .point_to_local_space(ctx, Point::new(state.position.x, state.position.y))
                    .to_vec2();

                self.transform = self.transform
                    * Affine::IDENTITY
                        .then_translate(-scale_focus)
                        .then_scale(1.0 + delta.y * ZOOM_SENSITIVITY)
                        .then_translate(scale_focus);

                ctx.request_paint();
            }
            _ => {}
        }

        let logical_position: Point = Point::new(
            event.pointer_state().position.x,
            event.pointer_state().position.y,
        );
        let physical_position: Point = Point::new(
            event.pointer_state().physical_position.x,
            event.pointer_state().physical_position.y,
        );

        let logical_position = self.point_to_faux_point(ctx, logical_position);
        let physical_position = self.point_to_faux_point(ctx, physical_position);

        let state = PointerState {
            physical_position: PhysicalPosition::new(physical_position.x, physical_position.y),
            position: LogicalPosition::new(logical_position.x, logical_position.y),
            // TODO: Don't clone here
            buttons: event.pointer_state().buttons.clone(),
            mods: event.pointer_state().mods,
            count: event.pointer_state().count,
            focus: event.pointer_state().focus,
        };

        let new_event = match event {
            PointerEvent::PointerDown(button, _) => PointerEvent::PointerDown(*button, state),
            PointerEvent::PointerUp(button, _) => PointerEvent::PointerUp(*button, state),
            PointerEvent::PointerMove(_) => PointerEvent::PointerMove(state),
            PointerEvent::PointerEnter(_) => PointerEvent::PointerEnter(state),
            PointerEvent::PointerLeave(_) => PointerEvent::PointerLeave(state),
            PointerEvent::MouseWheel(button, _) => PointerEvent::MouseWheel(*button, state),
            // TODO: Don't clone here
            PointerEvent::HoverFile(path_buf, _) => {
                PointerEvent::HoverFile(path_buf.clone(), state)
            }
            PointerEvent::DropFile(path_buf, _) => PointerEvent::DropFile(path_buf.clone(), state),
            PointerEvent::HoverFileCancel(_) => todo!(),
        };
        self.child.on_pointer_event(ctx, &new_event);
    }

    fn on_text_event(&mut self, ctx: &mut EventCtx, event: &TextEvent) {
        self.child.on_text_event(ctx, event);
    }

    fn on_access_event(&mut self, ctx: &mut EventCtx, event: &AccessEvent) {
        self.child.on_access_event(ctx, event);
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle) {
        self.child.lifecycle(ctx, event);
    }

    fn on_status_change(&mut self, _ctx: &mut LifeCycleCtx, _event: &StatusChange) {}

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints) -> Size {
        self.child.layout(ctx, &BoxConstraints::UNBOUNDED);
        ctx.place_child(&mut self.child, Point::ORIGIN);

        if bc.is_width_bounded() && bc.is_height_bounded() {
            bc.max()
        } else {
            let size = Size::new(100.0, 100.0);
            bc.constrain(size)
        }
    }

    fn paint(&mut self, ctx: &mut PaintCtx, parent_scene: &mut Scene) {
        let mut scene = Scene::new();

        let clip = Rect::from_origin_size(Point::ORIGIN, ctx.size());

        stroke(parent_scene, &clip, Color::RED, 1.0);

        // fill_color(parent_scene, &clip, Color::ORANGE);

        self.child.paint(ctx, &mut scene);

        parent_scene.push_layer(BlendMode::default(), 1.0, Affine::IDENTITY, &clip);
        parent_scene.append(&scene, Some(self.transform));
        parent_scene.pop_layer();
    }

    fn accessibility(&mut self, ctx: &mut AccessCtx) {
        self.child.accessibility(ctx);
    }

    fn accessibility_role(&self) -> Role {
        Role::GenericContainer
    }

    fn children_ids(&self) -> SmallVec<[WidgetId; 16]> {
        smallvec![self.child.id()]
    }

    fn make_trace_span(&self) -> Span {
        trace_span!("Canvas")
    }
}
