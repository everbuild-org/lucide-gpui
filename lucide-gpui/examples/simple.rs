use std::borrow::Cow;
use gpui::*;
use lucide_gpui::{Icon, IconSize};

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

struct Simple;

impl Render for Simple {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .flex()
            .bg(Hsla::white())
            .size_full()
            .justify_center()
            .items_center()
            .children(
                vec![
                    Icon::icon_anvil()
                        .size(IconSize::Lg)
                        .color(Hsla::red()),
                    Icon::icon_award()
                        .size(IconSize::Md)
                        .color(Hsla::green()),
                    Icon::icon_dna()
                        .size(IconSize::Sm)
                        .color(Hsla::blue()),
                    Icon::icon_pie_chart()
                        .size(IconSize::Xs)
                        .color(Hsla::black()),
                ]
            )
    }
}

fn main() {
    App::new()
        .with_assets(AssetSource) // Needed
        .run(|cx: &mut AppContext| {
            cx.open_window(WindowOptions::default(), |cx| {
                cx.new_view(|_cx| Simple)
            });
        });
}