use bevy::prelude::*;

#[cfg(feature = "debug")]
use bevy_inspector_egui::WorldInspectorPlugin;

fn main() {
    // Uygulama nesnesi oluşturuldu
    let mut app = App::new();

    // Belli boyutları ve başlığı olan bir pencere eklendi
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        window: WindowDescriptor {
            title: "Mayın Tarayıcı (Öğreti)".to_string(),
            width: 600.,
            height: 700.,
            ..default()
        },
        ..default()
    }));

    // Inspector plugin'i ekleniyor
    #[cfg(feature = "debug")] // fonksiyon içindeki bu makro kullanımı ilk kez görüyorum.
    app.add_plugin(WorldInspectorPlugin::new());
    // başlangıçta sistemi ayarlarken iki boyutlu bir ortografik kamera da ekleniyor.
    // Klasik bir sistem her frame için çalışırken startup sistemleri sadece başlangıçta
    // bir kereliğine çalışır.
    app.add_startup_system(setup_camera);
    // uygulama çalıştırılıyor
    app.run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
