use ferrix_lib::cpu::Processors;
use ferrix_lib::drm::Video;
use ferrix_lib::ram::RAM;
use ferrix_lib::sys::Sys;
use gtk::gio::{Menu, MenuItem};
use gtk::glib::clone;
use gtk::{
    Application, ApplicationWindow, Button, HeaderBar, MenuButton, ScrolledWindow, Stack,
    StackSidebar, glib,
};
use gtk::{Box, Label, prelude::*};
use std::fmt::Display;

const APP_ID: &str = "com.mskrasnov.Ferrix";
// const LOGO: &[u8] = include_bytes!("../data/icons/hicolor/scalable/apps/com.mskrasnov.Ferrix.svg");

#[derive(Debug, Clone, Copy, Default)]
pub enum Page {
    #[default]
    Dashboard,
    Processors,
    Memory,
    Storage,
    DMI,
    System,
}

impl Page {
    pub const ALL: [Self; 6] = [
        Self::Dashboard,
        Self::Processors,
        Self::Memory,
        Self::Storage,
        Self::DMI,
        Self::System,
    ];
}

impl Display for Page {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Dashboard => "Dashboard",
                Self::Processors => "Processors",
                Self::Memory => "Memory",
                Self::Storage => "Storage",
                Self::DMI => "DMI Tables",
                Self::System => "Operating System",
            }
        )
    }
}

#[derive(Debug)]
pub struct AppState {
    pub page: Page,
    pub proc: Option<Processors>,
    pub ram: Option<RAM>,
    pub system: Option<Sys>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            page: Page::default(),
            proc: None,
            ram: None,
            system: None,
        }
    }
}
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

    stack.add_titled(&todo_widget(), Some("dash"), "Dashboard");
    stack.add_titled(&proc_page(), Some("proc"), "Processors");
    stack.add_titled(&ram_page(), Some("Memory"), "Memory");
    stack.add_titled(&todo_widget(), Some("storage"), "Storage");
    stack.add_titled(&todo_widget(), Some("dmi"), "DMI Tables");
    stack.add_titled(&drm_page(), Some("drm"), "Video");
    stack.add_titled(&sys_page(), Some("os"), "Operating system");

    hbox.append(&sidebar);
    hbox.append(&stack);

    sidebar.set_hexpand(false);
    sidebar.set_halign(gtk::Align::Start);
    stack.set_hexpand(true);

    let win = ApplicationWindow::builder()
        .application(app)
        .title("Ferrix")
        .titlebar(&create_hdr_bar())
        .default_height(380)
        .default_width(510)
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

fn todo_widget() -> Box {
    let layout = Box::new(gtk::Orientation::Vertical, 10);
    layout.set_vexpand(false);
    let title = Label::builder()
        .label("TODO")
        .css_classes(["large-title"])
        .build();
    let subtitle = Label::builder()
        .label("This functionality is still in development.")
        .css_classes(["subtitle"])
        .build();

    layout.append(&title);
    layout.append(&subtitle);

    layout.set_halign(gtk::Align::Center);
    layout.set_valign(gtk::Align::Center);

    layout
}

fn create_hdr_bar() -> HeaderBar {
    let hdr = HeaderBar::new();
    let menu = MenuButton::builder()
        .icon_name("open-menu-symbolic")
        .menu_model(&create_menu_btn())
        .tooltip_text("Menu")
        .css_classes(["header-button", "text-button"])
        .build();
    hdr.pack_end(&menu);

    let export_btn = Button::new();
    export_btn.add_css_class("header-button");
    export_btn.set_icon_name("document-save-symbolic");
    export_btn.set_tooltip_text(Some("Export"));
    hdr.pack_start(&export_btn);

    hdr
}

fn create_menu_btn() -> Menu {
    let menu = Menu::new();
    let section = Menu::new();
    section.append_item(&MenuItem::new(Some("Update"), None));
    section.append_item(&MenuItem::new(Some("Export"), None));
    section.append_item(&MenuItem::new(Some("About Ferrix"), None));

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
                    Ok(proc) => format!("{:#?}", &proc.entries),
                    Err(why) => format!("Error: {why}"),
                });
            }
        }
    ));

    ScrolledWindow::builder().child(&page).build()
}

fn sys_page() -> ScrolledWindow {
    let page = Box::new(gtk::Orientation::Vertical, 0);
    let (sender, receiver) = async_channel::bounded(1);
    glib::spawn_future_local(clone!(
        #[strong]
        sender,
        async move {
            loop {
                let data = Sys::new();
                sender
                    .send(data)
                    .await
                    .expect("The channel needs to be open");
                glib::timeout_future_seconds(10).await;
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
            while let Ok(sys) = receiver.recv().await {
                proc.set_label(&match sys {
                    Ok(sys) => format!("{:#?}", &sys),
                    Err(why) => format!("Error: {why}"),
                });
            }
        }
    ));

    ScrolledWindow::builder().child(&page).build()
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

fn drm_page() -> ScrolledWindow {
    let page = Box::new(gtk::Orientation::Vertical, 0);
    let (sender, receiver) = async_channel::bounded(1);
    glib::spawn_future_local(clone!(
        #[strong]
        sender,
        async move {
            loop {
                let data = Video::new();
                sender
                    .send(data)
                    .await
                    .expect("The channel needs to be open");
                glib::timeout_future_seconds(1).await;
            }
        }
    ));
    let vid = Label::new(None);
    page.append(&vid);

    glib::spawn_future_local(clone!(
        #[weak]
        vid,
        #[strong]
        receiver,
        async move {
            while let Ok(vid_data) = receiver.recv().await {
                vid.set_label(&match vid_data {
                    Ok(vid_data) => format!("{:#?}", &vid_data.devices),
                    Err(why) => format!("Error: {why}"),
                });
            }
        }
    ));

    ScrolledWindow::builder().child(&page).build()
}
