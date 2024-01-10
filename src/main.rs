slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    let ui_handle = ui.as_weak();

    ui.on_calculate_leap_year(
        move |string| {
            let ui = ui_handle.upgrade().unwrap();
            if string.len() > 4 || string.len() < 1 {
                ui.set_results("Year must be between 1 and 4 digits.".into());
                return;
            }
            let year: i32 = match string.parse() {
                Ok(year) => {
                    if year < 1 {
                        ui.set_results("Year must be positive.".into());
                        return;
                    }
                    year
                },
                Err(_) => {
                    ui.set_results("Invalid year, Please try again.".into());
                    return;
                }
            };

            if year % 4 == 0 {
                if year % 100 == 0 {
                    if year % 400 == 0 {
                        ui.set_results(format!("{} is a leap year because it is divisible by 4, 100 and 400", year).into());
                        true
                    } else {
                        ui.set_results(format!("{} is not a leap year because it is divisible by 100 but not by 400", year).into());
                        false
                    }
                } else {
                    ui.set_results(format!("{} is a leap year because it is divisible by 4 but not by 100", year).into());
                    true
                }
            } else {
                ui.set_results(format!("{} is not a leap year because it is not divisible by 4", year).into());
                false
            };
        }
    );

    ui.run()
}
