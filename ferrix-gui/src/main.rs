use ferrix_lib::cpu::Processors;
use ferrix_lib::ram::RAM;
use gtk::gio::{Menu, MenuItem};
use gtk::glib::clone;
use gtk::{
    glib, Application, ApplicationWindow, Button, HeaderBar, MenuButton, ScrolledWindow, Stack, StackSidebar
};
use gtk::{Box, Label, prelude::*};

const APP_ID: &str = "com.mskrasnov.Ferrix";
// const LOGO: &[u8] = include_bytes!("../data/icons/hicolor/scalable/apps/com.mskrasnov.Ferrix.svg");

fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    app.run()
}

fn build_ui(app: &Application) {
    let hbox = Box::new(gtk::Orientation::Horizontal, 0);
    let stack = Stack::new();
    let sidebar = StackSidebar::new();
    sidebar.set_stack(&stack);

    // stack.add_titled(&Label::new(Some("[TODO]")), Some("dash"), "Dashboard");
    stack.add_titled(&proc_page(), Some("proc"), "Processors");
    stack.add_titled(&ram_page(), Some("Memory"), "Memory");
    // stack.add_titled(&Label::new(Some("[TODO]")), Some("storage"), "Storage");
    // stack.add_titled(&Label::new(Some("[TODO]")), Some("dmi"), "DMI Tables");
    // stack.add_titled(&Label::new(Some("[TODO]")), Some("os"), "Operating system");

    hbox.append(&sidebar);
    hbox.append(&stack);

    sidebar.set_hexpand(false);
    sidebar.set_halign(gtk::Align::Start);
    stack.set_hexpand(true);

    let win = ApplicationWindow::builder()
        .application(app)
        .title("Ferrix")
        .titlebar(&create_hdr_bar())
        .child(&hbox)
        .build();

    // let bytes = glib::Bytes::from_static(LOGO);
    // let logo = gtk::gdk::Texture::from_bytes(&bytes).unwrap();
    // let (sender, receiver) = async_channel::bounded(1);

    // about_btn.connect_clicked(clone!(
    //     #[weak]
    //     win,
    //     move |_| {
    //         let dialog = gtk::AboutDialog::builder()
    //             .transient_for(&win)
    //             .modal(true)
    //             .program_name("Ferrix")
    //             .version(env!("CARGO_PKG_VERSION"))
    //             .website("https://mskrasnov.github.io/Ferrix/")
    //             .authors(["Michail Krasnov <mskrasnov07@ya.ru>"])
    //             .license_type(gtk::License::Gpl30)
    //             .logo(&logo)
    //             .build();
    //         dialog.present();
    //     }
    // ));
    // update_btn.connect_clicked(clone!(
    //     #[strong]
    //     sender,
    //     move |_| {
    //         glib::spawn_future_local(clone!(
    //             #[strong]
    //             sender,
    //             async move {
    //                 let data = RAM::new();
    //                 sender
    //                     .send(data)
    //                     .await
    //                     .expect("The channel needs to be open");
    //             }
    //         ));
    //     }
    // ));

    // glib::spawn_future_local(clone!(
    //     #[strong]
    //     sender,
    //     async move {
    //         loop {
    //             let data = RAM::new();
    //             sender
    //                 .send(data)
    //                 .await
    //                 .expect("The channel needs to be open");
    //             glib::timeout_future_seconds(1).await;
    //         }
    //     },
    // ));

    // glib::spawn_future_local(clone!(
    //     #[weak]
    //     label,
    //     async move {
    //         while let Ok(ram) = receiver.recv().await {
    //             label.set_label(&match ram {
    //                 Ok(ram) => format!("Free RAM: {}", ram.free.round(2).unwrap()),
    //                 Err(why) => format!("Error getting information:\n{why}"),
    //             });
    //         }
    //     }
    // ));
    win.present();
}

fn create_hdr_bar() -> HeaderBar {
    let hdr = HeaderBar::new();
    let menu = MenuButton::builder()
        .icon_name("open-menu-symbolic")
        .menu_model(&create_menu_btn())
        .tooltip_text("Меню")
        .css_classes(["header-button", "text-button"])
        .build();
    hdr.pack_end(&menu);

    let export_btn = Button::new();
    export_btn.add_css_class("header-button");
    export_btn.set_icon_name("document-save");
    export_btn.set_tooltip_text(Some("Экспорт"));
    hdr.pack_start(&export_btn);

    hdr
}

fn create_menu_btn() -> Menu {
    let menu = Menu::new();
    let section = Menu::new();
    section.append_item(&MenuItem::new(Some("Обновить"), None));
    section.append_item(&MenuItem::new(Some("Экспортировать"), None));
    section.append_item(&MenuItem::new(Some("О программе"), None));

    menu.append_section(None, &section);
    menu
}

fn proc_page() -> ScrolledWindow {
    let page = Box::new(gtk::Orientation::Vertical, 0);
    let (sender, receiver) = async_channel::bounded(1);
    glib::spawn_future_local(clone!(
        #[strong]
        sender,
        async move {
            loop {
                let data = Processors::new();
                sender
                    .send(data)
                    .await
                    .expect("The channel needs to be open");
                glib::timeout_future_seconds(1).await;
            }
        }
    ));
    let proc = Label::new(None);
    page.append(&proc);

    glib::spawn_future_local(clone!(
        #[weak]
        proc,
        #[strong]
        receiver,
        async move {
            while let Ok(proc_data) = receiver.recv().await {
                proc.set_label(&match proc_data {
                    Ok(proc) => format!("{:#?}", &proc.entries[0]),
                    Err(why) => format!("Error: {why}"),
                });
            }
        }
    ));

    let scrolled = ScrolledWindow::builder()
        .child(&page)
        .build();
    scrolled
}

fn ram_page() -> Box {
    let page = Box::new(gtk::Orientation::Vertical, 0);
    let (sender, receiver) = async_channel::bounded(1);
    glib::spawn_future_local(clone!(
        #[strong]
        sender,
        async move {
            loop {
                let data = RAM::new();
                sender
                    .send(data)
                    .await
                    .expect("The channel needs to be open");
                glib::timeout_future_seconds(1).await;
            }
        }
    ));

    let total_ram = Label::new(Some("Total RAM: loading"));
    let free_ram = Label::new(Some("Free RAM: loading"));
    let available_ram = Label::new(Some("Available RAM: loading"));

    page.append(&total_ram);
    page.append(&free_ram);
    page.append(&available_ram);

    glib::spawn_future_local(clone!(
        #[weak]
        total_ram,
        #[strong]
        receiver,
        async move {
            while let Ok(ram) = receiver.recv().await {
                total_ram.set_label(&format!(
                    "Total RAM: {}",
                    match ram {
                        Ok(ram) => format!("{} ({})", ram.total.round(2).unwrap(), ram.total),
                        Err(why) => format!("error: {why}"),
                    }
                ));
            }
        }
    ));
    glib::spawn_future_local(clone!(
        #[weak]
        free_ram,
        #[strong]
        receiver,
        async move {
            while let Ok(ram) = receiver.recv().await {
                free_ram.set_label(&format!(
                    "Free RAM: {}",
                    match ram {
                        Ok(ram) => format!("{} ({})", ram.free.round(2).unwrap(), ram.free),
                        Err(why) => format!("error: {why}"),
                    }
                ));
            }
        }
    ));
    glib::spawn_future_local(clone!(
        #[weak]
        available_ram,
        #[strong]
        receiver,
        async move {
            while let Ok(ram) = receiver.recv().await {
                available_ram.set_label(&format!(
                    "Available RAM: {}",
                    match ram {
                        Ok(ram) =>
                            format!("{} ({})", ram.available.round(2).unwrap(), ram.available),
                        Err(why) => format!("error: {why}"),
                    }
                ));
            }
        }
    ));

    page
}
