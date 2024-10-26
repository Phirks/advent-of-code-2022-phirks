use cursive::event::{EventResult, Key};
use cursive::style::{Color, ColorStyle, Rgb};
use cursive::view::Resizable;
use cursive::views::Canvas;
use cursive::{Cursive, Printer};
use cursive_core::event::Event;
fn main() {
    // Start as usual
    let mut siv = cursive::default();
    siv.add_global_callback('q', |s| s.quit());

    // Canvas lets us easily override any method.
    // Canvas can have states, but we don't need any here, so we use `()`.
    let mut state = (1, 1);
    let my_view = Canvas::new(state)
        .with_draw(draw)
        .with_on_event(|state: &mut (u8, u8), event| match event {
            Event::Key(Key::Right) => {
                if state.0 < 5 {
                    state.0 += 1;
                }
                EventResult::Consumed(None)
            }
            Event::Key(Key::Left) => {
                if state.0 >= 1 {
                    state.0 -= 1;
                }
                EventResult::Consumed(None)
            }
            Event::Key(Key::Up) => {
                if state.1 >= 1 {
                    state.1 -= 1;
                }
                EventResult::Consumed(None)
            }
            Event::Key(Key::Down) => {
                if state.1 < 5 {
                    state.1 += 1;
                }
                EventResult::Consumed(None)
            }
            _ => EventResult::Consumed(None),
        })
        .fixed_size((6, 6));
    siv.add_layer(my_view);
    siv.set_autorefresh(true);
    siv.run();
}

fn draw(location: &(u8, u8), p: &Printer) {
    // We use the view size to calibrate the color
    let x_max = p.size.x as u8;
    let y_max = p.size.y as u8;
    let tail_location = 

    // Print each cell individually
    for x in 0..x_max {
        for y in 0..y_max {
            // We'll use a different style for each cell
            let style = ColorStyle::new(Color::Rgb(0, 0, 0), Color::Rgb(255, 255, 255));

            if *location != (x, y) {
                p.with_color(style, |printer| {
                    printer.print((x, y), ".");
                });
            } else {
                p.with_color(style, |printer| {
                    printer.print((x, y), "H");
                });
            }
        }
    }
}
