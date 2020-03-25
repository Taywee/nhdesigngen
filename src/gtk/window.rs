use crate::character::Alignment;
use crate::character::Character;
use crate::gtk::Ability;
use crate::gtk::AboutDialog;
use crate::gtk::DropdownEditDialog;
use crate::gtk::EditDialog;
use crate::gtk::NameContentEditDialog;
use crate::gtk::NumberEditDialog;
use crate::gtk::Result;
use crate::gtk::{MenuExt as _, MenuItemExt as _, TryGet as _};
use glib::object::IsA;
use gtk::prelude::*;
use std::borrow::Cow;
use std::cell::RefCell;
use std::fs::File;
use std::i16;
use std::ops::Deref;
use std::path::PathBuf;
use std::rc::Rc;

/** GTK character management struct.
 *
 * Manages a window and the menus as well.
 */
pub struct Window {
    builder: gtk::Builder,
    window: gtk::ApplicationWindow,
    character: RefCell<Character>,
}

fn save_as<T: IsA<gtk::Window>>(window: &T) -> Option<PathBuf> {
    let file_chooser_dialog = gtk::FileChooserDialog::with_buttons(
        Some("Save File"),
        Some(window),
        gtk::FileChooserAction::Save,
        &[
            ("_Cancel", gtk::ResponseType::Cancel),
            ("_Save", gtk::ResponseType::Accept),
        ],
    );
    file_chooser_dialog.set_do_overwrite_confirmation(true);
    let response = file_chooser_dialog.run();
    let filename = file_chooser_dialog.get_filename();
    file_chooser_dialog.destroy();
    if response == gtk::ResponseType::Accept {
        filename
    } else {
        None
    }
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
    pub fn new<C: Into<RefCell<Character>>>(character: C) -> Result<Rc<Window>> {
        let character = character.into();

        let builder = gtk::Builder::new_from_string(include_str!("character_window.glade"));
        let window: gtk::ApplicationWindow = builder.try_get("window")?;
        window.connect_destroy(|_| {
            if gtk::main_level() > 0 {
                gtk::main_quit();
            }
        });

        window.set_default_size(200, -1);

        let character_window = Rc::new(Window {
            builder,
            character,
            window,
        });

        let menu = gtk::Menu::new();
        menu.popup_for(&character_window.builder.try_get("name_event_box")?);
        menu.make_item("_Edit").connect_with_window(
            Rc::downgrade(&character_window),
            |character_window| {
                let editor = {
                    let character = character_window.character.borrow();
                    EditDialog::new(&character_window.window, "Name", &character.name).unwrap()
                };
                if let Some(name) = editor.run() {
                    {
                        let mut character = character_window.character.borrow_mut();
                        character.name = name;
                    }
                    Window::update(&character_window).unwrap();
                }
            },
        );

        let menu = gtk::Menu::new();
        menu.popup_for(&character_window.builder.try_get("race_event_box")?);
        menu.make_item("_Edit").connect_with_window(
            Rc::downgrade(&character_window),
            |character_window| {
                let editor = {
                    let character = character_window.character.borrow();
                    EditDialog::new(&character_window.window, "Race", &character.race).unwrap()
                };
                if let Some(race) = editor.run() {
                    {
                        let mut character = character_window.character.borrow_mut();
                        character.race = race;
                    }
                    Window::update(&character_window).unwrap();
                }
            },
        );

        let menu = gtk::Menu::new();
        menu.popup_for(&character_window.builder.try_get("alignment_event_box")?);
        menu.make_item("_Edit").connect_with_window(
            Rc::downgrade(&character_window),
            |character_window| {
                let editor = {
                    let character = character_window.character.borrow();
                    let choices: Vec<(String, String)> = vec![
                        ("lg".into(), "Lawful Good".into()),
                        ("ng".into(), "Neutral Good".into()),
                        ("cg".into(), "Chaotic Good".into()),
                        ("ln".into(), "Lawful Neutral".into()),
                        ("nn".into(), "Neutral".into()),
                        ("cn".into(), "Chaotic Neutral".into()),
                        ("le".into(), "Lawful Evil".into()),
                        ("ne".into(), "Neutral Evil".into()),
                        ("ce".into(), "Chaotic Evil".into()),
                    ];
                    DropdownEditDialog::new(
                        &character_window.window,
                        "Alignment",
                        choices,
                        &character.alignment.id_str(),
                    )
                    .unwrap()
                };
                if let Some(alignment) = editor.run() {
                    if let Some(alignment) = Alignment::from_id_str(&alignment) {
                        let mut character = character_window.character.borrow_mut();
                        character.alignment = alignment;
                    }
                    Window::update(&character_window).unwrap();
                }
            },
        );

        let menu = gtk::Menu::new();
        menu.popup_for(&character_window.builder.try_get("background_event_box")?);
        menu.make_item("_Edit").connect_with_window(
            Rc::downgrade(&character_window),
            |character_window| {
                let editor = {
                    let character = character_window.character.borrow();
                    EditDialog::new(
                        &character_window.window,
                        "Background",
                        &character.background,
                    )
                    .unwrap()
                };
                if let Some(background) = editor.run() {
                    {
                        let mut character = character_window.character.borrow_mut();
                        character.background = background;
                    }
                    Window::update(&character_window).unwrap();
                }
            },
        );

        let menu = gtk::Menu::new();
        menu.popup_for(&character_window.builder.try_get("class_event_box")?);
        menu.make_item("_Edit").connect_with_window(
            Rc::downgrade(&character_window),
            |character_window| {
                let editor = {
                    let character = character_window.character.borrow();
                    EditDialog::new(&character_window.window, "Class", &character.class).unwrap()
                };
                if let Some(class) = editor.run() {
                    {
                        let mut character = character_window.character.borrow_mut();
                        character.class = class;
                    }
                    Window::update(&character_window).unwrap();
                }
            },
        );

        let menu = gtk::Menu::new();
        menu.popup_for(&character_window.builder.try_get("weight_event_box")?);
        menu.make_item("_Edit").connect_with_window(
            Rc::downgrade(&character_window),
            |character_window| {
                let editor = {
                    let character = character_window.character.borrow();
                    NumberEditDialog::new(
                        &character_window.window,
                        "Weight",
                        character.body_weight as f64,
                    )
                    .unwrap()
                };
                if let Some(weight) = editor.run() {
                    {
                        let mut character = character_window.character.borrow_mut();
                        character.body_weight = weight as f32;
                    }
                    Window::update(&character_window).unwrap();
                }
            },
        );

        let menu = gtk::Menu::new();
        menu.popup_for(&character_window.builder.try_get("details")?);
        menu.make_item("Add").connect_with_window(
            Rc::downgrade(&character_window),
            |character_window| {
                let editor = NameContentEditDialog::new(&character_window.window, "", "").unwrap();
                if let Some((name, content)) = editor.run() {
                    {
                        let mut character = character_window.character.borrow_mut();
                        character.details.push((name, content));
                    }
                    Window::update(&character_window).unwrap();
                }
            },
        );

        let menu = gtk::Menu::new();
        menu.popup_for(&character_window.builder.try_get("experience_event_box")?);
        menu.make_item("_Edit").connect_with_window(
            Rc::downgrade(&character_window),
            |character_window| {
                let editor = {
                    let character = character_window.character.borrow();
                    NumberEditDialog::new(
                        &character_window.window,
                        "Experience",
                        character.experience as f64,
                    )
                    .unwrap()
                };
                if let Some(experience) = editor.run() {
                    {
                        let mut character = character_window.character.borrow_mut();
                        character.experience = experience as u32;
                    }
                    Window::update(&character_window).unwrap();
                }
            },
        );

        let menu = gtk::Menu::new();
        menu.popup_for(&character_window.builder.try_get("inspiration_event_box")?);
        menu.make_item("_Edit").connect_with_window(
            Rc::downgrade(&character_window),
            |character_window| {
                let editor = {
                    let character = character_window.character.borrow();
                    NumberEditDialog::new(
                        &character_window.window,
                        "Inspiration",
                        character.inspiration as f64,
                    )
                    .unwrap()
                };
                if let Some(inspiration) = editor.run() {
                    {
                        let mut character = character_window.character.borrow_mut();
                        character.inspiration = inspiration as u16;
                    }
                    Window::update(&character_window).unwrap();
                }
            },
        );

        let menu = gtk::Menu::new();
        menu.popup_for(&character_window.builder.try_get("armor_class_event_box")?);
        menu.make_item("_Edit").connect_with_window(
            Rc::downgrade(&character_window),
            |character_window| {
                let editor = {
                    let character = character_window.character.borrow();
                    NumberEditDialog::new(
                        &character_window.window,
                        "Armor Class",
                        character.armor_class as f64,
                    )
                    .unwrap()
                };
                if let Some(armor_class) = editor.run() {
                    {
                        let mut character = character_window.character.borrow_mut();
                        character.armor_class = armor_class as u8;
                    }
                    Window::update(&character_window).unwrap();
                }
            },
        );

        let menu = gtk::Menu::new();
        menu.popup_for(&character_window.builder.try_get("speed_event_box")?);
        menu.make_item("_Edit").connect_with_window(
            Rc::downgrade(&character_window),
            |character_window| {
                let editor = {
                    let character = character_window.character.borrow();
                    NumberEditDialog::new(&character_window.window, "Speed", character.speed as f64)
                        .unwrap()
                };
                if let Some(speed) = editor.run() {
                    {
                        let mut character = character_window.character.borrow_mut();
                        character.speed = speed as u16;
                    }
                    Window::update(&character_window).unwrap();
                }
            },
        );

        let menu = gtk::Menu::new();
        menu.popup_for(
            &character_window
                .builder
                .try_get("base_hit_point_maximum_event_box")?,
        );
        menu.make_item("_Edit").connect_with_window(
            Rc::downgrade(&character_window),
            |character_window| {
                let editor = {
                    let character = character_window.character.borrow();
                    NumberEditDialog::new(
                        &character_window.window,
                        "Base Hit Point Maximum",
                        character.base_hit_point_maximum as f64,
                    )
                    .unwrap()
                };
                if let Some(base_hit_point_maximum) = editor.run() {
                    {
                        let mut character = character_window.character.borrow_mut();
                        character.base_hit_point_maximum = base_hit_point_maximum as u16;
                    }
                    Window::update(&character_window).unwrap();
                }
            },
        );

        let menu = gtk::Menu::new();
        menu.popup_for(
            &character_window
                .builder
                .try_get("hit_point_bonus_event_box")?,
        );
        menu.make_item("_Edit").connect_with_window(
            Rc::downgrade(&character_window),
            |character_window| {
                let editor = {
                    let character = character_window.character.borrow();
                    NumberEditDialog::new(
                        &character_window.window,
                        "Hit Points Bonus",
                        character.hit_point_bonus as f64,
                    )
                    .unwrap()
                };
                editor.entry().set_range(i16::MIN as f64, i16::MAX as f64);
                if let Some(hit_point_bonus) = editor.run() {
                    {
                        let mut character = character_window.character.borrow_mut();
                        character.hit_point_bonus = hit_point_bonus as i16;
                    }
                    Window::update(&character_window).unwrap();
                }
            },
        );

        // Setup dropdown menus
        character_window
            .builder
            .try_get::<gtk::MenuItem>("menu_open")?
            .connect_with_window(Rc::downgrade(&character_window), |character_window| {
                if let Some(path) = open(&character_window.window) {
                    // TODO: error handling
                    let file = File::open(path).unwrap();
                    character_window
                        .character
                        .replace(serde_json::from_reader(file).unwrap());
                    Window::update(&character_window).unwrap();
                }
            });

        character_window
            .builder
            .try_get::<gtk::MenuItem>("menu_save")?
            .connect_with_window(Rc::downgrade(&character_window), |character_window| {
                if let Some(path) = save_as(&character_window.window) {
                    let file = File::create(path).unwrap();
                    serde_json::to_writer_pretty(file, &*character_window.character.borrow())
                        .unwrap();
                }
            });

        character_window
            .builder
            .try_get::<gtk::MenuItem>("menu_save_as")?
            .connect_with_window(Rc::downgrade(&character_window), |character_window| {
                if let Some(path) = save_as(&character_window.window) {
                    let file = File::create(path).unwrap();
                    serde_json::to_writer_pretty(file, &*character_window.character.borrow())
                        .unwrap();
                }
            });

        character_window
            .builder
            .try_get::<gtk::MenuItem>("menu_quit")?
            .connect_with_window(Rc::downgrade(&character_window), |character_window| {
                character_window.window.destroy();
            });

        character_window
            .builder
            .try_get::<gtk::MenuItem>("menu_about")?
            .connect_with_window(Rc::downgrade(&character_window), |character_window| {
                let dialog = AboutDialog::new(&character_window.window).unwrap();
                dialog.run();
            });

        Window::update(&character_window)?;
        Ok(character_window)
    }

    fn update(window: &Rc<Self>) -> Result<()> {
        let name_label: gtk::Label = window.builder.try_get("name")?;
        let race_label: gtk::Label = window.builder.try_get("race")?;
        let alignment_label: gtk::Label = window.builder.try_get("alignment")?;
        let background_label: gtk::Label = window.builder.try_get("background")?;
        let class_label: gtk::Label = window.builder.try_get("class")?;
        let weight_label: gtk::Label = window.builder.try_get("weight")?;
        let details_flowbox: gtk::FlowBox = window.builder.try_get("details")?;

        let character = window.character.borrow();
        name_label.set_text(&character.name);
        race_label.set_text(&character.race);
        alignment_label.set_text(&character.alignment.to_string());
        background_label.set_text(&character.background);
        class_label.set_text(&character.class);
        weight_label.set_text(&character.body_weight.to_string());
        // Wipe existing children
        details_flowbox.foreach(gtk::WidgetExt::destroy);
        // reload details_flowbox
        let details_len = character.details.len();
        // TODO: refactor this into its own section.  Probably do the same thing as the other glade
        // files.  Each type that takes its own builder should probably be in its own file.
        for (i, (key, value)) in character.details.iter().enumerate() {
            let builder = gtk::Builder::new_from_string(include_str!("details_frame.glade"));
            let name: gtk::Label = builder.try_get("name")?;
            let content: gtk::Label = builder.try_get("content")?;
            let event_box: gtk::EventBox = builder.try_get("event_box")?;
            name.set_text(key);
            content.set_text(value);
            let child = gtk::FlowBoxChild::new();

            let menu = gtk::Menu::new();
            {
                let menu = menu.clone();

                event_box.connect_button_press_event(move |_, event_button| {
                    let right_click = event_button.get_button() == 3;
                    if right_click {
                        menu.popup_easy(event_button.get_button(), event_button.get_time());
                    }
                    Inhibit(right_click)
                });
            }

            menu.make_item("_Edit").connect_with_window(
                Rc::downgrade(&window),
                move |character_window| {
                    let editor = {
                        let character = character_window.character.borrow();
                        let (key, content) = &character.details[i];
                        NameContentEditDialog::new(&character_window.window, key, content).unwrap()
                    };
                    if let Some((name, content)) = editor.run() {
                        {
                            let mut character = character_window.character.borrow_mut();
                            character.details[i] = (name, content);
                        }
                        Window::update(&character_window).unwrap();
                    }
                },
            );

            if i > 0 {
                menu.make_item("Move _Left").connect_with_window(
                    Rc::downgrade(&window),
                    move |character_window| {
                        {
                            let mut character = character_window.character.borrow_mut();
                            character.details.swap(i, i - 1);
                        }
                        Window::update(&character_window).unwrap();
                    },
                );
            }

            if i < details_len - 1 {
                menu.make_item("Move _Right").connect_with_window(
                    Rc::downgrade(&window),
                    move |character_window| {
                        {
                            let mut character = character_window.character.borrow_mut();
                            character.details.swap(i, i + 1);
                        }
                        Window::update(&character_window).unwrap();
                    },
                );
            }

            menu.add(&{
                let item = gtk::SeparatorMenuItem::new();
                item.set_visible(true);
                item
            });

            menu.make_item("_Add").connect_with_window(
                Rc::downgrade(&window),
                move |character_window| {
                    let editor =
                        NameContentEditDialog::new(&character_window.window, "", "").unwrap();
                    if let Some((name, content)) = editor.run() {
                        {
                            let mut character = character_window.character.borrow_mut();
                            character.details.push((name, content));
                        }
                        Window::update(&character_window).unwrap();
                    }
                },
            );

            child.add(&event_box);
            child.set_can_focus(false);
            details_flowbox.add(&child);
        }
        details_flowbox.show_all();

        // Generate skill and stat stuff
        let abilities_box: gtk::Box = window.builder.try_get("abilities")?;
        abilities_box.foreach(gtk::WidgetExt::destroy);
        for ability in character.abilities.iter_rc() {
            let ability_ref = ability.borrow();
            let ability_frame = Ability::new(ability_ref.deref())?;
            abilities_box.add(ability_frame.frame());

            // Allow editing modifier
            let menu = gtk::Menu::new();
            {
                let menu = menu.clone();

                ability_frame
                    .event_box()
                    .connect_button_press_event(move |_, event_button| {
                        // right click
                        if event_button.get_button() == 3 {
                            menu.popup_easy(event_button.get_button(), event_button.get_time());
                            Inhibit(true)
                        } else {
                            Inhibit(false)
                        }
                    });
            }

            {
                let ability = Rc::downgrade(&ability);

                menu.make_item("_Edit Score").connect_with_window(
                    Rc::downgrade(&window),
                    move |character_window| {
                        if let Some(ability) = ability.upgrade() {
                            let editor = {
                                let ability = ability.borrow();
                                let modifier = ability.score();
                                NumberEditDialog::new(
                                    &character_window.window,
                                    ability.name(),
                                    modifier as f64,
                                )
                                .unwrap()
                            };
                            editor.entry().set_range(0.0, 30.0);
                            editor.entry().set_increments(1.0, 2.0);
                            if let Some(modifier) = editor.run() {
                                {
                                    let mut ability = ability.borrow_mut();
                                    ability.set_score(modifier as i8);
                                }
                                Window::update(&character_window).unwrap();
                            }
                        }
                    },
                );
            }

            /// Make a simple skill box.
            ///
            /// Abstracted here because saving throws and skills are done slightly differently.
            /// Might want to do this elsewhere later, like crate::gtk::Ability.
            fn make_skill_box(
                window: &Rc<Window>,
                proficiency_id: String,
                label: String,
                modifier: i8,
            ) -> gtk::EventBox {
                let character = window.character.borrow();
                let proficiency_level = character.proficiency_level(&proficiency_id);
                let bonus = (proficiency_level * character.proficiency()) as i8;
                let modifier = modifier + bonus;
                // Borrow or owned, for a slight efficincy gain
                let proficiency_str: Cow<str> = match proficiency_level {
                    0 => Cow::Borrowed(""),
                    1 => Cow::Borrowed("P"),
                    2 => Cow::Borrowed("E"),
                    x => Cow::Owned(format!("{}", x)),
                };
                let event_box = gtk::EventBox::new();
                let frame = gtk::Frame::new(Some(&*proficiency_str));
                frame.set_property_label_xalign(0.5);
                // The total modifier
                frame.add(&gtk::Label::new(Some(&format!("{:+}", modifier))));
                // Allow editing proficiency
                let menu = gtk::Menu::new();
                {
                    let menu = menu.clone();

                    event_box.connect_button_press_event(move |_, event_button| {
                        // right click
                        if event_button.get_button() == 3 {
                            menu.popup_easy(event_button.get_button(), event_button.get_time());
                            Inhibit(true)
                        } else {
                            Inhibit(false)
                        }
                    });
                }

                menu.make_item("_Edit Proficiency").connect_with_window(
                    Rc::downgrade(&window),
                    move |character_window| {
                        let editor = {
                            NumberEditDialog::new(
                                &character_window.window,
                                &label,
                                proficiency_level as f64,
                            )
                            .unwrap()
                        };
                        editor.entry().set_range(0.0, 2.0);
                        editor.entry().set_increments(1.0, 1.0);
                        if let Some(proficiency) = editor.run() {
                            {
                                let mut character = character_window.character.borrow_mut();
                                character
                                    .set_proficiency(proficiency_id.clone(), proficiency as u8);
                            }
                            Window::update(&character_window).unwrap();
                        }
                    },
                );
                event_box.add(&frame);
                event_box
            }

            // Saving throw
            let skill_box = make_skill_box(
                &window,
                format!("save.{}", ability_ref.id()),
                format!("{} Saving Throw", ability_ref.name()),
                ability_ref.modifier(),
            );

            let skill_grid = ability_frame.skill_grid();
            skill_grid.insert_row(0);
            skill_grid.attach(&skill_box, 0, 0, 1, 1);
            let skill_label = gtk::Label::new(Some("Saving Throw"));
            skill_label.set_halign(gtk::Align::Start);
            skill_label.set_valign(gtk::Align::End);
            skill_grid.attach(&skill_label, 1, 0, 1, 1);

            // We build these in reverse because it's easier to just always be working on a static
            // row than to keep track of the row number as well for skill grid manipulation
            for skill in ability_ref.skills().rev() {
                let skill_box = make_skill_box(
                    &window,
                    format!("skill.{}", skill.id()),
                    skill.name().into(),
                    ability_ref.modifier(),
                );
                skill_grid.insert_row(1);
                skill_grid.attach(&skill_box, 0, 1, 1, 1);
                let skill_label = gtk::Label::new(Some(&format!("{}", skill.name())));
                skill_label.set_halign(gtk::Align::Start);
                skill_label.set_valign(gtk::Align::End);
                skill_grid.attach(&skill_label, 1, 1, 1, 1);
            }
            skill_grid.show_all();
        }

        let experience: gtk::Label = window.builder.try_get("experience")?;
        experience.set_text(&format!(
            "{} (level {})",
            character.experience,
            character.level()
        ));

        let proficiency: gtk::Label = window.builder.try_get("proficiency")?;
        proficiency.set_text(&format!("{:+}", character.proficiency()));

        let passive_perception: gtk::Label = window.builder.try_get("passive_perception")?;
        passive_perception.set_text(&(10 + character.skill_modifier("perception")).to_string());

        let passive_insight: gtk::Label = window.builder.try_get("passive_insight")?;
        passive_insight.set_text(&(10 + character.skill_modifier("insight")).to_string());

        let inspiration: gtk::Label = window.builder.try_get("inspiration")?;
        inspiration.set_text(&character.inspiration.to_string());

        let armor_class: gtk::Label = window.builder.try_get("armor_class")?;
        armor_class.set_text(&character.armor_class.to_string());

        let initiative: gtk::Label = window.builder.try_get("initiative")?;
        initiative.set_text(&format!(
            "{:+}",
            character.ability_modifier("dexterity").unwrap()
        ));

        let speed: gtk::Label = window.builder.try_get("speed")?;
        speed.set_text(&character.speed.to_string());

        let base_hit_point_maximum: gtk::Label =
            window.builder.try_get("base_hit_point_maximum")?;
        base_hit_point_maximum.set_text(&character.base_hit_point_maximum.to_string());

        let hit_point_bonus: gtk::Label = window.builder.try_get("hit_point_bonus")?;
        hit_point_bonus.set_text(&format!("{:+}", character.hit_point_bonus));

        let hit_point_maximum: gtk::Label = window.builder.try_get("hit_point_maximum")?;
        hit_point_maximum.set_text(&character.hit_point_maximum().to_string());

        Ok(())
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        self.window.destroy();
    }
}

