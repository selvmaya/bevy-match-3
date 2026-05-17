//! Main menu screen.
//!
//! Spawned on entering [`ScreenState::MainMenu`] and torn down on exit.
//! The only interaction is the Play button, which transitions to
//! [`ScreenState::InGame`].

use bevy::prelude::*;
use bevy::{
    input_focus::{
        AutoFocus, InputFocus, InputFocusVisible,
        directional_navigation::DirectionalNavigationPlugin,
    },
    math::{CompassOctant, Dir2},
    picking::events::{Out, Over, Pointer},
    ui::auto_directional_navigation::{AutoDirectionalNavigation, AutoDirectionalNavigator},
    ui_widgets::{Activate, Button},
};

use crate::GameState;
use crate::GameSystems;
use crate::ScreenState;
use crate::game_logic::Score;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DirectionalNavigationPlugin)
            .insert_resource(InputFocusVisible(true))
            .add_systems(OnEnter(ScreenState::MainMenu), setup_menu)
            .add_systems(OnEnter(GameState::GameOver), setup_game_over_screen)
            .add_systems(
                Update,
                (navigate_menu_with_arrows, update_menu_focus_indicator).in_set(GameSystems::Logic),
            );
    }
}

#[derive(Component, Default, Clone)]
struct MenuButton;

/// Stores the normal and hovered background colors for a menu button.
#[derive(Component, Default, Clone)]
struct MenuButtonColors {
    normal: Color,
    hovered: Color,
}

const BUTTON_NORMAL: Color = Color::hsl(224.0, 0.72, 0.38);
const BUTTON_HOVERED: Color = Color::hsl(224.0, 0.72, 0.50);
const QUIT_BUTTON_NORMAL: Color = Color::hsl(0.0, 0.60, 0.35);
const QUIT_BUTTON_HOVERED: Color = Color::hsl(0.0, 0.60, 0.47);
const BUTTON_FOCUS_BORDER: Color = Color::hsl(52.0, 0.30, 0.68);
const GAME_OVER_OVERLAY_COLOR: Color = Color::srgba(0.0, 0.0, 0.0, 0.75);
const GAME_OVER_SCORE_COLOR: Color = Color::hsl(54.0, 0.91, 0.70);

const TITLE_FONT_SIZE: f32 = 72.0;
const GAME_OVER_HEADING_FONT_SIZE: f32 = 64.0;
const GAME_OVER_SCORE_FONT_SIZE: f32 = 42.0;
const BUTTON_FONT_SIZE: f32 = 36.0;

const MENU_ROW_GAP: f32 = 28.0;
const BUTTON_PADDING_H: f32 = 52.0;
const BUTTON_PADDING_V: f32 = 18.0;
const BUTTON_TOP_MARGIN: f32 = 16.0;
const BUTTON_BORDER_WIDTH: f32 = 2.0;
const BUTTON_BORDER_RADIUS: f32 = 8.0;

fn menu_button(label: &str, normal: Color, hovered: Color) -> impl Scene {
    bsn! {
        MenuButton
        Button
        AutoDirectionalNavigation
        MenuButtonColors { normal: normal, hovered: hovered }
        Node {
            padding: UiRect::axes(px(BUTTON_PADDING_H), px(BUTTON_PADDING_V)),
            border: px(BUTTON_BORDER_WIDTH),
            border_radius: BorderRadius::all(px(BUTTON_BORDER_RADIUS)),
        }
        BackgroundColor(normal)
        BorderColor::all(Color::NONE)
        on(|ev: On<Pointer<Over>>, mut q: Query<(&MenuButtonColors, &mut BackgroundColor)>| {
            if let Ok((colors, mut bg)) = q.get_mut(ev.entity) {
                *bg = BackgroundColor(colors.hovered);
            }
        })
        on(|ev: On<Pointer<Out>>, mut q: Query<(&MenuButtonColors, &mut BackgroundColor)>| {
            if let Ok((colors, mut bg)) = q.get_mut(ev.entity) {
                *bg = BackgroundColor(colors.normal);
            }
        })
        Children [(
            Text(label)
            TextFont { font_size: px(BUTTON_FONT_SIZE) }
            TextColor(Color::WHITE)
        )]
    }
}

fn setup_menu(mut commands: Commands) {
    commands.spawn_scene(bsn! {
        DespawnOnExit::<ScreenState>(ScreenState::MainMenu)
        Node {
            width: percent(100),
            height: percent(100),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            row_gap: px(MENU_ROW_GAP),
        }
        Children [
            (
                Text("Bevy Match-3")
                TextFont { font_size: px(TITLE_FONT_SIZE) }
                TextColor(Color::WHITE)
            ),
            (
                menu_button("Play", BUTTON_NORMAL, BUTTON_HOVERED)
                AutoFocus
                Node { margin: UiRect::top(px(BUTTON_TOP_MARGIN)) }
                on(|_ev: On<Activate>, mut next_state: ResMut<NextState<ScreenState>>| {
                    next_state.set(ScreenState::InGame);
                })
            ),
            (
                menu_button("Quit", QUIT_BUTTON_NORMAL, QUIT_BUTTON_HOVERED)
                on(|_ev: On<Activate>, mut app_exit: MessageWriter<AppExit>| {
                    app_exit.write(AppExit::Success);
                })
            ),
        ]
    });
}

fn setup_game_over_screen(mut commands: Commands, score: Res<Score>) {
    let score_text = score.to_string();
    commands.spawn_scene(bsn! {
        DespawnOnExit::<GameState>(GameState::GameOver)
        Node {
            width: percent(100),
            height: percent(100),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            row_gap: px(MENU_ROW_GAP),
        }
        BackgroundColor(GAME_OVER_OVERLAY_COLOR)
        Children [
            (
                Text("No More Moves!")
                TextFont { font_size: px(GAME_OVER_HEADING_FONT_SIZE) }
                TextColor(Color::WHITE)
            ),
            (
                Text({score_text})
                TextFont { font_size: px(GAME_OVER_SCORE_FONT_SIZE) }
                TextColor(GAME_OVER_SCORE_COLOR)
            ),
            (
                menu_button("Main Menu", BUTTON_NORMAL, BUTTON_HOVERED)
                AutoFocus
                Node { margin: UiRect::top(px(BUTTON_TOP_MARGIN)) }
                on(|_ev: On<Activate>, mut next_state: ResMut<NextState<ScreenState>>| {
                    next_state.set(ScreenState::MainMenu);
                })
            ),
        ]
    });
}

fn navigate_menu_with_arrows(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut focus_visible: ResMut<InputFocusVisible>,
    mut auto_directional_navigator: AutoDirectionalNavigator,
    menu_buttons: Query<(), With<MenuButton>>,
) {
    if menu_buttons.is_empty() {
        return;
    }

    let net_east_west = keyboard.just_pressed(KeyCode::ArrowRight) as i8
        - keyboard.just_pressed(KeyCode::ArrowLeft) as i8;
    let net_north_south = keyboard.just_pressed(KeyCode::ArrowUp) as i8
        - keyboard.just_pressed(KeyCode::ArrowDown) as i8;

    if let Ok(direction) = Dir2::from_xy(net_east_west as f32, net_north_south as f32) {
        focus_visible.0 = true;
        let _ = auto_directional_navigator.navigate(CompassOctant::from(direction));
    }
}

fn update_menu_focus_indicator(
    input_focus: Res<InputFocus>,
    focus_visible: Res<InputFocusVisible>,
    mut buttons: Query<(Entity, &mut BorderColor), With<MenuButton>>,
) {
    for (entity, mut border_color) in &mut buttons {
        if focus_visible.0 && input_focus.get() == Some(entity) {
            *border_color = BorderColor::all(BUTTON_FOCUS_BORDER);
        } else {
            *border_color = BorderColor::all(Color::NONE);
        }
    }
}
