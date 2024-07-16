use masonry::parley::style::FontFamily;
use masonry::parley::style::FontStack;
use masonry::parley::style::GenericFamily;
use winit::event_loop::EventLoop;
use xilem::view::flex;
use xilem::view::label;
use xilem::WidgetView;
use xilem::Xilem;

struct AppData();

fn app_logic(_data: &mut AppData) -> impl WidgetView<AppData> {
    flex((label("a"), label("b"))).direction(xilem::Axis::Vertical)
}

fn main() {
    let data = AppData();

    let app = Xilem::new(data, app_logic);
    let event_loop = EventLoop::with_user_event();
    app.run_windowed(event_loop, "playground".into()).unwrap();
}
