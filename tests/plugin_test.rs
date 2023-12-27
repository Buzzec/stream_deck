use std::env::current_dir;
use std::fs::OpenOptions;
use std::io::Write;
use stream_deck::manifest::action::state::State;
use stream_deck::manifest::action::Action;
use stream_deck::manifest::os::OS;
use stream_deck::manifest::software::Software;
use stream_deck::manifest::{Category, Manifest};

#[test]
fn plugin_test() {
    let manifest = Manifest {
        actions: vec![Action {
            uuid: "net.buzzec.test.cool".to_string(),
            icon: Some("assets/test".to_string()),
            name: "Test Action".to_string(),
            states: vec![State {
                image: "assets/test".to_string(),
                multi_action_image: None,
                name: None,
                title: None,
                show_title: None,
                title_color: None,
                title_alignment: None,
                font_family: None,
                font_style: None,
                font_size: None,
                font_underline: None,
            }],
            property_inspector_path: None,
            supported_in_multi_actions: None,
            tooltip: None,
            disable_caching: None,
            disable_automatic_states: None,
            visible_in_actions_list: None,
            user_title_enabled: None,
            controllers: vec![],
            encoder: None,
        }],
        author: "Buzzec".to_string(),
        code_path: "plugin_test.exe".to_string(),
        description: "A Test Plugin".to_string(),
        icon: "assets/test".to_string(),
        name: "Test Plugin".to_string(),
        version: "0.1.0".to_string(),
        sdk_version: 2,
        os: vec![OS::default_windows()],
        software: Software {
            minimum_version: "6".to_string(),
        },
        category: Some(Category {
            category: "Cool Category".to_string(),
            category_icon: "assets/test".to_string(),
        }),
        code_path_mac: None,
        code_path_windows: None,
        profiles: vec![],
        property_inspector_path: None,
        default_window_size: None,
        url: None,
        applications_to_monitor: None,
    };

    let manifest_json = serde_json::to_string_pretty(&manifest).unwrap();
    OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(
            current_dir()
                .unwrap()
                .join("tests")
                .join("net.buzzec.test.sdPlugin")
                .join("manifest.json"),
        )
        .unwrap()
        .write_all(manifest_json.as_bytes())
        .unwrap();
    println!("{}", manifest_json);
}
