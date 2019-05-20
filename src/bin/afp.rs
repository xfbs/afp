extern crate afp;

use afp::ui::*;

fn main() {
    let app = App::new("net.xfbs.afs");
    app.init();
    app.run();
}
