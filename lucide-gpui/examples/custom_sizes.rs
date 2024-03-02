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

enum StarbucksIconSizes {
    Tall,
    Grande,
    Venti,
    Trenta,
}

impl Into<IconSize> for StarbucksIconSizes {
    fn into(self) -> IconSize {
        match self {
            StarbucksIconSizes::Tall => IconSize::Md,
            StarbucksIconSizes::Grande => IconSize::Lg,
            StarbucksIconSizes::Venti => IconSize::Px(50f32),
            StarbucksIconSizes::Trenta => IconSize::Px(200f32)
        }
    }

}

struct CustomSizes;

impl Render for CustomSizes {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .flex()
            .bg(Hsla::white())
            .size_full()
            .justify_center()
            .items_center()
            .children(
                vec![
                    Icon::icon_fish().size(StarbucksIconSizes::Tall),
                    Icon::icon_fish().size(StarbucksIconSizes::Grande),
                    Icon::icon_fish().size(StarbucksIconSizes::Venti),
                    Icon::icon_fish().size(StarbucksIconSizes::Trenta),
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