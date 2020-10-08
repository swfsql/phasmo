#![feature(or_patterns)]

pub mod app;
pub mod phasmo;

use self::app::AppData;
use druid::{AppLauncher, LocalizedString, WindowDesc};
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub fn wasm_main() {
    // This hook is necessary to get panic messages in the console
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    run()
}

pub fn run() {
    let main_window = WindowDesc::new(app::ui_builder).title(
        LocalizedString::new("app-window-title").with_placeholder("Phasmo Evidence Tracker"),
    );
    // Set our initial data
    let data = AppData::default();
    AppLauncher::with_window(main_window)
        .configure_env(|env: &mut _, _t: &AppData| {
            use druid::{theme, Color, Key};

            let base03 = Color::from_rgba32_u32(0x002b36ff);
            let base02 = Color::from_rgba32_u32(0x073642ff);
            let base01 = Color::from_rgba32_u32(0x586e75ff);
            let base00 = Color::from_rgba32_u32(0x657b83ff);

            let base0 = Color::from_rgba32_u32(0x839496ff);
            let base1 = Color::from_rgba32_u32(0x93a1a1ff);
            let base2 = Color::from_rgba32_u32(0xeee8d5ff);
            let base3 = Color::from_rgba32_u32(0xfdf6e3ff);

            let yellow = Color::from_rgba32_u32(0xb58900ff);
            let orange = Color::from_rgba32_u32(0xcb4b16ff);
            let red = Color::from_rgba32_u32(0xdc322fff);
            let magenta = Color::from_rgba32_u32(0xd33682ff);
            let violet = Color::from_rgba32_u32(0x6c71c4ff);
            let blue = Color::from_rgba32_u32(0x268bd2ff);
            let cyan = Color::from_rgba32_u32(0x2aa198ff);
            let green = Color::from_rgba32_u32(0x859900ff);

            // for e in env.get_all() {
            //     dbg!(e);
            // }
            env.set(theme::LABEL_COLOR, base02.clone());

            env.set(theme::PRIMARY_LIGHT, blue.clone());
            env.set(theme::PRIMARY_DARK, yellow.clone());

            env.set(theme::FOREGROUND_LIGHT, red.clone());
            env.set(theme::FOREGROUND_DARK, green.clone());

            env.set(theme::BACKGROUND_LIGHT, red.clone());
            env.set(theme::BACKGROUND_DARK, green.clone());

            env.set(theme::SELECTION_COLOR, red.clone());

            env.set(theme::BUTTON_LIGHT, base1.clone());
            env.set(theme::BUTTON_DARK, base1.clone());

            env.set(theme::WINDOW_BACKGROUND_COLOR, base1.clone());

            env.set(theme::TEXT_SIZE_NORMAL, 22.0f64);
            env.set(theme::TEXT_SIZE_LARGE, 30.0f64);

            // env.set("text_size_normal",base);
            // Float 19,
            // env.set("text_size_large",base);
            // Float 28,

            // println!("{:?}", )
        })
        .use_simple_logger()
        .launch(data)
        .expect("launch failed")
}
