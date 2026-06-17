use gpui::{App, Menu, MenuItem, SystemMenuType};

pub fn app_menus(cx: &mut App) -> Vec<Menu> {
    vec![Menu {
        name: "Ancestors".into(),
        // disabled: false,
        items: vec![
            MenuItem::os_submenu("Services", SystemMenuType::Services),
            MenuItem::separator(),
            MenuItem::action("Quit Ancestors", ancestors_actions::Quit),
        ],
    }]
}
