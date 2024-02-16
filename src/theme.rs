use catppuccin::{Flavour, FlavourColours};
use gpui::{AppContext, Global, Hsla, Rgba, SharedString};

fn color_to_hsla(color: catppuccin::Colour) -> Hsla {
    Rgba {
        r: color.0 as f32 / 255.0,
        g: color.1 as f32 / 255.0,
        b: color.2 as f32 / 255.0,
        a: 1.0,
    }
    .into()
}

#[derive(Debug)]
pub struct Theme {
    pub font_sans: SharedString,
    pub font_mono: SharedString,
    pub crust: Hsla,
    pub text: Hsla,
    pub base: Hsla,
    pub mantle: Hsla,
    pub green: Hsla,
    pub red: Hsla,
    pub blue: Hsla,
}

impl Global for Theme {}

impl From<FlavourColours> for Theme {
    fn from(colors: FlavourColours) -> Self {
        Theme {
            font_sans: "Inter".into(),
            font_mono: "JetBrains Mono".into(),
            crust: color_to_hsla(colors.crust),
            text: color_to_hsla(colors.text),
            base: color_to_hsla(colors.base),
            mantle: color_to_hsla(colors.mantle),
            green: color_to_hsla(colors.green),
            red: color_to_hsla(colors.red),
            blue: color_to_hsla(colors.blue),
        }
    }
}

impl Theme {
    fn new() -> Self {
        Self::from(Flavour::Latte.colours())
    }

    pub fn init(cx: &mut AppContext) {
        cx.set_global(Theme::new())
    }

    pub fn change(flavour: Flavour, cx: &mut AppContext) {
        cx.set_global(Self::from(flavour.colours()));
        cx.refresh();
    }
}
