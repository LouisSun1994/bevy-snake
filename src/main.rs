use bevy::prelude::*;

// Component（元件）：純資料，貼在 entity 上
#[derive(Component)]
struct Speed(f32);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup) // 開場跑一次
        .add_systems(Update, (move_square, report)) // 每一幀都跑
        .run(); // 主迴圈從這裡開始，關視窗才會 return
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d); // 沒有攝影機就什麼都畫不出來
    commands.spawn((
        Sprite::from_color(Color::srgb(0.9, 0.4, 0.1), Vec2::splat(80.0)),
        Transform::default(),
        Speed(2.0),
    ));
}

// System（系統）：一個普通函式，參數宣告「我要哪些資料」
fn move_square(time: Res<Time>, mut query: Query<(&mut Transform, &Speed)>) {
    for (mut transform, speed) in &mut query {
        transform.translation.x = (time.elapsed_secs() * speed.0).sin() * 300.0;
    }
}

// 每 60 幀印一次：觀察這個 system 被哪條 thread 執行
fn report(mut frame: Local<u32>, time: Res<Time>) {
    *frame += 1;
    if *frame % 60 == 0 {
        println!(
            "frame {:>5} | dt {:>6.2} ms | {:?}",
            *frame,
            time.delta_secs() * 1000.0,
            std::thread::current().id()
        );
    }
}
