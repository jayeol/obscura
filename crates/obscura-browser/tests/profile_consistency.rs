use obscura_browser::profiles::PROFILES;

#[test]
fn windows_profiles_keep_platform_fields_aligned() {
    for profile in PROFILES.iter().filter(|p| p.ua_platform == "Windows") {
        assert!(profile.user_agent.contains("Windows NT"));
        assert_eq!(profile.platform, "Win32");
        assert_eq!(profile.ua_platform, "Windows");
    }
}

#[test]
fn macos_profiles_keep_platform_fields_aligned() {
    for profile in PROFILES.iter().filter(|p| p.ua_platform == "macOS") {
        assert!(profile.user_agent.contains("Macintosh"));
        assert_eq!(profile.platform, "MacIntel");
        assert_eq!(profile.ua_platform, "macOS");
    }
}
