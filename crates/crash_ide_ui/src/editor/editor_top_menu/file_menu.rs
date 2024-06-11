use std::ops::Neg;
use bevy::prelude::*;
use bevy::ui::FocusPolicy;
use crash_ide_assets::{DefaultColors, DefaultFonts};
use crash_ide_widget::FocusNode;

pub(super) struct FileMenuPlugin;

impl Plugin for FileMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, open_file_menu)
        ;
    }
}

#[derive(Component)]
pub(super) struct FileMenu;

fn open_file_menu(
    mut commands: Commands,
    query: Query<(&Interaction, &Node, &Style, Entity), (With<FileMenu>, Changed<Interaction>)>
) {
    for (interaction, node, style, entity) in query.iter() {
        if *interaction != Interaction::Pressed {
            continue;
        }

        let size = node.size();

        commands.entity(entity).with_children(|parent| {
            parent.spawn((NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    top: Val::Px(size.y),
                    left: style.margin.left.neg(),
                    padding: UiRect::all(Val::Px(3.0)),
                    border: UiRect::all(Val::Px(1.0)),
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                background_color: BackgroundColor(DefaultColors::LEFT_MENU_BACKGROUND),
                border_color: BorderColor(Color::GRAY.with_a(0.1)),
                z_index: ZIndex::Global(1),
                focus_policy: FocusPolicy::Block,
                ..default()
            }, FocusNode, Interaction::None)).with_children(|parent| {
                parent.spawn(TextBundle {
                    text: Text::from_section("test text", TextStyle {
                        font: DefaultFonts::ROBOTO_REGULAR,
                        font_size: 18.0,
                        ..default()
                    }).with_no_wrap(),
                    ..default()
                });
            });
        });
    }
}

