pub mod launcher;
pub mod ui;

use dioxus::desktop::tao::window::Icon;
use dioxus::desktop::tao::dpi::{PhysicalSize, Size};

fn main() {
    dioxus::desktop::launch_cfg(
        ui::app,
        |c| {
            c
            .with_window(|w| {
                w
                .with_title("Ruten v2")
                .with_transparent(true)
                .with_decorations(true)
                .with_inner_size(Size::Physical(PhysicalSize::new(920, 580)))
            })
            .with_icon(Icon::from_rgba(
                include_bytes!("./../resources/Ruten64.rgba").to_vec(),
                64,
                64
            ).unwrap())
        }
    );
}