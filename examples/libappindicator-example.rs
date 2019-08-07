extern crate gtk_sys;
extern crate gtk;
extern crate libappindicator;

use gtk::{WidgetExt, MenuShellExt, GtkMenuItemExt};
use std::ffi::CStr;
use libappindicator::{AppIndicator, AppIndicatorStatus};

fn main() {
    gtk::init().unwrap();
    let mut indicator = AppIndicator::new("libappindicator test application", "");
    indicator.set_status(AppIndicatorStatus::Active);
    indicator.set_icon_full("/usr/share/gxkb/flags/ua.png", "icon");
    let mut m = gtk::Menu::new();
    let quit: &CStr = unsafe { CStr::from_ptr(gtk_sys::GTK_STOCK_QUIT) };
    let mi = gtk::MenuItem::new_with_label(quit.to_str().unwrap());
    mi.connect_activate(|_| {
        gtk::main_quit();
    });
    m.append(&mi);
    indicator.set_menu(&mut m);
    m.show_all();
    gtk::main();
}
