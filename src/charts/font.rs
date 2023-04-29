use fontdue::Font;
use once_cell::sync::Lazy;
use snafu::{ResultExt, Snafu};
use std::{collections::HashMap, sync::Mutex, sync::MutexGuard};
use super::util::*;

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Error unable get lock: {source}"))]
    UnableGetLock {
        source: std::sync::PoisonError<MutexGuard<'static, HashMap<String, Font>>>,
    },
    #[snafu(display("Error font not found"))]
    FontNotFound {},
    #[snafu(display("Error parse font: {message}"))]
    ParseFont { message: String },
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

static GLOBAL_FONTS: Lazy<Mutex<HashMap<String, Font>>> = Lazy::new(|| {
    let mut m = HashMap::new();
    // 初始化roboto字体
    // 失败时直接出错
    let font = include_bytes!("../Roboto-Regular.ttf") as &[u8];
    let font = fontdue::Font::from_bytes(font, fontdue::FontSettings::default()).unwrap();
    m.insert("roboto".to_string(), font);

    Mutex::new(m)
});

pub fn add_font(name: &str, data: &[u8]) -> Result<()> {
    let font =
        fontdue::Font::from_bytes(data, fontdue::FontSettings::default()).map_err(|str| {
            Error::ParseFont {
                message: str.to_string(),
            }
        })?;
    let mut m = GLOBAL_FONTS.lock().context(UnableGetLockSnafu)?;
    m.insert(name.to_string(), font);
    Ok(())
}

pub fn get_font(name: &str) -> Result<Font> {
    let m = GLOBAL_FONTS.lock().context(UnableGetLockSnafu)?;
    if let Some(font) = m.get(name) {
        Ok(font.clone())
    } else {
        FontNotFoundSnafu {}.fail()
    }
}

pub fn measure_text(font: &Font, font_size: f64, text: &str) -> Box{
    let px = font_size as f32;
    let mut width = 0;
    let mut height = 0;
    for ch in text.chars() {
        let metrics = font.metrics(ch, px);
        width += metrics.width;
        if metrics.height > height {
            height = metrics.height;
        }
    }
    Box{
        right: width as f64,
        bottom: height as f64,
        ..Default::default()
    }
}
