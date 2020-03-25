use nhdesigngen::gtk::window::Window;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    gtk::init()?;

    let _application = gtk::Application::new(
        Some("net.axfive.nhdesigngen"),
        gio::ApplicationFlags::FLAGS_NONE,
    )?;

    let _window = Window::new()?;

    gtk::main();
    Ok(())
}
