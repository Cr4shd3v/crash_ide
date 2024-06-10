use bevy::prelude::*;

#[derive(Component)]
pub struct GithubButton;

const URL: &'static str = "https://github.com/Cr4shd3v/editor";

pub(super) fn open_github_link(
    query: Query<&Interaction, (Changed<Interaction>, With<GithubButton>)>
) {
    for interaction in query.iter() {
        if *interaction != Interaction::Pressed {
            continue;
        }

        open::that(URL).unwrap();
    }
}