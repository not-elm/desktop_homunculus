use bevy::prelude::*;
use bevy::tasks::IoTaskPool;
use bevy_tray_icon::menu::accelerator::Accelerator;
use bevy_tray_icon::plugin::TrayIconPlugin;
use bevy_tray_icon::plugin::menu_event::MenuEvent;
use bevy_tray_icon::resource::{Menu, MenuItem, TrayIcon};
use homunculus_api::prelude::{ApiReactor, WebviewApi};
use homunculus_core::prelude::*;
use homunculus_deno::prelude::DenoScriptHandle;

pub(crate) struct ModSystemMenuPlugin;

impl Plugin for ModSystemMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<RequestUpdateSystemMenus>()
            .init_resource::<MenuEventItems>()
            .add_plugins(TrayIconPlugin)
            .add_systems(Startup, setup_tray)
            .add_systems(
                Update,
                (
                    push_menus.run_if(on_event::<RequestUpdateSystemMenus>),
                    read_menu_event,
                ),
            );
    }
}

#[derive(Event, Debug)]
pub struct RequestUpdateSystemMenus {
    pub mod_name: String,
    pub menus: Vec<ModSystemMenu>,
}

impl RequestUpdateSystemMenus {
    fn to_menu(&self) -> Menu {
        let items = self
            .menus
            .iter()
            .map(|menu| to_menu_item(&self.mod_name, menu))
            .collect::<Vec<_>>();
        Menu::new(items)
    }

    fn to_menu_event_items(&self) -> Vec<MenuEventItem> {
        self.menus
            .iter()
            .flat_map(|menu| to_menu_event_item(&self.mod_name, menu))
            .collect()
    }
}

fn to_menu_item(mod_name: &str, menu: &ModSystemMenu) -> MenuItem {
    match menu {
        ModSystemMenu::Common(m) => MenuItem::common(
            format!("{}::{}", mod_name, m.text),
            m.text.clone(),
            true,
            m.shortcut
                .as_ref()
                .map(|s| Accelerator::new(s.modifiers, s.key)),
        ),
        ModSystemMenu::Sub(sub) => {
            let submenus = sub
                .menus
                .iter()
                .map(|m| to_menu_item(mod_name, m))
                .collect();
            MenuItem::submenu(
                format!("{}::{}", mod_name, sub.title),
                sub.title.clone(),
                true,
                Menu::new(submenus),
            )
        }
    }
}

fn to_menu_event_item(mod_name: &str, menu: &ModSystemMenu) -> Vec<MenuEventItem> {
    match menu {
        ModSystemMenu::Common(m) => vec![MenuEventItem {
            id: format!("{}::{}", mod_name, m.text),
            script: m.script.clone(),
            webview: m.webview.clone(),
        }],
        ModSystemMenu::Sub(sub) => sub
            .menus
            .iter()
            .flat_map(|m| to_menu_event_item(mod_name, m))
            .collect(),
    }
}

#[derive(Resource, Debug, Deref, DerefMut, Default)]
struct MenuEventItems(pub Vec<MenuEventItem>);

#[derive(Debug, Clone)]
struct MenuEventItem {
    pub id: String,
    pub script: Option<ModModuleSource>,
    pub webview: Option<WebviewOpenOptions>,
}

fn setup_tray(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(TrayIcon {
        icon: Some(asset_server.load("icons/tray.png")),
        tooltip: Some("desktop_homunculus".to_string()),
        menu: Menu::default(),
        show_menu_on_left_click: true,
    });
}

fn push_menus(
    mut er: EventReader<RequestUpdateSystemMenus>,
    mut tray: ResMut<TrayIcon>,
    mut mod_menus: ResMut<MenuEventItems>,
) {
    for e in er.read() {
        mod_menus.0.extend(e.to_menu_event_items());
        tray.menu.extend_from_slice(e.to_menu().as_slice());
    }
}

fn read_menu_event(
    mut er: EventReader<MenuEvent>,
    mut commands: Commands,
    api: Res<ApiReactor>,
    asset_server: Res<AssetServer>,
    menus: Res<MenuEventItems>,
) {
    for e in er.read() {
        let Some(menu) = menus.iter().find(|m| m.id.as_str() == e.id.0.as_str()) else {
            continue;
        };
        call_script(&mut commands, menu, &asset_server);
        open_webview(menu, &api);
    }
}

fn call_script(commands: &mut Commands, menu: &MenuEventItem, asset_server: &AssetServer) {
    if let Some(source) = menu.script.as_ref() {
        commands.spawn(DenoScriptHandle(asset_server.load(source.to_string())));
    }
}

fn open_webview(menu: &MenuEventItem, api: &ApiReactor) {
    if let Some(options) = menu.webview.clone() {
        let api = api.clone();
        IoTaskPool::get()
            .spawn(async move {
                let webview_api = WebviewApi::from(api);
                webview_api
                    .open(options)
                    .await
                    .output_log_if_error("Webview");
            })
            .detach();
    };
}
