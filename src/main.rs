#![allow(unused)]

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

static FONT: FontFamily = FontFamily::Named("Source Code Pro");

fn text(text: impl Into<masonry::ArcStr>) -> Prose {
    Prose::new(text).with_font_family(FONT).with_text_size(40.)
}

fn main() {
    let label1 = text("Hello");
    let label2 = text("World!");

    let children = vec![
        panels::Child::new(Point::ORIGIN, text("Testing panel")),
        panels::Child::new(Point::new(50., 50.), text("another panel")),
        panels::Child::new(Point::new(100., 100.), text("and another panel")),
        panels::Child::new(
            Point::new(-100., 500.),
            Flex::column().with_child(Button::new("HI")),
        ),
    ];

    let main_widget = Split::columns(
        Flex::column()
            .with_child(label1)
            .with_spacer(20.0)
            .with_child(label2)
            .with_spacer(20.0)
            .with_child(CustomWidget("Haiii".into())),
        // Panel::new(Point::new(50., 50.), text("Testing panel").with_text_brush(brush)),
        Panel::new(children),
    )
    .draggable(true);

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
