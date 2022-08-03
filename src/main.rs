use std::process::Command;

fn main() {
    // Running dash to dock
    println!("Running dash to dock..");
    
    let mut disable_dash_to_dock = Command::new("gnome-extensions");
    disable_dash_to_dock.arg("disable").arg("dash-to-dock@micxgx.gmail.com");
    disable_dash_to_dock.status().expect("failed to disable dash-to-dock");

    let mut run_dash_to_dock = Command::new("gnome-extensions");
    run_dash_to_dock.arg("enable").arg("dash-to-dock@micxgx.gmail.com");
    run_dash_to_dock.status().expect("Unable to start dash to dock");
}