use babelz::{I18n, _rust_i18n_available_locales, _rust_i18n_translate};
use rust_i18n::t;

fn main() {
    rust_i18n::i18n!(backend = I18n::new());

    println!("available_locales: {:#?}", rust_i18n::available_locales!());
    println!("");

    println!("en: {}", t!("welcome", locale = "en"));
    println!("zh-CN: {}", t!("welcome", locale = "zh-CN"));
    println!("");

    println!("en: {}", t!("hello", locale = "en"));
    println!("zh-CN: {}", t!("hello", locale = "zh-CN"));
    println!("zh-HK: {}", t!("hello", locale = "zh-HK"));
    println!("");

    println!(
        "en: {}",
        t!("messages.hello", locale = "en", name => "Bulbbot")
    );
    println!(
        "zh-CN: {}",
        t!("messages.hello", locale = "zh-CN", name => "Bulbbot")
    );
    println!(
        "zh-HK: {}",
        t!("messages.hello", locale = "zh-HK", name => "Bulbbot",)
    );
}

/*
[
    "en",
    "zh-CN",
    "zh-HK",
]
Rust I18n Example for share locales in entire workspace.
Rust I18n 示例，用于在整个工作区中共享本地化。
Hello world
你好世界
Hello world
Hello, Bulbbot
你好，Bulbbot
Hello, Bulbbot
*/
