pub use bevy;
pub use bevy_networking_turbulence;
pub use serde_json as serde_form;
pub mod events;
pub mod game;
pub mod protocol;
pub mod pointer;
pub mod errors;
pub mod graphics;


/*fn main() {
    //bevy::info!("hello world")
    println!("Hello from common main")
}

 */

pub const SERVER_PORT: u16 = 15678;
pub const SERVER_WEBRTC_PORT: u16 = 15679;


#[cfg(target_arch = "wasm32")]
pub fn get_random() -> u32 {
    let mut dest: [u8; 4] = [0; 4];
    getrandom::getrandom(&mut dest).expect("Failed to generate random number");
    (dest[0] as u32) * 24 +
        (dest[1] as u32) * 16 +
        (dest[2] as u32) * 8 +
        (dest[3] as u32)
}

#[cfg(not(target_arch = "wasm32"))]
pub fn get_random() -> u32 {
    rand::random::<u32>()
}

