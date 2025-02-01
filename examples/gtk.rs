#[cfg(target_os = "linux")]
use gtk4::prelude::*;
#[cfg(target_os = "linux")]
use keyboard_types::{Code, Modifiers};
#[cfg(target_os = "linux")]
use muda::{accelerator::Accelerator, MenuEvent};

#[cfg(target_os = "linux")]
fn main() {
    // Create a new application
    let application = gtk4::Application::builder()
        .application_id("com.github.gtk4-rs.examples.menubar")
        .build();
    application.connect_startup(on_startup);
    application.connect_activate(on_activate);
    application.run();
}

#[cfg(target_os = "linux")]
fn on_startup(_: &gtk4::Application) {
    MenuEvent::set_event_handler(Some(|event| {
        println!("{event:?}");
    }));
}

#[cfg(target_os = "linux")]
fn on_activate(application: &gtk4::Application) {
    let window = gtk4::ApplicationWindow::builder()
        .application(application)
        .title("Menubar Example")
        .default_width(350)
        .default_height(350)
        .show_menubar(true)
        .build();

    window.present();

    let menubar = {
        let file_menu = {
            let about_menu_item = muda::MenuItem::new("About", true, None);
            let quit_menu_item = muda::MenuItem::new(
                "About",
                true,
                Some(Accelerator::new(Modifiers::CONTROL, Code::KeyQ)),
            );

            let file_menu = muda::Submenu::new("File", true);
            file_menu.append(&about_menu_item).unwrap();
            file_menu.append(&quit_menu_item).unwrap();
            file_menu
        };

        let menubar = muda::Menu::new();
        menubar.append(&file_menu).unwrap();

        menubar
    };

    menubar
        .init_for_gtk_window(&window, None::<&gtk4::Window>)
        .unwrap();
}

#[cfg(not(target_os = "linux"))]
fn main() {}
