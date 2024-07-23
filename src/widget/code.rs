use masonry::{
    parley::{
        style::{FontStack, StyleProperty},
        FontContext, LayoutContext,
    },
    text::TextLayout,
    vello::{peniko::BlendMode, Scene},
    AccessCtx, AccessEvent, Affine, BoxConstraints, Color, EventCtx, LayoutCtx, LifeCycle,
    LifeCycleCtx, PaintCtx, Point, PointerEvent, Size, StatusChange, TextEvent, Widget, WidgetId,
};
use smallvec::SmallVec;
use std::sync::Arc;
use tracing::trace;
use tree_sitter::Parser;
use tree_sitter_highlight::HighlightConfiguration;
use accesskit::Role;

use super::colors::get_colors;

// From label.rs
#[derive(Debug, Clone, Copy, PartialEq)]
enum LineBreaking {
    /// Lines are broken at word boundaries.
    WordWrap,
    /// Lines are truncated to the width of the label.
    Clip,
    /// Lines overflow the label.
    Overflow,
}

const LABEL_X_PADDING: f64 = 2.0;

pub struct CodeBlock {
    text_layout: TextLayout<Arc<str>>,
    line_break_mode: LineBreaking,
}

static FONT: FontStack = FontStack::Source("Source Code Pro");
const FONT_SIZE: f32 = 14.0;

impl CodeBlock {
    pub fn new(text: impl Into<Arc<str>>) -> Self {
        let text = text.into();

        let mut parser = Parser::new();

        let language = tree_sitter_rust::language();
        parser
            .set_language(&language)
            .expect("Error loading Rust grammar");
        let mut highlighter = tree_sitter_highlight::Highlighter::new();
        let highlight_config = get_colors();

        let mut rust_config = HighlightConfiguration::new(
            language,
            "rust",
            tree_sitter_rust::HIGHLIGHTS_QUERY,
            tree_sitter_rust::INJECTIONS_QUERY,
            "",
        )
        .unwrap();

        let names: Vec<_> = highlight_config.iter().map(|x| x.0).collect();
        rust_config.configure(&names);

        let highlights = highlighter
            .highlight(&rust_config, text.as_bytes(), None, |_| None)
            .unwrap();

        let tree = parser.parse(text.clone().as_ref(), None).unwrap();

        // let fn_query = tree_sitter::Query::new(
        //     &language,
        //     "(source_file (function_item name: (identifier) @function_name))",
        // )
        // .unwrap();
        // let mut query_cursor = tree_sitter::QueryCursor::new();
        // let fns = query_cursor
        //     .captures(&fn_query, tree.root_node(), text.as_bytes())
        //     .map(|(m, _)| m.captures[0].node);

        let mut text_layout = TextLayout::new(text.clone(), FONT_SIZE);
        text_layout.set_font(FONT);

        // let colors = &[Color::RED, Color::BLUE];
        //
        // let binding = text.clone();
        // let split_points = binding.char_indices().filter_map(|(i, c)| {
        //     if c.is_ascii_whitespace() {
        //         Some(i)
        //     } else {
        //         None
        //     }
        // });

        text_layout.rebuild_with_attributes(
            &mut FontContext::default(),
            &mut LayoutContext::default(),
            |mut x| {
                // for (count, (l, r)) in split_points.tuple_windows().enumerate() {
                //     x.push(
                //         &StyleProperty::Brush(colors[count % colors.len()].into()),
                //         l..=r,
                //     );
                // }

                let mut color = Color::WHITE;
                for h in highlights {
                    match h.unwrap() {
                        tree_sitter_highlight::HighlightEvent::Source { start, end } => {
                            x.push(&StyleProperty::Brush(color.into()), start..end);
                        }
                        tree_sitter_highlight::HighlightEvent::HighlightStart(s) => {
                            color = highlight_config[s.0].1;
                        }
                        tree_sitter_highlight::HighlightEvent::HighlightEnd => {
                            color = Color::WHITE;
                        }
                    }
                }

                // for a in fns {
                //     x.push(
                //         &StyleProperty::Brush(Color::GREEN.into()),
                //         a.start_byte()..a.end_byte(),
                //     );
                // }

                // x.push(&StyleProperty::FontSize(40.0), 2..=4);
                x
            },
        );

        Self {
            text_layout,
            line_break_mode: LineBreaking::Clip,
        }
    }

    pub fn text(&self) -> &Arc<str> {
        self.text_layout.text()
    }
}

impl Widget for CodeBlock {
    fn on_pointer_event(&mut self, ctx: &mut EventCtx, event: &PointerEvent) {
        // todo!()
    }

    fn on_text_event(&mut self, ctx: &mut EventCtx, event: &TextEvent) {
        // todo!()
    }

    fn on_access_event(&mut self, ctx: &mut EventCtx, event: &AccessEvent) {
        // todo!()
    }

    fn on_status_change(&mut self, ctx: &mut LifeCycleCtx, event: &StatusChange) {
        // todo!()
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle) {
        // todo!()
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints) -> masonry::Size {
        // Compute max_advance from box constraints
        let max_advance = if self.line_break_mode != LineBreaking::WordWrap {
            None
        } else if bc.max().width.is_finite() {
            Some(bc.max().width as f32 - 2. * LABEL_X_PADDING as f32)
        } else if bc.min().width.is_sign_negative() {
            Some(0.0)
        } else {
            None
        };
        self.text_layout.set_max_advance(max_advance);
        if self.text_layout.needs_rebuild() {
            let (font_ctx, layout_ctx) = ctx.text_contexts();
            self.text_layout.rebuild(font_ctx, layout_ctx);
        }
        // We ignore trailing whitespace for a label
        let text_size = self.text_layout.size();
        let label_size = Size {
            height: text_size.height,
            width: text_size.width + 2. * LABEL_X_PADDING,
        };
        let size = bc.constrain(label_size);
        trace!(
            "Computed layout: max={:?}. w={}, h={}",
            max_advance,
            size.width,
            size.height,
        );
        size
    }

    fn paint(&mut self, ctx: &mut PaintCtx, scene: &mut Scene) {
        if self.text_layout.needs_rebuild() {
            panic!("Called Label paint before layout");
        }
        if self.line_break_mode == LineBreaking::Clip {
            let clip_rect = ctx.size().to_rect();
            scene.push_layer(BlendMode::default(), 1., Affine::IDENTITY, &clip_rect);
        }
        self.text_layout
            .draw(scene, Point::new(LABEL_X_PADDING, 0.0));

        if self.line_break_mode == LineBreaking::Clip {
            scene.pop_layer();
        }
    }

    fn accessibility_role(&self) -> Role {
        accesskit::Role::Paragraph
    }

    fn accessibility(&mut self, ctx: &mut AccessCtx) {
        ctx.current_node().set_name("Code Block");
    }

    fn children_ids(&self) -> smallvec::SmallVec<[WidgetId; 16]> {
        SmallVec::new()
    }
}
