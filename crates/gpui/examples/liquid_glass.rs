#![cfg_attr(target_family = "wasm", no_main)]

//! Panes of liquid glass over a busy backdrop. Move the pointer over the
//! panes to light them; the stripes show the refraction at each bevel.
//! `LIQUID_GLASS_POINTER=x,y` pins that light instead, for screenshots.

#[path = "example_support/fonts.rs"]
mod example_support;

use gpui::{
    App, Bounds, Context, Corners, Hsla, LiquidGlassStyle, Pixels, Radians, Window, WindowBounds,
    WindowOptions, canvas, div, hsla, point, prelude::*, px, size,
};
use gpui_platform::application;

struct LiquidGlassExample;

fn glass(radius: Pixels, style: LiquidGlassStyle) -> impl IntoElement {
    canvas(
        |_, _, _| {},
        move |bounds, _, window: &mut Window, _| {
            let style = LiquidGlassStyle {
                pointer: Some(fixed_pointer().unwrap_or_else(|| window.mouse_position())),
                ..style
            };
            window.paint_liquid_glass(bounds, Corners::all(radius), style);
        },
    )
    .absolute()
    .top_0()
    .left_0()
    .size_full()
}

/// `LIQUID_GLASS_POINTER=x,y` pins the light to a point, for screenshots.
fn fixed_pointer() -> Option<gpui::Point<Pixels>> {
    let value = std::env::var("LIQUID_GLASS_POINTER").ok()?;
    let (x, y) = value.split_once(',')?;
    Some(point(
        px(x.trim().parse().ok()?),
        px(y.trim().parse().ok()?),
    ))
}

fn pane(
    width: Pixels,
    height: Pixels,
    radius: Pixels,
    style: LiquidGlassStyle,
    label: &'static str,
) -> impl IntoElement {
    div()
        .relative()
        .w(width)
        .h(height)
        .flex()
        .items_center()
        .justify_center()
        .child(glass(radius, style))
        .child(
            div()
                .text_size(px(18.))
                .text_color(hsla(0., 0., 0.1, 0.9))
                .child(label),
        )
}

fn stripe_color(index: usize) -> Hsla {
    hsla((index as f32 * 0.083) % 1.0, 0.75, 0.55, 1.0)
}

impl Render for LiquidGlassExample {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let stripes = (0..36).map(|index| {
            div()
                .w(px(28.))
                .h_full()
                .bg(stripe_color(index))
                .when(index % 2 == 1, |stripe| stripe.bg(hsla(0., 0., 0.97, 1.)))
        });
        let lines = (0..14).map(|index| {
            div()
                .text_size(px(22.))
                .text_color(hsla(0., 0., 0.08, 1.))
                .child(format!(
                    "{index:02}  The quick brown fox jumps over the lazy dog — 液态玻璃"
                ))
        });
        let clear = LiquidGlassStyle {
            blur: px(0.),
            ..LiquidGlassStyle::default()
        };
        let frosted = LiquidGlassStyle {
            blur: px(10.),
            tint: hsla(0., 0., 1., 0.25),
            ..LiquidGlassStyle::default()
        };
        let lit_from_right = LiquidGlassStyle {
            light_angle: Radians(-0.6),
            dispersion: 0.3,
            thickness: px(28.),
            ..LiquidGlassStyle::default()
        };
        let supported = window.supports_liquid_glass();

        div()
            .id("root")
            .size_full()
            .relative()
            .on_mouse_move(cx.listener(|_, _, _, cx| cx.notify()))
            .child(div().absolute().size_full().flex().children(stripes))
            .child(
                div()
                    .absolute()
                    .size_full()
                    .p_6()
                    .flex()
                    .flex_col()
                    .gap_3()
                    .children(lines),
            )
            .child(
                div()
                    .absolute()
                    .size_full()
                    .flex()
                    .flex_col()
                    .items_center()
                    .justify_center()
                    .gap_8()
                    .child(
                        div()
                            .flex()
                            .gap_6()
                            .child(pane(px(220.), px(64.), px(32.), clear, "Clear pill"))
                            .child(pane(px(64.), px(64.), px(32.), clear, "+")),
                    )
                    .child(
                        div()
                            .flex()
                            .gap_8()
                            .child(pane(px(300.), px(180.), px(28.), frosted, "Frosted card"))
                            .child(pane(
                                px(180.),
                                px(180.),
                                px(90.),
                                lit_from_right,
                                "Thick lens",
                            )),
                    )
                    .child(
                        div()
                            .text_size(px(14.))
                            .text_color(hsla(0., 0., 0.1, 1.))
                            .bg(hsla(0., 0., 1., 0.8))
                            .px_2()
                            .child(if supported {
                                "renderer draws liquid glass"
                            } else {
                                "fallback: blur + tint + rim"
                            }),
                    ),
            )
    }
}

fn run_example() {
    application().run(|cx: &mut App| {
        if !example_support::load_fonts(cx) {
            return;
        }
        let bounds = Bounds::new(point(px(120.), px(120.)), size(px(1000.), px(680.)));
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |_, cx| cx.new(|_| LiquidGlassExample),
        )
        .unwrap();
        cx.activate(true);
    });
}

#[cfg(not(target_family = "wasm"))]
fn main() {
    run_example();
}

#[cfg(target_family = "wasm")]
#[wasm_bindgen::prelude::wasm_bindgen(start)]
pub fn start() {
    gpui_platform::web_init();
    run_example();
}
