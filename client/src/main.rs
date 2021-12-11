use common::bevy::prelude::*;
use common::bevy_networking_turbulence::{NetworkResource, NetworkingPlugin};
use common::events::*;

pub fn main() {
    let mut app = App::build();

    app.add_plugins(DefaultPlugins)
        .add_plugin(NetworkingPlugin::default());



    // when building for Web, use WebGL2 rendering
    #[cfg(target_arch = "wasm32")]
        app.add_plugin(common::bevy_webgl2::WebGL2Plugin);

    app.add_system(capture_clicks.system());

    app.run();
}

fn capture_clicks(mut net: ResMut<NetworkResource>, mouse_input: Res<Input<MouseButton>>, windows: Res<Windows>) {
    let win = windows.get_primary().expect("no primary window");
    if mouse_input.just_pressed(MouseButton::Left) {
        let position = win.cursor_position().expect("Mouse was clicked, cursor should have position");
        info!("Click detected at {},{}", position.x, position.y);
        net.broadcast_message(GameEvent::MovePoint(position));
    }
}
