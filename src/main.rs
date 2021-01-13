extern crate native_windows_derive as nwd;
extern crate native_windows_gui as nwg;

use std::cell::RefCell;
use std::os::windows::io::RawHandle;
use std::path::Path;
use std::ptr::null_mut as NULL;

use djin::{inject_dll, open_process};
use nwd::NwgUi;
use nwg::NativeUi;
use sysinfo::{ProcessExt, System, SystemExt};
use winapi::{
    shared::minwindef::FALSE,
    um::{
        processthreadsapi::OpenProcess,
        winnt::{
            PROCESS_CREATE_THREAD, PROCESS_QUERY_INFORMATION, PROCESS_VM_OPERATION,
            PROCESS_VM_READ, PROCESS_VM_WRITE,
        },
    },
};

#[derive(Default, NwgUi)]
pub struct App {
    #[nwg_control(size: (300, 100), position: (300, 300), title: "melatonin", flags: "WINDOW|VISIBLE")]
    #[nwg_events(OnWindowClose: [App::close])]
    window: nwg::Window,

    #[nwg_layout(parent: window, spacing: 2)]
    grid: nwg::GridLayout,

    #[nwg_control(parent: window, position: (50, 5), size: (200, 300))]
    process_combobox: nwg::ComboBox<String>,

    #[nwg_control(parent: window, text: "Inject Process", size: (150, 32), position: (76, 48))]
    #[nwg_events(OnButtonClick: [App::inject_process])]
    inject_button: nwg::Button,

    processes: RefCell<Vec<usize>>,
}

impl App {
    fn close(&self) {
        nwg::stop_thread_dispatch();
    }

    fn inject_process(&self) {
        let hwnd: RawHandle;

        unsafe {
            hwnd = OpenProcess(
                PROCESS_CREATE_THREAD
                    | PROCESS_QUERY_INFORMATION
                    | PROCESS_VM_OPERATION
                    | PROCESS_VM_READ
                    | PROCESS_VM_WRITE,
                FALSE,
                self.processes.borrow()[self.process_combobox.selection().unwrap()] as u32,
            )
            .cast();
        }

        if hwnd != NULL() {
            let err = inject_dll(hwnd.cast(), ".\\SleepMargin32.dll", b"my_library_init");
            match err {
                Ok(_) => nwg::simple_message(
                    "Injection Status",
                    "Injection was successful! Sleep margin was set to 1.",
                ),
                Err(code) => nwg::error_message(
                    "Injection Status",
                    format!("Injection failed! Error code: {}", code).as_str(),
                ),
            };
        } else {
            nwg::error_message("Process not found", "The target process could not be used or found due to the current permission level.");
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // setup basic nwg stuff
    nwg::init().expect("Failed to init Native Windows GUI.");
    nwg::Font::set_global_family("Segoe UI").expect("Failed to set default font.");
    let app = App::build_ui(Default::default()).expect("Failed to build UI.");

    // get system information
    let sys = System::new_all();

    // notify user
    nwg::simple_message("Notice", "This application is used to inject a DLL into a running process remotely. It is most recommended that you only use this when there aren't any active process running an anti-cheat in the background, as it might get you banned. Please use this application responibly.");

    // enumerate all currently running processes
    for (pid, process) in sys.get_processes() {
        app.process_combobox
            .push(format!("[{}] {}", pid, process.name()));
        app.processes.borrow_mut().push(*pid);
    }

    // run the app
    nwg::dispatch_thread_events();
    Ok(())
}
