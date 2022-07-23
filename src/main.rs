use adw::prelude::*;

mod window;

fn main() {
    let app = gtk::Application::builder()
        .application_id("com.github.nebulaa44.Hello")
        .build();

    app.connect_startup(|_| {
        adw::init();
    });

    app.connect_activate(|app| {
        window::main_window(app);
    });

    window::main_window(&app);
}
