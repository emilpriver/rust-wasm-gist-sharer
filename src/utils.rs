use cfg_if::cfg_if;
use syntect::highlighting::ThemeSet;
use syntect::html::highlighted_html_for_string;
use syntect::parsing::SyntaxSet;

cfg_if! {
    // https://github.com/rustwasm/console_error_panic_hook#readme
    if #[cfg(feature = "console_error_panic_hook")] {
        extern crate console_error_panic_hook;
        pub use self::console_error_panic_hook::set_once as set_panic_hook;
    } else {
        #[inline]
        pub fn set_panic_hook() {}
    }
}

pub fn get_code_template() -> String {
    r#"
      <html>
        <head>
            <title> {lang} code </title>
        </head>
        <body>
            {code}
        </body>
    </html>
    "#
    .to_string()
}

pub fn syntax_highlight_code(code: String, lang: String) -> String {
    let ss = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();

    let theme = &ts.themes["Solarized (dark)"];
    let sr = ss.find_syntax_by_extension(lang.as_str()).unwrap();

    let syntax_code = highlighted_html_for_string(code.as_str(), &ss, &sr, theme).unwrap();

    get_code_template()
        .replace("{code}", syntax_code.as_str())
        .replace("{lang}", lang.as_str())
        .to_string()
}
