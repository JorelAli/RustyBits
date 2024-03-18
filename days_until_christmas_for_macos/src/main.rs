use chrono::{Datelike, NaiveDate, Utc};
use std::{thread, time};

use cacao::appkit::{App, AppDelegate};
use cacao::appkit::window::Window;
use cacao::geometry::Rect;
use cacao::layout::Layout;
use cacao::notification_center::Dispatcher;
use cacao::text::{Font, Label};

#[derive(Default)]
struct ChristmasApplication {
    window: Window,
	label: Label
}

impl AppDelegate for ChristmasApplication {
    fn did_finish_launching(&self) {
		App::activate();

		let width = 215.;
		let height = 270.;
		self.window.set_minimum_content_size(width, height);
		self.window.set_content_size(width, height);
		self.window.set_title("Christmas");
		self.window.set_content_view(&self.label);
		self.window.show();

		self.label.set_translates_autoresizing_mask_into_constraints(true);
		self.label.set_frame(Rect::new(-35., 10., width, height));
		self.label.set_font(Font::system(38.));

		thread::spawn(move || {
			loop {
				App::<ChristmasApplication, String>::dispatch_main(generate_time_until_christmas());
				thread::sleep(time::Duration::from_millis(500))
			}
		});
    }

    fn should_terminate_after_last_window_closed(&self) -> bool {
        true
    }
}

impl Dispatcher for ChristmasApplication {
	type Message = String;

	fn on_ui_message(&self, _message: Self::Message) {
		self.label.set_text(_message);
	}
}

fn main() {
    App::new("com.hello.world", ChristmasApplication::default()).run();
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
