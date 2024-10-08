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
            background_color: BackgroundColor(DefaultColors::DEFAULT_BACKGROUND),
            border_color: BorderColor(DefaultColors::GRAY.with_alpha(0.1)),
            z_index: ZIndex::Global(1),
            focus_policy: FocusPolicy::Block,
            ..default()
        }, FocusNode, ContextMenu, Interaction::None)
    }

    pub fn new_at_cursor(cursor_pos: Vec2) -> impl Bundle {
        Self::new_top(cursor_pos.y, Val::Px(cursor_pos.x))
    }
}