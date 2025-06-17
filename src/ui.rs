use bevy::prelude::*;
use game_over_menu::GameOverMenuPlugin;
use main_menu::MainMenuPlugin;

mod game_over_menu;
mod main_menu;

const BUTTON_COLOR: Srgba = Srgba::new(0.15, 0.15, 0.15, 1.0);
const HOVERED_BUTTON_COLOR: Srgba = Srgba::new(0.25, 0.25, 0.25, 1.0);
const PRESSED_BUTTON_COLOR: Srgba = Srgba::new(0.35, 0.75, 0.35, 1.0);

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MainMenuPlugin)
            .add_plugins(GameOverMenuPlugin)
            .add_systems(Update, interact_with_buttons);
    }
}

fn interact_with_buttons(mut button_query: Query<(&Interaction, &mut BackgroundColor), Changed<Interaction>>) {
    for (interaction, mut background_color) in button_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                *background_color = PRESSED_BUTTON_COLOR.into();
            }
            Interaction::Hovered => {
                *background_color = HOVERED_BUTTON_COLOR.into();
            }
            Interaction::None => {
                *background_color = BUTTON_COLOR.into();
            }
        }
    }
}

pub fn hub_node() -> Node {
    Node {
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::FlexStart,
        flex_direction: FlexDirection::Row,
        ..default()
    }
}

pub fn box_bundle() -> Node {
    Node {
        width: Val::Px(200.0),
        height: Val::Px(100.0),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        flex_direction: FlexDirection::Row,
        margin: UiRect::new(Val::Px(20.0), Val::Px(20.0), Val::Px(20.0), Val::Px(20.0)),
        //background_color: Srgba::new(0.30, 0.30, 0.30, 0.5).into(),
        ..default()
    }
}

pub fn container_node() -> Node {
    Node {
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        flex_direction: FlexDirection::Column,
        row_gap: Val::Px(8.0),
        ..default()
    }
}

pub fn spawn_title(spawner: &mut ChildSpawnerCommands<'_>, asset_server: &Res<AssetServer>) {
    spawner
        .spawn(Node {
            width: Val::Px(600.0),
            height: Val::Px(120.0),
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        })
        .with_children(|parent| {
            spawn_text(parent, asset_server, "Bevy Ball Game", 64.0);
        });
}

pub fn spawn_button(
    spawner: &mut ChildSpawnerCommands<'_>,
    asset_server: &Res<AssetServer>,
    text: &str,
    button: impl Component,
) {
    spawner
        .spawn((
            Button,
            Node {
                width: Val::Px(200.0),
                height: Val::Px(80.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(BUTTON_COLOR.into()),
            button,
        ))
        .with_children(|parent| {
            spawn_text(parent, asset_server, text, 32.0);
        });
}

pub fn spawn_text(spawner: &mut ChildSpawnerCommands<'_>, asset_server: &Res<AssetServer>, text: &str, font_size: f32) {
    spawner.spawn((
        Text::new(text),
        TextLayout {
            justify: JustifyText::Center,
            ..default()
        },
        TextColor(Color::WHITE),
        TextFont {
            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
            font_size: font_size,
            ..default()
        },
    ));
}

pub fn text_bundle(asset_server: &Res<AssetServer>, text: &str, font_size: f32) -> impl Bundle {
    (
        Text::new(text),
        TextLayout {
            justify: JustifyText::Center,
            ..default()
        },
        TextColor(Color::WHITE),
        TextFont {
            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
            font_size: font_size,
            ..default()
        },
    )
}
