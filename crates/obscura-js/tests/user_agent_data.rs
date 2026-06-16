use obscura_js::runtime::ObscuraJsRuntime;

fn runtime_with_ua(ua: &str, platform: &str, ua_platform: &str, ua_platform_version: &str) -> ObscuraJsRuntime {
    let mut rt = ObscuraJsRuntime::new();
    rt.set_user_agent(ua);
    rt.set_platform(platform, ua_platform, ua_platform_version);
    rt.run_page_init();
    rt
}

#[test]
fn brands_follow_user_agent_major() {
    let mut rt = runtime_with_ua(
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/146.0.0.0 Safari/537.36",
        "Win32",
        "Windows",
        "15.0.0",
    );

    let value = rt
        .evaluate("JSON.stringify(navigator.userAgentData.brands)")
        .expect("brands should evaluate");
    let brands: serde_json::Value = serde_json::from_str(value.as_str().unwrap()).unwrap();

    assert!(brands
        .as_array()
        .unwrap()
        .iter()
        .any(|item| item["brand"] == "Google Chrome" && item["version"] == "146"));
}

#[test]
fn platform_follows_platform_globals() {
    let mut rt = runtime_with_ua(
        "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/145.0.0.0 Safari/537.36",
        "MacIntel",
        "macOS",
        "14.6.0",
    );

    let platform = rt
        .evaluate("navigator.userAgentData.platform")
        .expect("platform should evaluate");
    assert_eq!(platform.as_str(), Some("macOS"));
}
