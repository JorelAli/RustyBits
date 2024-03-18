#![windows_subsystem = "windows"]

extern crate native_windows_gui as nwg;
extern crate native_windows_derive as nwd;

use chrono::{Datelike, NaiveDate, Utc};

use nwd::NwgUi;
use nwg::NativeUi;

use std::{thread, time};

// Function to build a font with specific attributes
fn build_font() -> nwg::Font {
    let mut font = nwg::Font::default();
    
    // Configure the font using Font builder
    let _ = nwg::Font::builder()
        .size(38)
        .family("Arial")
        .build(&mut font);
    
    font
}

// Struct representing the Christmas application UI
#[derive(Default, NwgUi)]
pub struct ChristmasApplication {
    // Main window of the application
    #[nwg_control(size: (212, 217), title: "Christmas", flags: "WINDOW|VISIBLE")]
    #[nwg_events( OnInit: [ChristmasApplication::init] )]
    window: nwg::Window,

    // Label to display countdown until Christmas
    #[nwg_control(text: "Loading...", size: (212, 217), position: (10, 10), font: Some(&build_font()) )]
    label: nwg::Label,

    // Timer to trigger updates to the countdown label
    #[nwg_control]
    #[nwg_events( OnNotice: [ChristmasApplication::update_countdown] )]
    timer_notice: nwg::Notice,
}

impl ChristmasApplication {
    // Function to start the timer for updating the countdown
    fn start_timer(sender: nwg::NoticeSender) -> thread::JoinHandle<String> {
        thread::spawn(move || {
            loop {
                // Sleep for 500 milliseconds
                thread::sleep(time::Duration::from_millis(500));
                // Send notice to update the countdown label
                sender.notice();
            }
        })
    }

    // Initialization function for the UI
    fn init(&self) {
        // Start the timer to update the countdown label
        ChristmasApplication::start_timer(self.timer_notice.sender());
    }

    // Function to update the countdown label
    fn update_countdown(&self) {
        // Set the text of the label to the time remaining until Christmas
        self.label.set_text(generate_time_until_christmas().as_str());
    }
}

fn main() {
    // Initialize the native Windows GUI
    nwg::init().expect("Failed to init Native Windows GUI");

    // Build the UI for the Christmas application
    let _app = ChristmasApplication::build_ui(Default::default()).expect("Failed to build UI");

    // Dispatch thread events
    nwg::dispatch_thread_events();
}


// Function to generate a string representing the time remaining until Christmas
fn generate_time_until_christmas() -> std::string::String {
    // Get the current time in UTC
    let now = Utc::now().naive_utc();
    
    // Calculate the date and time for this year's Christmas
    let mut christmas = NaiveDate::from_ymd_opt(now.year(), 12, 25).unwrap().and_hms_opt(0, 0, 0).unwrap();
    
    // If Christmas has passed this year, calculate for next year's Christmas
    if now > christmas {
        christmas = christmas.with_year(now.year() + 1).unwrap();
    }

    // Calculate the time difference between now and Christmas
    let difference = christmas - now;
    
    // Calculate days, hours, minutes, and seconds remaining until Christmas
    let days = difference.num_days();
    let hours = difference.num_hours() - (days * 24);
    let minutes = difference.num_minutes() - (days * 24 * 60) - (hours * 60);
    let seconds = difference.num_seconds() - (days * 24 * 60 * 60) - (hours * 60 * 60) - (minutes * 60);
    
    // Format the result string
    format!("{} day{}\n{} hour{}\n{} minute{}\n{} second{}\nuntil Xmas!",
        days, if days == 1 { "" } else { "s" },
        hours, if hours == 1 { "" } else { "s" },
        minutes, if minutes == 1 { "" } else { "s" },
        seconds, if seconds == 1 { "" } else { "s" },
    )
}