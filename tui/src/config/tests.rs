// use pretty_assertions::assert_eq;

use super::*;

// #[test]
// fn test_parse_style_default() {
//     let style = parse_style("");
//     assert_eq!(style, Style::default());
// }

// #[test]
// fn test_parse_style_foreground() {
//     let style = parse_style("red");
//     assert_eq!(style.fg, Some(Color::Indexed(1)));
// }

// #[test]
// fn test_parse_style_background() {
//     let style = parse_style("on blue");
//     assert_eq!(style.bg, Some(Color::Indexed(4)));
// }

// #[test]
// fn test_parse_style_modifiers() {
//     let style = parse_style("underline red on blue");
//     assert_eq!(style.fg, Some(Color::Indexed(1)));
//     assert_eq!(style.bg, Some(Color::Indexed(4)));
// }

// #[test]
// fn test_process_color_string() {
//     let (color, modifiers) = process_color_string("underline bold inverse gray");
//     assert_eq!(color, "gray");
//     assert!(modifiers.contains(Modifier::UNDERLINED));
//     assert!(modifiers.contains(Modifier::BOLD));
//     assert!(modifiers.contains(Modifier::REVERSED));
// }

// #[test]
// fn test_parse_color_rgb() {
//     let color = parse_color("rgb123");
//     let expected = 16 + 1 * 36 + 2 * 6 + 3;
//     assert_eq!(color, Some(Color::Indexed(expected)));
// }

// #[test]
// fn test_parse_color_unknown() {
//     let color = parse_color("unknown");
//     assert_eq!(color, None);
// }

#[test]
fn test_config() -> Result<()> {
    let c = Config::new()?;
    assert_eq!(
        c.keybindings
            .get(&Mode::Home)
            .unwrap()
            .get(&parse_key_sequence("<q>").unwrap_or_default())
            .unwrap(),
        &Action::Quit
    );
    Ok(())
}

#[test]
fn test_simple_keys() {
    assert_eq!(
        parse_key_event("a").unwrap(),
        KeyEvent::new(KeyCode::Char('a'), KeyModifiers::empty())
    );

    assert_eq!(
        parse_key_event("enter").unwrap(),
        KeyEvent::new(KeyCode::Enter, KeyModifiers::empty())
    );

    assert_eq!(
        parse_key_event("esc").unwrap(),
        KeyEvent::new(KeyCode::Esc, KeyModifiers::empty())
    );
}

#[test]
fn test_with_modifiers() {
    assert_eq!(
        parse_key_event("ctrl-a").unwrap(),
        KeyEvent::new(KeyCode::Char('a'), KeyModifiers::CONTROL)
    );

    assert_eq!(
        parse_key_event("alt-enter").unwrap(),
        KeyEvent::new(KeyCode::Enter, KeyModifiers::ALT)
    );

    assert_eq!(
        parse_key_event("shift-esc").unwrap(),
        KeyEvent::new(KeyCode::Esc, KeyModifiers::SHIFT)
    );
}

#[test]
fn test_multiple_modifiers() {
    assert_eq!(
        parse_key_event("ctrl-alt-a").unwrap(),
        KeyEvent::new(
            KeyCode::Char('a'),
            KeyModifiers::CONTROL | KeyModifiers::ALT
        )
    );

    assert_eq!(
        parse_key_event("ctrl-shift-enter").unwrap(),
        KeyEvent::new(KeyCode::Enter, KeyModifiers::CONTROL | KeyModifiers::SHIFT)
    );
}

#[test]
fn test_reverse_multiple_modifiers() {
    assert_eq!(
        key_event_to_string(&KeyEvent::new(
            KeyCode::Char('a'),
            KeyModifiers::CONTROL | KeyModifiers::ALT
        )),
        "ctrl-alt-a".to_string()
    );
}

#[test]
fn test_invalid_keys() {
    assert!(parse_key_event("invalid-key").is_err());
    assert!(parse_key_event("ctrl-invalid-key").is_err());
}

#[test]
fn test_case_insensitivity() {
    assert_eq!(
        parse_key_event("CTRL-a").unwrap(),
        KeyEvent::new(KeyCode::Char('a'), KeyModifiers::CONTROL)
    );

    assert_eq!(
        parse_key_event("AlT-eNtEr").unwrap(),
        KeyEvent::new(KeyCode::Enter, KeyModifiers::ALT)
    );
}
