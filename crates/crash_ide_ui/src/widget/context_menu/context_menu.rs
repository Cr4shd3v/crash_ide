use bevy::prelude::*;
use bevy::ui::FocusPolicy;
use crash_ide_assets::DefaultColors;
use crash_ide_widget::FocusNode;

/// Component that marks a context menu.
#[derive(Component)]
pub struct ContextMenu;

impl ContextMenu {
    /// Creates a new Context Menu for the top menu bar.
    pub fn new_top(top: f32, left: Val) -> impl Bundle {
        (NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                top: Val::Px(top),
                left,
                border: UiRect::all(Val::Px(1.0)),
                flex_direction: FlexDirection::Column,
                ..default()
            },
            background_color: BackgroundColor(DefaultColors::LEFT_MENU_BACKGROUND),
            border_color: BorderColor(Color::GRAY.with_a(0.1)),
            z_index: ZIndex::Global(1),
            focus_policy: FocusPolicy::Block,
            ..default()
        }, FocusNode, Interaction::None)
    }
}