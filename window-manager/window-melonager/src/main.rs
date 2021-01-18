#[macro_use]
extern crate penrose;

use penrose::{
    core::{
        bindings::KeyEventHandler, config::Config, helpers::index_selectors,
        manager::WindowManager, Layout, Selector,
    },
    logging_error_handler,
    xcb::new_xcb_backed_window_manager,
    Backward, Forward, Less, More,
};

use simplelog::{LevelFilter, SimpleLogger};

// Replace these with your preferred terminal and program launcher
const TERMINAL: &str = "xterm";
const LAUNCHER: &str = "dmenu_run";

fn main() -> penrose::Result<()> {
    // Initialise the logger (use LevelFilter::Debug to enable debug logging)
    if let Err(e) = SimpleLogger::init(LevelFilter::Info, simplelog::Config::default()) {
        panic!("unable to set log level: {}", e);
    };

    let mut config_builder = Config::default().builder();
    let config = config_builder
        .layouts(vec![Layout::floating("Floating")])
        .build()
        .expect("Failed to build config");

    let key_bindings = gen_keybindings! {
        // Program launchers
        "M-semicolon" => run_external!(LAUNCHER);
        "M-Return" => run_external!(TERMINAL);

        // Exit Penrose (important to remember this one!)
        "M-A-C-Escape" => run_internal!(exit);

        // client management
        "M-j" => run_internal!(cycle_client, Forward);
        "M-k" => run_internal!(cycle_client, Backward);
        "M-S-j" => run_internal!(drag_client, Forward);
        "M-S-k" => run_internal!(drag_client, Backward);
        "M-S-f" => run_internal!(toggle_client_fullscreen, &Selector::Focused);
        "M-S-q" => run_internal!(kill_client);

        // workspace management
        "M-Tab" => run_internal!(toggle_workspace);
        "M-A-period" => run_internal!(cycle_workspace, Forward);
        "M-A-comma" => run_internal!(cycle_workspace, Backward);

        // Layout management
        "M-grave" => run_internal!(cycle_layout, Forward);
        "M-S-grave" => run_internal!(cycle_layout, Backward);
        "M-A-Up" => run_internal!(update_max_main, More);
        "M-A-Down" => run_internal!(update_max_main, Less);
        "M-A-Right" => run_internal!(update_main_ratio, More);
        "M-A-Left" => run_internal!(update_main_ratio, Less);

        refmap [ config.ws_range() ] in {
            "M-{}" => focus_workspace [ index_selectors(config.workspaces().len()) ];
            "M-S-{}" => client_to_workspace [ index_selectors(config.workspaces().len()) ];
        };
    };

    let mut wm = new_xcb_backed_window_manager(config, vec![], logging_error_handler())?;
    wm.grab_keys_and_run(key_bindings, map! {})
}
