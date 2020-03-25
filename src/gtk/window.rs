use crate::color::NHPaletteItem;
use crate::design::{Design};
use crate::gtk::{Result, TryGet};
use glib::object::IsA;
use gtk::prelude::*;
use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;
use exoquant::optimizer;
use exoquant::ditherer;

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

fn open_multiple<T: IsA<gtk::Window>>(window: &T) -> Option<Vec<PathBuf>> {
    let file_chooser_dialog = gtk::FileChooserDialog::with_buttons(
        Some("Open File"),
        Some(window),
        gtk::FileChooserAction::Open,
        &[
            ("_Cancel", gtk::ResponseType::Cancel),
            ("_Open", gtk::ResponseType::Accept),
        ],
    );
    file_chooser_dialog
        .set_property("select-multiple", &true)
        .unwrap();
    let response = file_chooser_dialog.run();
    let filenames = file_chooser_dialog.get_filenames();
    file_chooser_dialog.destroy();
    if response == gtk::ResponseType::Accept {
        Some(filenames)
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
            let ditherer_combobox: gtk::ComboBoxText = window.builder.try_get("ditherer")?;
            let window = Rc::downgrade(&window);
            ditherer_combobox.connect_changed(move |_| {
                if let Some(window) = window.upgrade() {
                    Window::update(&window).unwrap();
                }
            });
        }

        {
            let load_palette: gtk::Button = window.builder.try_get("load_palette")?;
            let optimizer_combobox: gtk::ComboBoxText = window.builder.try_get("optimizer")?;

            let window = Rc::downgrade(&window);
            load_palette.connect_clicked(move |_| {
                if let Some(window) = window.upgrade() {
                    let optimizer_active_id: Option<String> = optimizer_combobox.get_active_id().map(|s| s.to_string());
                    let optimizer: Box<dyn optimizer::Optimizer> = match optimizer_active_id {
                        Some(id) => {
                            match id.as_str() {
                                "kmeans" => Box::new(optimizer::KMeans),
                                "weighted-kmeans" => Box::new(optimizer::WeightedKMeans),
                                o => panic!("Could not find optimizer {:?}", o),
                                
                            }
                        }
                        None => panic!("Optimizer dropdown box failed to give a value!"),
                    };

                    if let Some(paths) = open_multiple(&window.window) {
                        let mut design = window.design.borrow_mut();
                        design.load_palette(paths, optimizer).unwrap();
                    }
                    Window::update(&window).unwrap();
                }
            });
        }

        {
            let load_image: gtk::Button = window.builder.try_get("load_image")?;
            let window = Rc::downgrade(&window);
            load_image.connect_clicked(move |_| {
                if let Some(window) = window.upgrade() {
                    if let Some(path) = open(&window.window) {
                        let mut design = window.design.borrow_mut();
                        design.load_image(path).unwrap();
                    }
                    Window::update(&window).unwrap();
                }
            });
        }

        Window::update(&window)?;
        Ok(window)
    }

    fn update(window: &Rc<Self>) -> Result<()> {
        let design = window.design.borrow();
        let dimensions = design.dimensions();
        let width = dimensions.0 as usize;
        let palette_box: gtk::FlowBox = window.builder.try_get("palette")?;
        let ditherer_combobox: gtk::ComboBoxText = window.builder.try_get("ditherer")?;
        let ditherer_active_id: Option<String> = ditherer_combobox.get_active_id().map(|s| s.to_string());
        let ditherer: Box<dyn ditherer::Ditherer> = match ditherer_active_id {
            Some(id) => {
                match id.as_str() {
                    "none" => Box::new(ditherer::None),
                    "floyd-steinberg" => Box::new(ditherer::FloydSteinberg::new()),
                    "floyd-steinberg-vanilla" => Box::new(ditherer::FloydSteinberg::vanilla()),
                    "floyd-steinberg-checkered" => Box::new(ditherer::FloydSteinberg::checkered()),
                    "ordered" => Box::new(ditherer::Ordered),
                    d => panic!("Could not find ditherer {:?}", d),
                }
            }
            None => panic!("Ditherer dropdown box failed to give a value!"),
        };

        let palette: Vec<NHPaletteItem> = design.palette().iter().cloned().collect();
        palette_box.foreach(gtk::WidgetExt::destroy);
        for (i, item) in palette.iter().enumerate() {
            let rgba: gdk::RGBA = item.into();
            let frame = gtk::Frame::new(Some(&(i + 1).to_string()));
            let color_button = gtk::ColorButton::new_with_rgba(&rgba);
            let item_box = gtk::Box::new(gtk::Orientation::Vertical, 12);
            item_box.set_border_width(12);
            item_box.add(&color_button);
            match item {
                NHPaletteItem::Color(color) => {
                    let hue_frame = gtk::Frame::new(Some("hue"));
                    hue_frame.add(&gtk::Label::new(Some(&(color.h + 1).to_string())));
                    let saturation_frame = gtk::Frame::new(Some("saturation"));
                    saturation_frame.add(&gtk::Label::new(Some(&(color.s + 1).to_string())));
                    let value_frame = gtk::Frame::new(Some("value"));
                    value_frame.add(&gtk::Label::new(Some(&(color.v + 1).to_string())));
                    item_box.add(&hue_frame);
                    item_box.add(&saturation_frame);
                    item_box.add(&value_frame);
                }
                NHPaletteItem::Transparent => {
                    item_box.add(&gtk::Label::new(Some("Transparent")));
                }
            }
            frame.add(&item_box);
            palette_box.add(&frame);
        }
        palette_box.show_all();

        let pixbuf = gdk_pixbuf::Pixbuf::new(gdk_pixbuf::Colorspace::Rgb, true, 8, dimensions.0 as i32, dimensions.1 as i32).unwrap();

        let grid: gtk::Grid = window.builder.try_get("grid")?;
        grid.foreach(gtk::WidgetExt::destroy);

        for (i, index) in design.generate(ditherer).iter().enumerate() {
            let x = i % width;
            let y = i / width;
            let pixel = &palette[*index as usize];
            let rgba: gdk::RGBA = pixel.into();
            let color_button = gtk::ColorButton::new_with_rgba(&rgba);
            let aspect_frame = gtk::AspectFrame::new(Some(&(index + 1).to_string()), 0.5, 0.5, 1.0, false);
            aspect_frame.set_label_align(0.5, 0.5);
            aspect_frame.add(&color_button);
            grid.attach(&aspect_frame, x as i32, y as i32, 1, 1);

            let color: exoquant::Color = pixel.into();
            pixbuf.put_pixel(x as i32, y as i32, color.r, color.g, color.b, color.a);
        }
        grid.show_all();

        let image: gtk::Image = window.builder.try_get("image")?;

        let pixbuf = pixbuf.scale_simple(dimensions.0 as i32 * 10, dimensions.1 as i32 * 10, gdk_pixbuf::InterpType::Nearest).unwrap();
        image.set_from_pixbuf(Some(&pixbuf));

        Ok(())
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        self.window.destroy();
    }
}
