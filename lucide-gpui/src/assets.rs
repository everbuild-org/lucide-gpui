use lucide_gpui_macros::match_icons;
use std::borrow::Cow;

pub fn asset_load_hook(path: &str) -> Option<anyhow::Result<Cow<'static, [u8]>>> {
    if !path.starts_with("lucide:") {
        return None;
    }

    let path = &path[7..];

    return Some(match_icons!(path));
}