use std::{error::Error, process::Command};

fn command_to_str(command: &mut Command) -> Result<String, Box<dyn Error>> {
    Ok(std::str::from_utf8(&command.output()?.stdout)?.to_string())
}

pub fn get_running_gui_apps() -> Result<Vec<String>, Box<dyn Error>> {
    let mut windows = Vec::new();

    for window in command_to_str(Command::new("wmctrl").arg("-l"))?.split('\n') {
        let win_id = window.split(' ').next().unwrap();
        let window_type =
            command_to_str(Command::new("xprop").args(vec!["-id", win_id, "_NET_WM_WINDOW_TYPE"]))?;

        if window_type.contains("_NET_WM_WINDOW_TYPE_NORMAL") {
            windows.push(win_id.to_string());
        }
    }

    Ok(windows)
}

#[cfg(test)]
mod tests {
    use super::get_running_gui_apps;

    #[test]
    fn basic_test() {
        get_running_gui_apps().unwrap();
    }
}
