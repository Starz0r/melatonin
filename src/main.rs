extern crate native_windows_derive as nwd;
extern crate native_windows_gui as nwg;

use nwd::NwgUi;
use nwg::NativeUi;
use sysinfo::{ProcessExt, System, SystemExt};

#[derive(Default, NwgUi)]
pub struct App {
    #[nwg_control(size: (300, 100), position: (300, 300), title: "melatonin", flags: "WINDOW|VISIBLE")]
    #[nwg_events(OnWindowClose: [App::close])]
    window: nwg::Window,

    #[nwg_layout(parent: window, spacing: 2)]
    grid: nwg::GridLayout,

    #[nwg_control(parent: window, position: (50, 5), size: (200, 300))]
    process_combobox: nwg::ComboBox<String>,
}

impl App {
    fn close(&self) {
        nwg::stop_thread_dispatch();
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // setup basic nwg stuff
    nwg::init().expect("Failed to init Native Windows GUI.");
    nwg::Font::set_global_family("Segoe UI").expect("Failed to set default font.");
    let app = App::build_ui(Default::default()).expect("Failed to build UI.");

    // get system information
    let sys = System::new_all();

    // enumerate all currently running processes
    for (pid, process) in sys.get_processes() {
        app.process_combobox.push(process.name().to_string());
        println!("{:?}", process.name())
    }

    // build and run the app
    nwg::dispatch_thread_events();
    Ok(())
}
