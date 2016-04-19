extern crate piston_window;
extern crate camera_controllers;

use piston_window::*;

fn main() {
    let mut window : PistonWindow = WindowSettings::new("Hello Piston!", [640, 480])
        .exit_on_esc(true).build().unwrap();
    let mut events : WindowEvents = window.events();

    while let Some(e) = events.next(&mut window) {
        window.draw_2d(&e, |_c, g| {
            clear([0.3, 0.85, 0.2, 1.0], g); // A lovely green
        });
    }
}
