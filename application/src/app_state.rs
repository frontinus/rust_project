use druid::{Data, Lens, Rect, WindowId, WidgetId, Color};
use crate::custom_widget::{Alert, ShortcutKeys};

#[derive(Clone, PartialEq, Data)]
pub enum ImageModified {
    NotSavable,
    Savable,
}

#[derive(Clone, PartialEq, Data)]
pub enum State {
    Start,
    ScreenTaken(ImageModified),
}

#[derive(Clone, Data, Lens)]
pub struct AppState {
    pub rect: Rect,
    pub alpha: f64,
    pub extension: String,
    pub name: String,
    pub delay: f64,
    pub screen: String,
    #[data(eq)]
    pub state: State,
    #[data(ignore)]
    pub main_window_id: Option<WindowId>,
    #[data(ignore)]
    pub custom_zstack_id: Option<WidgetId>,
    #[data(ignore)]
    pub screenshot_id: Option<WidgetId>,
    #[data(ignore)]
    pub color: Option<Color>,
    #[data(ignore)]
    pub colors_window_opened: Option<WindowId>,
    #[data(ignore)]
    pub base_path: String,
    pub alert: Alert,
    pub shortcut_keys: ShortcutKeys,
    #[data(ignore)]
    pub text_field_zstack: bool,
    pub text_field: String,
    pub crop_screenshot_enabled: bool,
    pub rename_file_enabled: bool,
}
