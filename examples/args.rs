use bevy::prelude::*;
use bevy_intl::{I18n, I18nPlugin, LanguageAppExt};

#[derive(Component)]
struct AttackText;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins)
        .add_plugins(I18nPlugin::default())
        .add_systems(Startup, setup_ui)
        .add_systems(Update, (language_switcher, update_text));
    app.set_lang_i18n("en");
    app.set_fallback_lang("en");
    app.run();
}

fn setup_ui(mut commands: Commands, i18n: Res<I18n>) {
    commands.spawn(Camera2d);

    let label = i18n
        .translation("args")
        .t_with_arg("attack_msg", &[&"Dragon", &42, &"Hero"]);

    commands.spawn((
        AttackText,
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

fn update_text(i18n: Res<I18n>, mut query: Query<&mut Text, With<AttackText>>) {
    if !i18n.is_changed() {
        return;
    }
    let label = i18n
        .translation("args")
        .t_with_arg("attack_msg", &[&"Dragon", &42, &"Hero"]);
    for mut text in &mut query {
        **text = label.clone();
    }
}
