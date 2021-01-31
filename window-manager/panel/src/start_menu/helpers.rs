use freedesktop_entry_parser::parse_entry;
use std::{collections::HashMap, error::Error, fmt::Display, fs, process::Command};

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

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub enum Catagories {
    AudioVideo,
    Audio,
    Video,
    Development,
    Education,
    Game,
    Graphics,
    Network,
    Office,
    Settings,
    System,
    Utility,
    Other,
}

impl Catagories {
    pub fn from_vec(data: Vec<&str>) -> Vec<Self> {
        data.iter().map(|e| (*e).into()).collect()
    }
}

impl Display for Catagories {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Catagories::AudioVideo => "Multimedia",
                Catagories::Audio => "Audio",
                Catagories::Video => "Video",
                Catagories::Development => "Development",
                Catagories::Education => "Education",
                Catagories::Game => "Games",
                Catagories::Graphics => "Graphics",
                Catagories::Network => "Network",
                Catagories::Office => "Productivity",
                Catagories::Settings => "Settings",
                Catagories::System => "System",
                Catagories::Utility => "Utilities",
                _ => "Other",
            }
        )
    }
}

impl From<&str> for Catagories {
    fn from(name: &str) -> Self {
        match name {
            "AudioVideo" => Catagories::AudioVideo,
            "Audio" => Catagories::Audio,
            "Video" => Catagories::Video,
            "Development" => Catagories::Development,
            "Education" => Catagories::Education,
            "Game" => Catagories::Game,
            "Graphics" => Catagories::Graphics,
            "Network" => Catagories::Network,
            "Office" => Catagories::Office,
            "Settings" => Catagories::Settings,
            "System" => Catagories::System,
            "Utility" => Catagories::Utility,
            _ => Catagories::Other,
        }
    }
}

impl From<String> for Catagories {
    fn from(name: String) -> Self {
        match &*name {
            "AudioVideo" => Catagories::AudioVideo,
            "Audio" => Catagories::Audio,
            "Video" => Catagories::Video,
            "Development" => Catagories::Development,
            "Education" => Catagories::Education,
            "Game" => Catagories::Game,
            "Graphics" => Catagories::Graphics,
            "Network" => Catagories::Network,
            "Office" => Catagories::Office,
            "Settings" => Catagories::Settings,
            "System" => Catagories::System,
            "Utility" => Catagories::Utility,
            _ => Catagories::Other,
        }
    }
}

// App name, icon path, app exec path
pub type SystemApps = HashMap<Catagories, Vec<(String, String, String)>>;

pub fn get_system_apps() -> Result<SystemApps, Box<dyn Error>> {
    let mut apps = Vec::new();

    for desktop_file in fs::read_dir("/usr/share/applications")? {
        let path = desktop_file?.path();
        let file = parse_entry(&path)?;

        if file
            .section("Desktop Entry")
            .attr("Type")
            .unwrap_or("Invalid")
            == "Application"
        {
            // This is an application desktop file
            let name = file
                .section("Desktop Entry")
                .attr("Name")
                .unwrap_or("Unknown name")
                .to_string();

            let icon = file
                .section("Desktop Entry")
                .attr("Icon")
                .unwrap_or("applications-other")
                .to_string();

            let exec = file
                .section("Desktop Entry")
                .attr("Exec")
                .unwrap_or("/usr/bin/graviton %F")
                .to_string();

            let mut catagories = file
                .section("Desktop Entry")
                .attr("Categories")
                .unwrap_or("Other;")
                .to_string();
            catagories.pop();

            apps.push((name, icon, exec, catagories))
        }
    }

    let mut app_catagories = HashMap::new();

    apps.iter()
        .map(|(name, icon, exec, catagories)| {
            (
                name,
                icon,
                exec,
                catagories.split(';').collect::<Vec<&str>>(),
            )
        })
        .map(|(name, icon, exec, catagories)| (name, icon, exec, Catagories::from_vec(catagories)))
        .map(|mut e: (&String, &String, &String, Vec<Catagories>)| {
            e.3.dedup();
            e
        })
        .for_each(|e: (&String, &String, &String, Vec<Catagories>)| {
            for category in e.3 {
                if !app_catagories.contains_key(&category) {
                    app_catagories.insert(category.clone(), Vec::new());
                }

                app_catagories.get_mut(&category).unwrap().push((
                    e.0.to_string(),
                    e.1.to_string(),
                    e.2.to_string(),
                ));
            }
        });

    Ok(app_catagories)
}

#[cfg(test)]
mod tests {
    use super::get_running_gui_apps;

    #[test]
    fn basic_test() {
        get_running_gui_apps().unwrap();
    }
}
