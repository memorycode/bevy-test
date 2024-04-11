use bevy::{
    diagnostic::FrameTimeDiagnosticsPlugin,
    prelude::*,
    render::camera::{self, ScalingMode},
    window::Cursor,
};
use bevy_asset_loader::{
    asset_collection::AssetCollection,
    loading_state::{config::ConfigureLoadingState, LoadingState, LoadingStateAppExt},
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

mod player;
const DEFAULT_MOVEMENT_SPEED: f32 = 128.0;

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
enum GameState {
    #[default]
    LoadingScreen,
    Playing,
    Upgrading,
    GameOver,
}

#[derive(Resource, Default)]
struct Game {}

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Bullet Hell".into(),
                        cursor: Cursor {
                            visible: true,
                            ..default()
                        },
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_plugins(WorldInspectorPlugin::new())
        .insert_resource(ClearColor(Color::rgb(0.53, 0.53, 0.53)))
        .init_resource::<Game>()
        .init_state::<GameState>()
        .add_loading_state(
            LoadingState::new(GameState::LoadingScreen)
                .continue_to_state(GameState::Playing)
                .load_collection::<Assets>(),
        )
        .add_systems(Startup, setup_camera)
        .add_systems(OnEnter(GameState::Playing), setup)
        .add_systems(
            FixedUpdate,
            (move_character, follow_character, rotate_character)
                .chain()
                .run_if(in_state(GameState::Playing)),
        )
        .add_systems(Update, draw_cursor)
        .run();
}

#[derive(AssetCollection, Resource)]
struct Assets {
    // if the sheet would have padding, you could set that with `padding_x` and `padding_y`.
    // if there would be space between the top left corner of the sheet and the first sprite, you could configure that with `offset_x` and `offset_y`
    // A texture atlas layout does not have a path as no asset file will be loaded for the layout
    #[asset(texture_atlas_layout(tile_size_x = 12., tile_size_y = 12., columns = 12, rows = 12))]
    layout: Handle<TextureAtlasLayout>,
    // you can configure the sampler for the sprite sheet image
    #[asset(path = "roguelikeChar_transparent.png")]
    sheet: Handle<Image>,
}

#[derive(Component)]
struct Position {
    x: f32,
    y: f32,
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Health(f32);

fn setup_camera(mut commands: Commands) {
    let mut camera_bundle = Camera2dBundle::default();
    //camera_bundle.projection.scaling_mode = ScalingMode::FixedVertical(10.);
    commands.spawn(camera_bundle);
}

fn move_character(
    keys: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    let mut transform = query.single_mut();
    let mut direction = Vec2::ZERO;
    if keys.any_pressed([KeyCode::ArrowLeft, KeyCode::KeyA]) {
        direction.x -= 1.;
    }

    if keys.any_pressed([KeyCode::ArrowRight, KeyCode::KeyD]) {
        direction.x += 1.;
    }

    if keys.any_pressed([KeyCode::ArrowUp, KeyCode::KeyW]) {
        direction.y += 1.;
    }

    if keys.any_pressed([KeyCode::ArrowDown, KeyCode::KeyS]) {
        direction.y -= 1.;
    }

    direction = direction.normalize_or_zero();
    let move_delta = direction * DEFAULT_MOVEMENT_SPEED * time.delta_seconds();

    transform.translation.x = transform.translation.x + move_delta.x;
    transform.translation.y = transform.translation.y + move_delta.y;
}

fn draw_cursor(
    camera_query: Query<(&Camera, &GlobalTransform)>,
    windows: Query<&Window>,
    mut gizmos: Gizmos,
) {
    let (camera, camera_transform) = camera_query.single();
    let Some(cursor_position) = windows.single().cursor_position() else {
        return;
    };

    // Calculate a world position based on the cursor's position.
    let Some(point) = camera.viewport_to_world_2d(camera_transform, cursor_position) else {
        return;
    };

    gizmos.circle_2d(point, 5., Color::WHITE);
}

fn rotate_character(
    mut query: Query<&mut Transform, With<Player>>,
    windows: Query<&Window>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
) {
    /*let mut paddle_transform = query.single_mut();
    let z = paddle_transform.translation.z;
    let (camera, camera_transform) = camera_query.single();
    let Some(cursor_position) = windows.single().cursor_position() else {
        return;
    };

    // Calculate a world position based on the cursor's position.
    let Some(point) = camera.viewport_to_world_2d(camera_transform, cursor_position) else {
        return;
    };

    println!("{:?}", paddle_transform);
    paddle_transform.look_at(point.extend(z), Vec3::Y);*/
}

fn follow_character(
    mut camera_query: Query<&mut Transform, (With<Camera>, Without<Player>)>,
    player_query: Query<&Transform, With<Player>>,
) {
    if player_query.is_empty() {
        //return;
    }

    let mut camera_transform = camera_query.single_mut();
    let player_translation = player_query.single().translation;
    camera_transform.translation = camera_transform.translation.lerp(player_translation, 0.5);
}

fn setup(mut commands: Commands, assets: Res<Assets>) {
    commands.spawn((
        SpriteSheetBundle {
            texture: assets.sheet.clone(),
            atlas: TextureAtlas::from(assets.layout.clone()),
            transform: Transform::from_scale(Vec3::splat(6.0)),
            ..default()
        },
        Player,
        Health(0.),
    ));
}
