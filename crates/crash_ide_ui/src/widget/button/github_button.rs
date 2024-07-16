use bevy::prelude::*;

#[derive(Component)]
pub struct GithubButton;

const URL: &'static str = "https://github.com/Cr4shd3v/crash_ide";

pub(super) fn open_github_link(
    query: Query<&Interaction, (Changed<Interaction>, With<GithubButton>)>,
) {
    for interaction in query.iter() {
        if *interaction != Interaction::Pressed {
            continue;
        }

        open::that(URL).unwrap();
    }
}

#[derive(Component)]
pub struct GithubIssueButton;

pub(super) fn open_github_issue_link(
    query: Query<&Interaction, (Changed<Interaction>, With<GithubIssueButton>)>,
) {
    for interaction in query.iter() {
        if *interaction != Interaction::Pressed {
            continue;
        }

        open::that(format!("{}/issues/new", URL)).unwrap();
    }
}

