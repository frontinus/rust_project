use druid::{WidgetId, LocalizedString};
use lazy_static::lazy_static;
use crate::app_state::AppState;

pub const STARTING_IMG_PATH: &'static str = "./src/images/starting_img.png";

lazy_static! {
    pub static ref SCREENSHOT_WIDGET_ID: WidgetId = WidgetId::next();
    pub static ref ZSTACK_ID: WidgetId = WidgetId::next();
}

pub const WINDOW_TITLE: LocalizedString<AppState> = LocalizedString::new("Screen Grabbing Application");
pub const X0: f64 = 0.;
pub const Y0: f64 = 0.;
pub const X1: f64 = 500.;
pub const Y1: f64 = 500.;

pub const BASE_PATH: &str = "./src/";
pub const BASE_PATH_SCREENSHOT: &str = "./src/screenshots/";
pub const BASE_PATH_FAVORITE_SHORTCUT: &str = "./src/shortcut/";
pub const PATH_FAVORITE_SHORTCUT: &str = "./src/shortcut/shortcut_settings.json";
