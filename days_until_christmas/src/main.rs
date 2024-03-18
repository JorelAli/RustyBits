#![windows_subsystem = "windows"]

extern crate native_windows_gui as nwg;
extern crate native_windows_derive as nwd;

use chrono::{Datelike, NaiveDate, Utc};

use nwd::NwgUi;
use nwg::NativeUi;

use std::{thread, time};

fn build_font() -> nwg::Font {
	let mut font = nwg::Font::default();

	let _ = nwg::Font::builder()
		.size(38)
		.family("Arial")
		.build(&mut font);

	font
}

#[derive(Default, NwgUi)]
pub struct ChristmasApplication {
	#[nwg_control(size: (212, 217), title: "Christmas", flags: "WINDOW|VISIBLE")]
	#[nwg_events( OnInit: [ChristmasApplication::init] )]
	window: nwg::Window,

	#[nwg_control(text: "Loading...", size: (212, 217), position: (10, 10), font: Some(&build_font()) )]
	label: nwg::Label,

	#[nwg_control]
	#[nwg_events( OnNotice: [ChristmasApplication::update_countdown] )]
	timer_notice: nwg::Notice,
}

impl ChristmasApplication {

	fn start_timer(sender: nwg::NoticeSender) -> thread::JoinHandle<String> {
		thread::spawn(move || {
			loop {
				thread::sleep(time::Duration::from_millis(500));
				sender.notice();
			}
		})
	}

	fn init(&self) {
		ChristmasApplication::start_timer(self.timer_notice.sender());
	}

	fn update_countdown(&self) {
		self.label.set_text(generate_time_until_christmas().as_str());
	}

}

fn main() {
	nwg::init().expect("Failed to init Native Windows GUI");

	let _app = ChristmasApplication::build_ui(Default::default()).expect("Failed to build UI");

	nwg::dispatch_thread_events();
}


fn generate_time_until_christmas() -> std::string::String {
	let now = Utc::now().naive_utc();
	let mut christmas = NaiveDate::from_ymd_opt(now.year(), 12, 25).unwrap().and_hms_opt(0, 0, 0).unwrap();
	if now > christmas {
		christmas = christmas.with_year(now.year() + 1).unwrap();
	}

	let difference = christmas - now;
	let days = difference.num_days();
	let hours = difference.num_hours() - (days * 24);
	let minutes = difference.num_minutes() - (days * 24 * 60) - (hours * 60);
	let seconds = difference.num_seconds() - (days * 24 * 60 * 60) - (hours * 60 * 60) - (minutes * 60);
	format!("{} day{}\n{} hour{}\n{} minute{}\n{} second{}\nuntil Xmas!",
		days, if days == 1 { "" } else { "s" },
		hours, if hours == 1 { "" } else { "s" },
		minutes, if minutes == 1 { "" } else { "s" },
		seconds, if seconds == 1 { "" } else { "s" },
	)
}