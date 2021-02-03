use std::{error::Error, process::Command};

const GET_VOLUME: &str = "
# Get the index of the selected sink:
getsink() {
    pacmd list-sinks |
        awk '/index:/{i++} /* index:/{print i; exit}'
}

# Get the selected sink volume
getvolume() {
    pacmd list-sinks |
        awk '/^\\svolume:/{i++} i=='$(getsink)'{print $5; exit}'
}

getvolume";
const SET_VOLUME: &str = "pactl set-sink-volume @DEFAULT_SINK@";

fn command_to_str(command: &mut Command) -> Result<String, Box<dyn Error>> {
    Ok(std::str::from_utf8(&command.output()?.stdout)?.to_string())
}

pub fn get_volume() -> Result<u8, Box<dyn std::error::Error>> {
    let mut volume = command_to_str(Command::new("sh").args(vec!["-c", GET_VOLUME]))?;
    volume.pop();
    volume.pop();

    Ok(volume.parse()?)
}

pub fn set_volume(volume: u8) -> Result<(), Box<dyn std::error::Error>> {
    Command::new("sh")
        .args(vec!["-c", &format!("{} {}%", SET_VOLUME, volume)])
        .spawn()?;

    Ok(())
}
