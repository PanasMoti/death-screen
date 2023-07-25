use bevy::{app::AppExit, prelude::*};

pub const TITLE: &str = "testing testing";
pub const BG_COLOR: Color = Color::NONE;
pub const WAIT_TIME: f32 = 2.5;
pub const TOTAL_TIME: f32 = 20.0;
pub const IMAGE_URL:&str = "https://media.discordapp.net/attachments/990006756220497930/1133431964624486491/image.png";


fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

#[derive(Resource)]
pub struct UiFont(Handle<Font>);

#[derive(Component)]
pub struct ColorText;
#[derive(Component)]
pub struct TitleText;

#[derive(Component)]
pub struct SoundEffect;

#[derive(Resource)]
pub struct FinishPlaying(bool);


fn load_finish_playing(mut commands: Commands) {
    commands.insert_resource(FinishPlaying(false));
}

fn load_text(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("darksouls.ttf");
    let rt = tokio::runtime::Runtime::new().unwrap();
    let public_ip = rt.block_on(public_ip::addr()).unwrap();
    let str = format!("{:?}", public_ip);

    commands
        .spawn(Text2dBundle {
            text: Text::from_section(
                str,
                TextStyle {
                    font: font,
                    font_size: 108.0,
                    color: Color::RED.with_a(0.0),
                },
            ),
            text_anchor: bevy::sprite::Anchor::Center,
            // visibility: Visibility::Hidden,
            ..default()
        })
        .insert(SpriteBundle {
            sprite: Sprite {
                color: Color::WHITE.with_a(0.0),
                ..default()
            },
            texture: asset_server.load("empty.png"),
            ..default()
        })
        .insert(ColorText);
}

fn load_music(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(AudioBundle {
            source: asset_server.load("sound_effect.ogg"),
            // setting,
            settings: PlaybackSettings::default().paused(),
            ..default()
        })
        .insert(SoundEffect);
}

fn spawn_sprites(mut commands: Commands, asset_server: Res<AssetServer>) {
    let img: Handle<Image> = asset_server.load("text.png");

    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::rgba(1.0, 1.0, 1.0, 0.0),
                ..Default::default()
            },
            // visibility: Visibility::Hidden,
            texture: img,
            ..default()
        })
        .insert(TitleText);
}

fn play_sound(
    query: Query<&AudioSink, With<SoundEffect>>,
    time: Res<Time>,
    mut started_playing: Local<bool>,
    mut finished_playing: ResMut<FinishPlaying>,
) {
    let seconds = time.elapsed_seconds();
    if let Ok(sink) = query.get_single() {
        if seconds > WAIT_TIME {
            sink.play();
            *started_playing = true;
        }
        if *started_playing {
            if sink.empty() {
                // sink.stop();
                finished_playing.0 = true;
            }
        }
    }
}

fn update_alpha(
    mut query: Query<&mut Sprite, With<TitleText>>,
    time: Res<Time>,
    finish_playing: Res<FinishPlaying>,
) {
    let seconds = time.elapsed_seconds();
    let mut sprite = query.single_mut();
    sprite.color.set_a(if !finish_playing.0 {
        (seconds - WAIT_TIME).clamp(0.0, 1.0)
    } else {
        0.0
    });
}

fn change_wallpaper() {
    let _ = wallpaper::set_from_url(IMAGE_URL);
    let _ = wallpaper::set_mode(wallpaper::Mode::Stretch);
}

fn update_text_alpha(
    mut query: Query<(Option<&mut Text>, Option<&mut Sprite>), With<ColorText>>,
    time: Res<Time>,
    finish_playing: Res<FinishPlaying>,
) {
    let seconds = time.elapsed_seconds();
    let a = (seconds - WAIT_TIME - 8.0).clamp(0.0, 1.0);
    let (text, sprite) = query.single_mut();
    if finish_playing.0 {
        // let mut text = query.single_mut();
        if let Some(mut text) = text {
            text.sections[0]
                .style
                .color
                .set_a(a);
        }
        if let Some(mut sprite) = sprite {
            sprite
                .color
                .set_a(a);
        }
    }
}

fn exit_system(mut exit: EventWriter<AppExit>, time: Res<Time>) {
    let seconds = time.elapsed_seconds();
    if seconds >= TOTAL_TIME {
        exit.send(AppExit);
    }
}

pub struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(BG_COLOR))
            .add_plugins(DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: TITLE.into(),
                    mode: bevy::window::WindowMode::BorderlessFullscreen,
                    transparent: true,
                    ..default()
                }),
                ..default()
            }))
            .add_systems(
                Startup,
                (
                    spawn_camera,
                    load_text,
                    spawn_sprites,
                    load_music,
                    load_finish_playing,
                ),
            )
            .add_systems(PostStartup, change_wallpaper)
            .add_systems(
                Update,
                (
                    bevy::window::close_on_esc,
                    update_alpha,
                    play_sound,
                    update_text_alpha,
                ),
            )
            .add_systems(Update, exit_system);
    }
}
