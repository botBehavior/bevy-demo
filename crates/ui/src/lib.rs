use bevy::input::gamepad::{GamepadButtonType, GamepadEvent};
use bevy::prelude::*;
use bevy::ui::BorderRadius;
use bevy::window::{PrimaryWindow, WindowResized};
use threadweaver_core::components::*;
use threadweaver_core::prelude::*;
use threadweaver_core::shop::{ShopItem, UpgradeType, SHOP_ITEMS};
use threadweaver_gameplay::ShopPurchaseEvent;

pub struct ThreadweaverUiPlugin;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub struct UiSet;

#[derive(Resource, Clone)]
pub struct UiTheme {
    pub panel_background: Color,
    pub panel_border: Color,
    pub accent: Color,
    pub accent_soft: Color,
    pub text_primary: Color,
    pub text_muted: Color,
}

impl Default for UiTheme {
    fn default() -> Self {
        Self {
            panel_background: Color::srgba(0.05, 0.07, 0.1, 0.78),
            panel_border: Color::srgba(0.18, 0.24, 0.33, 0.9),
            accent: Color::srgba(0.33, 0.66, 0.93, 1.0),
            accent_soft: Color::srgba(0.33, 0.66, 0.93, 0.25),
            text_primary: Color::srgb(0.92, 0.95, 0.98),
            text_muted: Color::srgb(0.66, 0.72, 0.82),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum LayoutClass {
    Compact,
    Wide,
}

impl LayoutClass {
    const fn from_width(width: f32) -> Self {
        if width <= 720.0 {
            LayoutClass::Compact
        } else {
            LayoutClass::Wide
        }
    }
}

#[derive(Resource, Clone, Copy, PartialEq, Eq)]
pub struct UiLayout {
    pub class: LayoutClass,
}

impl Default for UiLayout {
    fn default() -> Self {
        Self {
            class: LayoutClass::Wide,
        }
    }
}

#[derive(Component)]
struct ShopGrid;

impl Plugin for ThreadweaverUiPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(UiTheme::default())
            .insert_resource(UiLayout::default())
            .add_systems(PostStartup, (setup_ui, initialize_layout).chain())
            .configure_sets(Update, UiSet)
            .add_systems(Update, update_layout_class.in_set(UiSet))
            .add_systems(
                Update,
                (
                    apply_layout_to_hud,
                    apply_layout_to_shop_modal,
                    apply_layout_to_shop_button,
                    apply_layout_to_shop_grid,
                    apply_layout_to_shop_cards,
                    update_hud,
                    update_health_bar,
                    update_shop_button_label,
                    sync_shop_visibility,
                    handle_shop_open_close,
                    handle_shop_purchases,
                    handle_keyboard_navigation,
                    handle_gamepad_navigation,
                    highlight_selected_card,
                )
                    .in_set(UiSet),
            );
    }
}

fn setup_ui(mut commands: Commands, theme: Res<UiTheme>, assets: Res<GameAssets>) {
    let font = assets.font_primary.clone();

    // HUD root
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    top: Val::Px(16.0),
                    left: Val::Px(16.0),
                    width: Val::Percent(28.0),
                    min_width: Val::Px(220.0),
                    padding: UiRect::axes(Val::Px(16.0), Val::Px(14.0)),
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(10.0),
                    ..Default::default()
                },
                background_color: theme.panel_background.into(),
                border_color: theme.panel_border.into(),
                border_radius: BorderRadius::all(Val::Px(12.0)),
                ..Default::default()
            },
            HudRoot,
        ))
        .with_children(|hud| {
            hud.spawn((
                TextBundle::from_section(
                    "Score 0000",
                    TextStyle {
                        font: font.clone(),
                        font_size: 24.0,
                        color: theme.text_primary,
                    },
                ),
                HudScore,
            ));

            hud.spawn((
                TextBundle::from_section(
                    "Best 0000",
                    TextStyle {
                        font: font.clone(),
                        font_size: 18.0,
                        color: theme.text_muted,
                    },
                ),
                HudStatus,
            ));

            hud.spawn(NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Px(18.0),
                    align_items: AlignItems::Stretch,
                    ..Default::default()
                },
                background_color: theme.accent_soft.into(),
                border_radius: BorderRadius::all(Val::Px(12.0)),
                ..Default::default()
            })
            .with_children(|bar| {
                bar.spawn((
                    NodeBundle {
                        style: Style {
                            width: Val::Percent(100.0),
                            height: Val::Percent(100.0),
                            ..Default::default()
                        },
                        background_color: theme.accent.into(),
                        border_radius: BorderRadius::all(Val::Px(12.0)),
                        ..Default::default()
                    },
                    HudHealthBar,
                ));
            });

            hud.spawn((
                TextBundle::from_section(
                    "Health 4 / 4",
                    TextStyle {
                        font: font.clone(),
                        font_size: 18.0,
                        color: theme.text_primary,
                    },
                ),
                HudHealth,
            ));

            hud.spawn((
                TextBundle::from_section(
                    "Shield Ready",
                    TextStyle {
                        font: font.clone(),
                        font_size: 18.0,
                        color: theme.text_muted,
                    },
                ),
                HudBuffs,
            ));

            hud.spawn((
                TextBundle::from_section(
                    "Currency 0",
                    TextStyle {
                        font: font.clone(),
                        font_size: 18.0,
                        color: theme.text_primary,
                    },
                ),
                HudCombo,
            ));
        });

    // Shop button bottom center
    commands
        .spawn((
            ButtonBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    bottom: Val::Px(24.0),
                    left: Val::Percent(50.0),
                    margin: UiRect::left(Val::Px(-70.0)),
                    width: Val::Px(140.0),
                    height: Val::Px(48.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
                background_color: theme.accent_soft.into(),
                border_color: theme.accent.into(),
                border_radius: BorderRadius::all(Val::Px(24.0)),
                ..Default::default()
            },
            ShopButton,
        ))
        .with_children(|button| {
            button.spawn(TextBundle::from_section(
                "Open Shop",
                TextStyle {
                    font: font.clone(),
                    font_size: 20.0,
                    color: theme.text_primary,
                },
            ));
        });

    // Shop modal overlay
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
                background_color: Color::srgba(0.0, 0.0, 0.0, 0.65).into(),
                visibility: Visibility::Hidden,
                ..Default::default()
            },
            ShopRoot,
        ))
        .with_children(|overlay| {
            overlay
                .spawn((
                    NodeBundle {
                        style: Style {
                            width: Val::Percent(70.0),
                            max_width: Val::Px(920.0),
                            min_width: Val::Px(320.0),
                            height: Val::Percent(70.0),
                            padding: UiRect::all(Val::Px(24.0)),
                            flex_direction: FlexDirection::Column,
                            row_gap: Val::Px(18.0),
                            ..Default::default()
                        },
                        background_color: theme.panel_background.into(),
                        border_color: theme.panel_border.into(),
                        border_radius: BorderRadius::all(Val::Px(18.0)),
                        ..Default::default()
                    },
                    ShopModal,
                ))
                .with_children(|modal| {
                    modal.spawn(TextBundle::from_section(
                        "Upgrade Loom",
                        TextStyle {
                            font: font.clone(),
                            font_size: 32.0,
                            color: theme.text_primary,
                        },
                    ));

                    modal
                        .spawn((
                            NodeBundle {
                                style: Style {
                                    width: Val::Percent(100.0),
                                    flex_wrap: FlexWrap::Wrap,
                                    column_gap: Val::Px(16.0),
                                    row_gap: Val::Px(16.0),
                                    justify_content: JustifyContent::FlexStart,
                                    ..Default::default()
                                },
                                ..Default::default()
                            },
                            ShopGrid,
                        ))
                        .with_children(|grid| {
                            for (index, item) in SHOP_ITEMS.iter().enumerate() {
                                spawn_shop_card(grid, item, index, &theme, &font);
                            }
                        });
                });
        });
}

fn spawn_shop_card(
    parent: &mut ChildBuilder,
    item: &ShopItem,
    index: usize,
    theme: &UiTheme,
    font: &Handle<Font>,
) {
    parent
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(30.0),
                    min_width: Val::Px(220.0),
                    max_width: Val::Px(280.0),
                    flex_grow: 1.0,
                    flex_shrink: 1.0,
                    padding: UiRect::all(Val::Px(16.0)),
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(12.0),
                    justify_content: JustifyContent::SpaceBetween,
                    ..Default::default()
                },
                background_color: theme.panel_background.into(),
                border_color: theme.panel_border.into(),
                border_radius: BorderRadius::all(Val::Px(14.0)),
                ..Default::default()
            },
            ShopCard { index },
        ))
        .with_children(|card| {
            card.spawn(TextBundle::from_section(
                format!("{} {}", item.upgrade.emoji(), item.upgrade.display_name()),
                TextStyle {
                    font: font.clone(),
                    font_size: 22.0,
                    color: theme.text_primary,
                },
            ));

            card.spawn((
                TextBundle::from_section(
                    item.description,
                    TextStyle {
                        font: font.clone(),
                        font_size: 16.0,
                        color: theme.text_muted,
                    },
                ),
                ShopDescriptionText(index),
            ));

            card.spawn((
                TextBundle::from_section(
                    "Level 0",
                    TextStyle {
                        font: font.clone(),
                        font_size: 16.0,
                        color: theme.text_muted,
                    },
                ),
                ShopLevelText(index),
            ));

            card.spawn((
                ButtonBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Px(40.0),
                        justify_content: JustifyContent::SpaceBetween,
                        align_items: AlignItems::Center,
                        padding: UiRect::axes(Val::Px(16.0), Val::Px(8.0)),
                        ..Default::default()
                    },
                    background_color: theme.accent_soft.into(),
                    border_color: theme.accent.into(),
                    border_radius: BorderRadius::all(Val::Px(12.0)),
                    ..Default::default()
                },
                ShopPurchaseButton { index },
            ))
            .with_children(|button| {
                button.spawn((
                    TextBundle::from_section(
                        "Buy",
                        TextStyle {
                            font: font.clone(),
                            font_size: 18.0,
                            color: theme.text_primary,
                        },
                    ),
                    ShopCostText(index),
                ));
            });
        });
}

fn initialize_layout(mut layout: ResMut<UiLayout>, windows: Query<&Window, With<PrimaryWindow>>) {
    if let Ok(window) = windows.get_single() {
        layout.class = LayoutClass::from_width(window.width());
        layout.set_changed();
    }
}

fn update_layout_class(
    mut layout: ResMut<UiLayout>,
    mut resize_events: EventReader<WindowResized>,
) {
    let mut maybe_new = layout.class;
    for event in resize_events.read() {
        maybe_new = LayoutClass::from_width(event.width);
    }

    if maybe_new != layout.class {
        layout.class = maybe_new;
        layout.set_changed();
    }
}

fn apply_layout_to_hud(layout: Res<UiLayout>, mut hud: Query<&mut Style, With<HudRoot>>) {
    if !layout.is_changed() {
        return;
    }

    if let Ok(mut style) = hud.get_single_mut() {
        match layout.class {
            LayoutClass::Wide => {
                style.top = Val::Px(16.0);
                style.left = Val::Px(16.0);
                style.width = Val::Percent(28.0);
                style.min_width = Val::Px(260.0);
            }
            LayoutClass::Compact => {
                style.top = Val::Px(12.0);
                style.left = Val::Percent(4.0);
                style.width = Val::Percent(92.0);
                style.min_width = Val::Px(0.0);
            }
        }
    }
}

fn apply_layout_to_shop_button(
    layout: Res<UiLayout>,
    mut button: Query<&mut Style, With<ShopButton>>,
) {
    if !layout.is_changed() {
        return;
    }

    if let Ok(mut style) = button.get_single_mut() {
        match layout.class {
            LayoutClass::Wide => {
                style.left = Val::Percent(50.0);
                style.width = Val::Px(140.0);
                style.margin.left = Val::Px(-70.0);
            }
            LayoutClass::Compact => {
                style.left = Val::Percent(20.0);
                style.width = Val::Percent(60.0);
                style.margin.left = Val::Px(0.0);
            }
        }
    }
}

fn apply_layout_to_shop_modal(
    layout: Res<UiLayout>,
    mut modal: Query<&mut Style, With<ShopModal>>,
) {
    if !layout.is_changed() {
        return;
    }

    if let Ok(mut style) = modal.get_single_mut() {
        match layout.class {
            LayoutClass::Wide => {
                style.width = Val::Percent(70.0);
                style.height = Val::Percent(70.0);
                style.max_width = Val::Px(920.0);
                style.min_width = Val::Px(320.0);
                style.max_height = Val::Auto;
                style.padding = UiRect::all(Val::Px(24.0));
            }
            LayoutClass::Compact => {
                style.width = Val::Percent(94.0);
                style.height = Val::Auto;
                style.max_width = Val::Px(640.0);
                style.min_width = Val::Px(0.0);
                style.max_height = Val::Percent(90.0);
                style.padding = UiRect::axes(Val::Px(18.0), Val::Px(22.0));
            }
        }
    }
}

fn apply_layout_to_shop_grid(layout: Res<UiLayout>, mut grid: Query<&mut Style, With<ShopGrid>>) {
    if !layout.is_changed() {
        return;
    }

    if let Ok(mut style) = grid.get_single_mut() {
        match layout.class {
            LayoutClass::Wide => {
                style.justify_content = JustifyContent::FlexStart;
                style.flex_wrap = FlexWrap::Wrap;
                style.column_gap = Val::Px(16.0);
                style.row_gap = Val::Px(16.0);
            }
            LayoutClass::Compact => {
                style.justify_content = JustifyContent::Center;
                style.flex_wrap = FlexWrap::Wrap;
                style.column_gap = Val::Px(12.0);
                style.row_gap = Val::Px(14.0);
            }
        }
    }
}

fn apply_layout_to_shop_cards(layout: Res<UiLayout>, mut cards: Query<&mut Style, With<ShopCard>>) {
    if !layout.is_changed() {
        return;
    }

    for mut style in &mut cards {
        match layout.class {
            LayoutClass::Wide => {
                style.width = Val::Percent(30.0);
                style.min_width = Val::Px(220.0);
                style.max_width = Val::Px(280.0);
                style.align_self = AlignSelf::Auto;
            }
            LayoutClass::Compact => {
                style.width = Val::Percent(100.0);
                style.min_width = Val::Px(0.0);
                style.max_width = Val::Percent(100.0);
                style.align_self = AlignSelf::Stretch;
            }
        }
    }
}

fn update_hud(
    score: Res<Score>,
    currency: Res<Currency>,
    player_health: Res<PlayerHealth>,
    shield: Res<ShieldState>,
    mut hud_text: ParamSet<(
        Query<&mut Text, With<HudScore>>,
        Query<&mut Text, With<HudStatus>>,
        Query<&mut Text, With<HudHealth>>,
        Query<&mut Text, With<HudBuffs>>,
        Query<&mut Text, With<HudCombo>>,
    )>,
) {
    if let Ok(mut text) = hud_text.p0().get_single_mut() {
        text.sections[0].value = format!("Score {:04}", score.current);
    }

    if let Ok(mut text) = hud_text.p1().get_single_mut() {
        text.sections[0].value = format!("Best {:04}", score.best);
    }

    if let Ok(mut text) = hud_text.p2().get_single_mut() {
        text.sections[0].value =
            format!("Health {} / {}", player_health.current, player_health.max);
    }

    if let Ok(mut text) = hud_text.p3().get_single_mut() {
        if shield.remaining > 0.0 {
            text.sections[0].value = format!("Shield {:0.1}s", shield.remaining);
        } else {
            text.sections[0].value = "Shield Ready".into();
        }
    }

    if let Ok(mut text) = hud_text.p4().get_single_mut() {
        text.sections[0].value = format!("Currency {}", currency.balance);
    }
}

fn update_health_bar(
    player_health: Res<PlayerHealth>,
    mut bar: Query<&mut Style, With<HudHealthBar>>,
) {
    if !player_health.is_changed() {
        return;
    }
    if let Ok(mut style) = bar.get_single_mut() {
        let percent = if player_health.max == 0 {
            0.0
        } else {
            player_health.current as f32 / player_health.max as f32 * 100.0
        };
        style.width = Val::Percent(percent.clamp(0.0, 100.0));
    }
}

fn update_shop_button_label(
    run_state: Res<RunState>,
    shop_state: Res<ShopState>,
    mut buttons: Query<(&mut Text, &mut BackgroundColor), (With<ShopButton>, Without<ShopModal>)>,
    theme: Res<UiTheme>,
) {
    if run_state.is_changed() || shop_state.is_changed() {
        if let Ok((mut text, mut background)) = buttons.get_single_mut() {
            text.sections[0].value = if shop_state.is_open {
                "Close Shop".into()
            } else {
                "Open Shop".into()
            };
            background.0 = if shop_state.is_open {
                theme.accent.into()
            } else {
                theme.accent_soft.into()
            };
        }
    }
}

fn sync_shop_visibility(
    shop_state: Res<ShopState>,
    mut overlay: Query<&mut Visibility, With<ShopRoot>>,
) {
    if !shop_state.is_changed() {
        return;
    }
    if let Ok(mut visibility) = overlay.get_single_mut() {
        *visibility = if shop_state.is_open {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }
}

fn handle_shop_open_close(
    mut interactions: Query<&Interaction, (Changed<Interaction>, With<ShopButton>)>,
    mut shop_state: ResMut<ShopState>,
    mut run_state: ResMut<RunState>,
) {
    for interaction in &mut interactions {
        if *interaction == Interaction::Pressed {
            if shop_state.is_open {
                run_state.resume();
            } else {
                run_state.pause();
                shop_state.selected_index = 0;
            }
            shop_state.toggle();
        }
    }
}

fn handle_shop_purchases(
    mut interactions: Query<(&Interaction, &ShopPurchaseButton), Changed<Interaction>>,
    mut shop_state: ResMut<ShopState>,
    currency: Res<Currency>,
    upgrades: Res<PurchasedUpgrades>,
    theme: Res<UiTheme>,
    mut cost_texts: Query<(&ShopCostText, &mut Text)>,
    mut level_texts: Query<(&ShopLevelText, &mut Text)>,
    mut writer: EventWriter<ShopPurchaseEvent>,
) {
    for (interaction, button) in &mut interactions {
        if *interaction == Interaction::Pressed {
            shop_state.selected_index = button.index;
            let item = &SHOP_ITEMS[button.index];
            let level = match item.upgrade {
                UpgradeType::MovementSpeed => upgrades.movement_speed_level,
                UpgradeType::MaxHealth => upgrades.max_health_level,
                UpgradeType::TrailDamage => upgrades.trail_damage_level,
                UpgradeType::ShieldDuration => upgrades.shield_level,
            };

            if level < item.max_level {
                let cost = item.cost_for_level(level);
                if currency.balance >= cost {
                    writer.send(ShopPurchaseEvent { item: item.upgrade });
                }
            }
        }
    }

    for (handle, mut text) in &mut cost_texts {
        let item = &SHOP_ITEMS[handle.0];
        let level = match item.upgrade {
            UpgradeType::MovementSpeed => upgrades.movement_speed_level,
            UpgradeType::MaxHealth => upgrades.max_health_level,
            UpgradeType::TrailDamage => upgrades.trail_damage_level,
            UpgradeType::ShieldDuration => upgrades.shield_level,
        };
        if level >= item.max_level {
            text.sections[0].value = "Maxed".into();
            text.sections[0].style.color = theme.text_muted;
        } else {
            let cost = item.cost_for_level(level);
            text.sections[0].value = format!("Buy ({})", cost);
            text.sections[0].style.color = if currency.balance >= cost {
                theme.text_primary
            } else {
                Color::srgb(0.85, 0.45, 0.45)
            };
        }
    }

    for (handle, mut text) in &mut level_texts {
        let item = &SHOP_ITEMS[handle.0];
        let level = match item.upgrade {
            UpgradeType::MovementSpeed => upgrades.movement_speed_level,
            UpgradeType::MaxHealth => upgrades.max_health_level,
            UpgradeType::TrailDamage => upgrades.trail_damage_level,
            UpgradeType::ShieldDuration => upgrades.shield_level,
        };
        text.sections[0].value = format!("Level {} / {}", level, item.max_level);
    }
}

fn handle_keyboard_navigation(
    keys: Res<ButtonInput<KeyCode>>,
    mut shop_state: ResMut<ShopState>,
    mut run_state: ResMut<RunState>,
    mut writer: EventWriter<ShopPurchaseEvent>,
) {
    if shop_state.is_open {
        if keys.just_pressed(KeyCode::ArrowRight) {
            shop_state.selected_index = (shop_state.selected_index + 1) % SHOP_ITEMS.len();
        }
        if keys.just_pressed(KeyCode::ArrowLeft) {
            shop_state.selected_index = shop_state
                .selected_index
                .checked_sub(1)
                .unwrap_or(SHOP_ITEMS.len() - 1);
        }
        if keys.just_pressed(KeyCode::ArrowDown) {
            shop_state.selected_index = (shop_state.selected_index + 2).min(SHOP_ITEMS.len() - 1);
        }
        if keys.just_pressed(KeyCode::ArrowUp) {
            shop_state.selected_index = shop_state.selected_index.saturating_sub(2);
        }
    }

    if keys.just_pressed(KeyCode::Escape) && shop_state.is_open {
        shop_state.is_open = false;
        run_state.resume();
    }

    if keys.just_pressed(KeyCode::Enter) && shop_state.is_open {
        let index = shop_state.selected_index.min(SHOP_ITEMS.len() - 1);
        writer.send(ShopPurchaseEvent {
            item: SHOP_ITEMS[index].upgrade,
        });
    }
}

fn handle_gamepad_navigation(
    mut events: EventReader<GamepadEvent>,
    mut shop_state: ResMut<ShopState>,
    mut run_state: ResMut<RunState>,
) {
    for event in events.read() {
        if let GamepadEvent::Button(button_event) = event {
            if button_event.value <= 0.5 {
                continue;
            }

            match button_event.button_type {
                GamepadButtonType::South => {
                    if shop_state.is_open {
                        shop_state.selected_index =
                            (shop_state.selected_index + 1) % SHOP_ITEMS.len();
                    }
                }
                GamepadButtonType::East => {
                    if shop_state.is_open {
                        shop_state.is_open = false;
                        run_state.resume();
                    } else {
                        shop_state.is_open = true;
                        run_state.pause();
                        shop_state.selected_index = 0;
                    }
                }
                _ => {}
            }
        }
    }
}

fn highlight_selected_card(
    shop_state: Res<ShopState>,
    mut cards: Query<(&ShopCard, &mut BorderColor, &mut BackgroundColor)>,
    theme: Res<UiTheme>,
) {
    if !shop_state.is_changed() {
        return;
    }

    for (card, mut border, mut background) in &mut cards {
        if card.index == shop_state.selected_index {
            border.0 = theme.accent;
            background.0 = theme.panel_background.with_alpha(0.95);
        } else {
            border.0 = theme.panel_border;
            background.0 = theme.panel_background;
        }
    }
}
