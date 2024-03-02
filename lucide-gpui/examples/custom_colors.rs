use std::borrow::Cow;
use gpui::*;
use lucide_gpui::Icon;

struct AssetSource;

impl gpui::AssetSource for AssetSource {
    fn load(&self, path: &str) -> Result<Cow<'static, [u8]>> {
        // Include the generated assets (revisit if there's progress with https://github.com/zed-industries/zed/issues/8713)
        if let Some(data) = lucide_gpui::asset_load_hook(&path) {
            return data;
        }

        return Err(anyhow::anyhow!("Asset not found: {}", path));
    }

    fn list(&self, _path: &str) -> Result<Vec<SharedString>> {
        Ok(vec![])
    }
}

enum NordColors {
    Aurora1,
    Aurora2,
    Aurora3,
    Aurora4,
    Aurora5,
}

impl Into<Hsla> for NordColors {
    fn into(self) -> Hsla {
        match self {
            NordColors::Aurora1 => Hsla::from(Rgba::try_from("#bf616a").unwrap()),
            NordColors::Aurora2 => Hsla::from(Rgba::try_from("#d08770").unwrap()),
            NordColors::Aurora3 => Hsla::from(Rgba::try_from("#ebcb8b").unwrap()),
            NordColors::Aurora4 => Hsla::from(Rgba::try_from("#a3be8c").unwrap()),
            NordColors::Aurora5 => Hsla::from(Rgba::try_from("#b48ead").unwrap()),
        }
    }
}

struct CustomSizes;

impl Render for CustomSizes {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .flex()
            .bg(Hsla::from(Rgba::try_from("#2e3440").unwrap()))
            .size_full()
            .justify_center()
            .items_center()
            .children(
                vec![
                    Icon::icon_fish().color(NordColors::Aurora1),
                    Icon::icon_fish().color(NordColors::Aurora2),
                    Icon::icon_fish().color(NordColors::Aurora3),
                    Icon::icon_fish().color(NordColors::Aurora4),
                    Icon::icon_fish().color(NordColors::Aurora5),
                ]
            )
    }
}

fn main() {
    App::new()
        .with_assets(AssetSource)
        .run(|cx: &mut AppContext| {
            cx.open_window(WindowOptions::default(), |cx| {
                cx.new_view(|_cx| CustomSizes)
            });
        });
}