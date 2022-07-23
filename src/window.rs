use adw::prelude::*;

pub fn main_window(app: gtk::Application) {
  app.connect_activate(|app| {

    let text = gtk::Label::builder()
      .label("Hello world!")
      .margin_top(32)
      .margin_bottom(32)
      .build();

    let content = gtk::Box::new(gtk::Orientation::Vertical, 0);

    content.append(&adw::HeaderBar::new());
    content.append(&text);

    let window = adw::ApplicationWindow::builder()
      .application(app)
      .title("Hello")
      .resizable(false)
      .content(&content)
      .build();
    
    window.show();

  });

  app.run();
}