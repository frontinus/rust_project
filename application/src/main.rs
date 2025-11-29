mod custom_widget;
mod app_state;
mod constants;
mod ui_builder;

use std::collections::HashSet;
use std::str::FromStr;

use druid::{
    commands as sys_cmd, commands, AppDelegate, AppLauncher, Code, Command, Data,
    DelegateCtx, Env, Event, FileDialogOptions, Handled, LocalizedString, Menu, MenuItem, Screen, Size, Target,
    WindowDesc, WindowId, WidgetId,
};

use crate::app_state::{AppState, State};
use crate::constants::*;
use crate::ui_builder::{build_root_widget, build_screenshot_widget, build_about_us_widget, build_shortcut_keys_widget};
use crate::custom_widget::{
    read_from_file, write_to_file, Alert, ShortcutKeys, StateShortcutKeys, OverImages,
    SHOW_OVER_IMG, SHORTCUT_KEYS
};

fn main() {
    // Verify if the screenshot dir exists
    // Improved path handling: ensure directories exist using create_dir_all for robustness
    if let Err(e) = std::fs::create_dir_all(BASE_PATH) {
        eprintln!("Failed to create base directory {}: {}", BASE_PATH, e);
    }
    if let Err(e) = std::fs::create_dir_all(BASE_PATH_SCREENSHOT) {
        eprintln!("Failed to create screenshot directory {}: {}", BASE_PATH_SCREENSHOT, e);
    }
    if let Err(e) = std::fs::create_dir_all(BASE_PATH_FAVORITE_SHORTCUT) {
        eprintln!("Failed to create shortcut directory {}: {}", BASE_PATH_FAVORITE_SHORTCUT, e);
    }

    let default_shortcut: HashSet<Code> = HashSet::<Code>::from([Code::KeyB, Code::KeyA]);

    let main_window = WindowDesc::new(build_root_widget())
        .title("Welcome!")
        .menu(make_menu)
        .with_min_size((1200., 670.))
        //.set_window_state(WindowState::Maximized)
        .set_position((50., 20.));

    // create the initial app state
    let mut initial_state = AppState {
        rect: druid::Rect {
            x0: X0,
            y0: Y0,
            x1: X1,
            y1: Y1,
        },
        alpha: 100.0,
        extension: "png".to_string(),
        name: "".to_string(),
        delay: 0.0,
        screen: "0".to_string(),
        main_window_id: None,
        custom_zstack_id: Some(*ZSTACK_ID),
        screenshot_id: Some(*SCREENSHOT_WIDGET_ID),
        color: None,
        colors_window_opened: None,
        state: State::Start,
        base_path: BASE_PATH_SCREENSHOT.to_string(),
        alert: Alert {
            alert_visible: false,
            alert_message: "".to_string(),
        },
        shortcut_keys: ShortcutKeys {
            favorite_hot_keys: default_shortcut.clone(),
            pressed_hot_keys: HashSet::new(),
            state: StateShortcutKeys::NotBusy,
        },
        text_field_zstack: true,
        text_field: "".to_string(),
        crop_screenshot_enabled: false,
        rename_file_enabled: false,
    };

    // Reading and deserialization from file to set the favourite shortcut
    if let Some(deserialized) = read_from_file::<HashSet<String>>(PATH_FAVORITE_SHORTCUT) {
        let mut convert_code = HashSet::<Code>::new();
        for code in deserialized {
            match Code::from_str(code.as_str()) {
                Ok(code_deserialized) => {
                    convert_code.insert(code_deserialized);
                }
                Err(_) => {
                    convert_code = default_shortcut.clone();
                    break;
                }
            }
        }
        initial_state.shortcut_keys.favorite_hot_keys = convert_code;
    } else {
        initial_state.shortcut_keys.favorite_hot_keys = default_shortcut.clone();
    }

    let delegate = Delegate;

    // start the application
    AppLauncher::with_window(main_window)
        .delegate(delegate)
        .launch(initial_state)
        .expect("Failed to launch application");
}

struct Delegate;

impl AppDelegate<AppState> for Delegate {
    fn event(
        &mut self,
        ctx: &mut DelegateCtx,
        window_id: WindowId,
        event: Event,
        data: &mut AppState,
        _env: &Env,
    ) -> Option<Event> {
        match event.clone() {
            Event::KeyDown(key) => {
                data.shortcut_keys.pressed_hot_keys.insert(key.code);
            }
            Event::KeyUp(_) => {
                if data.shortcut_keys.state == StateShortcutKeys::SetFavoriteShortcut {
                    // check if there is a not available combination
                    if data.shortcut_keys.pressed_hot_keys
                        == HashSet::from([Code::ControlLeft, Code::KeyC])
                        || data.shortcut_keys.pressed_hot_keys == HashSet::from([Code::Escape])
                        || data.shortcut_keys.pressed_hot_keys
                            == HashSet::from([Code::ControlLeft, Code::KeyW])
                    {
                        // ctrl + c : this is reserved for the copy shortcut, Esc is reserved to close the subwindows and ctrl + w is reserved to close the main window
                        data.shortcut_keys.state = StateShortcutKeys::ShortcutNotAvailable;
                    } else {
                        data.shortcut_keys.favorite_hot_keys =
                            data.shortcut_keys.pressed_hot_keys.clone();
                        data.shortcut_keys.state = StateShortcutKeys::NotBusy;
                        data.shortcut_keys.pressed_hot_keys = HashSet::new();

                        let mut convert_code = HashSet::<String>::new();
                        for code in data.shortcut_keys.favorite_hot_keys.clone() {
                            convert_code.insert(code.to_string());
                        }

                        match write_to_file(PATH_FAVORITE_SHORTCUT, &convert_code) {
                            Ok(_) => data
                                .alert
                                .show_alert("Favorite Shortcut Saved Successfully!"),
                            Err(_) => data
                                .alert
                                .show_alert("Error during writing to the shortcut settings file!"),
                        }
                    }
                } else if data.shortcut_keys.pressed_hot_keys == HashSet::from([Code::Escape]) {
                    data.shortcut_keys.pressed_hot_keys = HashSet::new(); // clean map
                    data.shortcut_keys.state = StateShortcutKeys::NotBusy; // it has finished its job

                    // Key Escape has been pressed
                    if let Some(main_id) = data.main_window_id {
                        if let Err(e) = ctx.get_external_handle()
                            .submit_command(sys_cmd::SHOW_WINDOW, (), main_id) {
                                eprintln!("Error sending the event: {:?}", e);
                            }

                        if main_id != window_id {
                            ctx.submit_command(sys_cmd::CLOSE_WINDOW.to(Target::Window(window_id)));
                        }
                    }
                } else if data.shortcut_keys.pressed_hot_keys
                    == HashSet::from([Code::ControlLeft, Code::KeyW])
                {
                    data.shortcut_keys.state = StateShortcutKeys::NotBusy; // it has finished its job

                    // Keys ctrl + w has been pressed
                    if let Some(main_id) = data.main_window_id {
                        data.shortcut_keys.pressed_hot_keys = HashSet::new(); // clean map

                        if main_id == window_id {
                            ctx.submit_command(sys_cmd::CLOSE_WINDOW.to(Target::Window(main_id)));
                        }
                    } else {
                        ctx.submit_command(sys_cmd::CLOSE_WINDOW.to(Target::Window(window_id)));
                    }
                } else if data.shortcut_keys.pressed_hot_keys.len()
                    == data.shortcut_keys.favorite_hot_keys.len()
                    && data.shortcut_keys.pressed_hot_keys == data.shortcut_keys.favorite_hot_keys
                    && (data.shortcut_keys.state == StateShortcutKeys::NotBusy
                        || data.shortcut_keys.state == StateShortcutKeys::ShortcutNotAvailable)
                {
                    data.shortcut_keys.state = StateShortcutKeys::StartScreenGrabber; // started to capture the screen
                    data.shortcut_keys.pressed_hot_keys = HashSet::new(); // clean map

                    // start the screen grabber
                    data.main_window_id = Some(window_id);
                    ctx.submit_command(sys_cmd::HIDE_WINDOW.to(Target::Window(window_id)));
                    let mut monitors = Screen::get_monitors();
                    monitors.sort_by_key(|monitor| !monitor.is_primary());
                    let index: usize =
                        std::str::FromStr::from_str(data.screen.trim_start_matches(".")).unwrap_or(0);
                    if let Some(monitor) = monitors.get(index) {
                        ctx.new_window(
                            WindowDesc::new(build_screenshot_widget(index))
                                .title(WINDOW_TITLE)
                                .set_always_on_top(true)
                                .transparent(true)
                                .resizable(false)
                                .show_titlebar(false)
                                .window_size((monitor.virtual_rect().width(), monitor.virtual_rect().height()))
                                //.set_window_state(WindowState::Maximized)
                                .set_position(monitor.virtual_rect().origin()),
                        );
                        ctx.submit_command(
                            SHOW_OVER_IMG
                                .with((OverImages::Remove, None))
                                .to(Target::Widget(WidgetId::next())),
                        );
                    } else {
                        eprintln!("Monitor index {} out of bounds", index);
                    }
                }

                data.shortcut_keys.pressed_hot_keys = HashSet::new(); // clean map
            }
            _ => {}
        }

        Some(event)
    }

    fn command(
        &mut self,
        ctx: &mut DelegateCtx,
        _target: Target,
        cmd: &Command,
        data: &mut AppState,
        _env: &Env,
    ) -> Handled {
        // this gets the open file command when a directory has been selectioned
        if let Some(file_info) = cmd.get(commands::OPEN_FILE) {
            data.base_path = file_info
                .path
                .to_string_lossy()
                .to_string()
                .replace("\\", "/");
            data.base_path.push('/');
            return Handled::Yes;
        } else if cmd.is(sys_cmd::SHOW_ABOUT) {
            let mut monitors = Screen::get_monitors();
            monitors.sort_by_key(|monitor| !monitor.is_primary());
            let index: usize =
                std::str::FromStr::from_str(data.screen.trim_start_matches(".")).unwrap_or(0);
            if let Some(monitor) = monitors.get(index) {
                let window_aboutus = WindowDesc::new(build_about_us_widget())
                    .title(LocalizedString::new("About Us"))
                    .set_always_on_top(false)
                    .transparent(true)
                    .resizable(true)
                    .show_titlebar(true)
                    .window_size((400., 200.))
                    .set_position(monitor.virtual_rect().origin())
                    .with_min_size(Size::new(450., 300.));

                ctx.new_window(window_aboutus);
            }
        } else if cmd.is(SHORTCUT_KEYS) {
            let mut monitors = Screen::get_monitors();
            monitors.sort_by_key(|monitor| !monitor.is_primary());
            let index: usize =
                std::str::FromStr::from_str(data.screen.trim_start_matches(".")).unwrap_or(0);
            if let Some(monitor) = monitors.get(index) {
                let window_shortcut = WindowDesc::new(build_shortcut_keys_widget())
                    .title(LocalizedString::new("Shortcut Keys Configuration"))
                    .set_always_on_top(false)
                    .transparent(true)
                    .resizable(true)
                    .show_titlebar(true)
                    .window_size((500., 460.))
                    .set_position(monitor.virtual_rect().origin())
                    .with_min_size(Size::new(500., 450.));

                ctx.new_window(window_shortcut);
            }
        }
        Handled::No
    }
}

pub fn show_about<T: Data>() -> MenuItem<T> {
    MenuItem::new(LocalizedString::new("About Us")).command(sys_cmd::SHOW_ABOUT)
}

pub fn set_shortcutkeys<T: Data>() -> MenuItem<T> {
    MenuItem::new(LocalizedString::new("Shortcut Keys")).command(SHORTCUT_KEYS)
}
pub fn set_path<T: Data>() -> MenuItem<T> {
    MenuItem::new(LocalizedString::new("Set Path"))
        .command(commands::SHOW_OPEN_PANEL.with(FileDialogOptions::default().select_directories()))
}

fn make_menu(_window: Option<WindowId>, _data: &AppState, _env: &Env) -> Menu<AppState> {
    let base = Menu::empty();
    base.entry(Menu::new(LocalizedString::new("Edit")).entry(druid::platform_menus::common::copy()))
        .entry(
            Menu::new(LocalizedString::new("Settings"))
                .entry(show_about())
                .entry(set_path())
                .entry(set_shortcutkeys()),
        )
}
