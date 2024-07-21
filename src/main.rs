#![allow(unused)]

use std::io::BufReader;

use accesskit::Role;
use masonry::{
    app_driver::{AppDriver, DriverCtx},
    kurbo::{BezPath, Stroke},
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
    LifeCycleCtx, PaintCtx, Point, PointerEvent, Rect, Size, StatusChange, TextEvent, Widget,
    WidgetId,
};
use parley::layout::Alignment;
use parley::style::StyleProperty;
use smallvec::SmallVec;
use tracing::{trace_span, Span};
use winit::{dpi::LogicalSize, window::Window};

mod widget;
use widget::*;

struct Driver;

impl AppDriver for Driver {
    fn on_action(&mut self, ctx: &mut DriverCtx<'_>, widget_id: WidgetId, action: Action) {
        match action {
            Action::ButtonPressed(_) => {
                println!("Hello!");
            }
            action => {
                todo!();
            }
        }
    }
}

fn text(text: impl Into<masonry::ArcStr>) -> CodeBlock {
    let mut code = CodeBlock::new(text);
    code

    // let mut prose = Prose::new(text).with_font(FONT).with_text_size(20.);
    // let mut prose_mut: WidgetMut<'_, Prose> = prose.;
    // WidgetMut::from(prose).set_text_properties(|layout| {
    //     layout.rebuild_with_attributes();
    // });
    // prose
}

fn main() {
    let label1 = text("Hello");
    let label2 = text("World!");

    let file_contents = std::fs::read_to_string("src/main.rs").unwrap();

    let children = vec![
        panels::Child::new(Point::ORIGIN, text("Testing panel asdf \n another line")),
        panels::Child::new(Point::new(50., 50.), text(file_contents)),
        panels::Child::new(Point::new(100., 100.), text("and another panel")),
        panels::Child::new(
            Point::new(-100., 500.),
            Flex::column().with_child(Button::new("HI")),
        ),
    ];

    let main_widget = Panel::new(children);

    let window_attributes = Window::default_attributes()
        .with_title("Playground")
        .with_resizable(true)
        .with_min_inner_size(LogicalSize::new(400.0, 400.0));

    masonry::event_loop_runner::run(
        masonry::event_loop_runner::EventLoop::with_user_event(),
        window_attributes,
        RootWidget::new(main_widget),
        Driver,
    )
    .unwrap();
}
