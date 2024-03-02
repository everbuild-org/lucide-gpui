# lucide-gpui

This is a simple library to use lucide-icons in your gpui projects.

## Installation

Just add the following line to your `cargo.toml` and you are good to go.

```toml
lucide-gpui = "0.1.0"
```

## Examples

Take a look at the [examples](./lucide-gpui/examples) folder to see how to use this library.
If you just want to run an example, you can do so by running the following command:

> **Note**:
> If you clone the repository, you'll need to do so recursively to get the submodules (
> i.e. `git clone --recursive https://github.com/everbuild-org/lucide-gpui.git`
> or `git submodule update --init --recursive` if you already cloned the repository).

```sh
cargo run --example simple
```

## Usage

Start by importing the library and adding the hook to your AssetSource.

```rust
use lucide_gpui::*;
use gpui::AssetSource;

impl AssetSource for MyAssetSource {
    fn load(&self, path: &str) -> Result<Cow<'static, [u8]>> {
        if let Some(data) = lucide_gpui::asset_load_hook(&path) { return data; }

        // old code
    }

    fn list(&self, _path: &str) -> Result<Vec<SharedString>> {
        // old code
    }
}
```

Then, you can use the `Icon` struct to render the icons.

```rust
impl Render for MyComponent {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .child(
                    Icon::icon_fish()
                        .color(Hsla::red())
                        .size(IconSize::Lg),
            )
    }
}
```

Color & Size can also be omitted, in which case the default values will be used.
Or they can be custom Structs, as long as they implement the `Into<Hsla>` or
`Into<IconSize>` traits.

Look at the corresponding example for more details.

A list of all icons can be found [here](https://lucide.dev/icons/). Just take the name, 
add a `icon_` prefix, replace `-` with `_` and you are good to go.

## Contributing

Contributions are welcome. Just open an issue or a pull request and we can discuss it.
You can also reach out to us directly via discord [here](https://discord.gg/aF3J2X42cE).

## License

This project is licensed under the MIT License - see the [LICENSE](./LICENSE) file for details.

Lucide icons are licensed under the ISC License - see the [LICENSE](https://github.com/lucide-icons/lucide/blob/main/LICENSE) file for details.
