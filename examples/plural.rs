use bevy::prelude::*;
use bevy_intl::{I18n, I18nPlugin, LanguageAppExt};

#[derive(Resource, Default)]
struct ItemCount(usize);

#[derive(Component)]
struct CountText;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(I18nPlugin::default())
        .init_resource::<ItemCount>()
        .add_systems(Startup, setup_ui)
        .add_systems(
            Update,
            (
                language_switcher,
                change_count,
                update_text.after(change_count),
            ),
        )
        .run();
}

fn setup_ui(mut commands: Commands, i18n: Res<I18n>, count: Res<ItemCount>) {
    commands.spawn(Camera2d);

    let label = i18n.translation("plural").t_with_plural("items", count.0);

    commands.spawn((
        CountText,
        Text::new(label),
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(5.0),
            right: Val::Px(5.0),
            ..default()
        },
    ));
}

fn language_switcher(input: Res<ButtonInput<KeyCode>>, mut i18n: ResMut<I18n>) {
    if input.just_pressed(KeyCode::F1) {
        i18n.set_lang("en");
    }
    if input.just_pressed(KeyCode::F2) {
        i18n.set_lang("fr");
    }
}

fn change_count(input: Res<ButtonInput<KeyCode>>, mut count: ResMut<ItemCount>) {
    if input.just_pressed(KeyCode::ArrowUp) {
        count.0 += 1;
    }
    if input.just_pressed(KeyCode::ArrowDown) {
        count.0 = count.0.saturating_sub(1);
    }
}

fn update_text(
    i18n: Res<I18n>,
    count: Res<ItemCount>,
    mut query: Query<&mut Text, With<CountText>>,
) {
    if !count.is_changed() && !i18n.is_changed() {
        return;
    }
    let label = i18n.translation("plural").t_with_plural("items", count.0);
    for mut text in &mut query {
        **text = label.clone();
    }
}
