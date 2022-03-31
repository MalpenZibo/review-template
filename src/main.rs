mod app;

use app::App;

fn main() {
    review::init_logger(review::log::Level::Debug);

    review::render(App(()).into(), "root");
}
