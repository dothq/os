use std::process::Command;

fn main() {
    let _ = Command::new("sh")
        .args(&["-c", "cd resources && glib-compile-resources resources.xml"])
        .output()
        .expect("failed to execute process");
    println!("Finished building resources");
}
