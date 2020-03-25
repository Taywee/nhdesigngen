use crate::design::{Design, Type};
use crate::gtk::{Result, TryGet};
use glib::object::IsA;
use gtk::prelude::*;
use std::borrow::Cow;
use std::cell::RefCell;
use std::fs::File;
use std::i16;
use std::ops::Deref;
use std::path::PathBuf;
use std::rc::Rc;

/** GTK design management struct.
 *
 * Manages a window and the menus as well.
 */
pub struct Window {
    builder: gtk::Builder,
    window: gtk::ApplicationWindow,
    design: RefCell<Design>,
}

fn open<T: IsA<gtk::Window>>(window: &T) -> Option<PathBuf> {
    let file_chooser_dialog = gtk::FileChooserDialog::with_buttons(
        Some("Open File"),
        Some(window),
        gtk::FileChooserAction::Open,
        &[
            ("_Cancel", gtk::ResponseType::Cancel),
            ("_Open", gtk::ResponseType::Accept),
        ],
    );
    let response = file_chooser_dialog.run();
    let filename = file_chooser_dialog.get_filename();
    file_chooser_dialog.destroy();
    if response == gtk::ResponseType::Accept {
        filename
    } else {
        None
    }
}

impl Window {
    /**
     * Build and show a Window.
     *
     * Also sets up all menus and other such things for regular functionality.  Returns a Rc to
     * signal that the reference is shared.  Nameably, the reference is shared between this class
     * and its signals, which retain weak references to it.
     */
    pub fn new() -> Result<Rc<Window>> {
        let design = Default::default();

        let builder = gtk::Builder::new_from_string(include_str!("window.glade"));
        let window: gtk::ApplicationWindow = builder.try_get("window")?;
        window.connect_destroy(|_| {
            if gtk::main_level() > 0 {
                gtk::main_quit();
            }
        });

        window.set_default_size(400, 400);
        window.show_all();

        let window = Rc::new(Window {
            builder,
            window,
            design,
        });

        {
            let load_palette: gtk::Button = window.builder.try_get("load_palette")?;
            let window = Rc::downgrade(&window);
            load_palette.connect_clicked(move |_| {
                if let Some(window) = window.upgrade() {
                    if let Some(path) = open(&window.window) {
                        let mut design = window.design.borrow_mut();
                        design.load_palette(&[path], Type::Simple);
                    }
                }
            });
        }

        Window::update(&window)?;
        Ok(window)
    }

    fn update(window: &Rc<Self>) -> Result<()> {
        Ok(())
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        self.window.destroy();
    }
}

