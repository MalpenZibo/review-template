mod app;

use app::App;

fn main() {
    review::init_logger();

    review::render(App(()).into(), "root");
}
