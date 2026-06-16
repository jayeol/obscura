use obscura_js::runtime::ObscuraJsRuntime;

#[test]
fn navigator_language_defaults_to_en_us() {
    let mut rt = ObscuraJsRuntime::new();
    rt.run_page_init();

    let language = rt
        .evaluate("navigator.language")
        .expect("language should evaluate");
    let languages = rt
        .evaluate("JSON.stringify(navigator.languages)")
        .expect("languages should evaluate");

    assert_eq!(language.as_str(), Some("en-US"));
    assert_eq!(languages.as_str(), Some("[\"en-US\",\"en\"]"));
}

#[test]
fn navigator_languages_follow_runtime_locale_global() {
    let mut rt = ObscuraJsRuntime::new();
    rt.execute_script("<set-locale>", "globalThis.__obscura_locale='fr-FR';")
        .expect("locale global should set");
    rt.run_page_init();

    let language = rt
        .evaluate("navigator.language")
        .expect("language should evaluate");
    let languages = rt
        .evaluate("JSON.stringify(navigator.languages)")
        .expect("languages should evaluate");

    assert_eq!(language.as_str(), Some("fr-FR"));
    assert_eq!(languages.as_str(), Some("[\"fr-FR\",\"fr\"]"));
}
