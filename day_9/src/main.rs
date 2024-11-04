use std::borrow::Borrow;
use std::cell::RefCell;
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread;
use cursive::event::{Event, EventResult, Key};
use cursive::style::{Color, ColorStyle, Rgb};
use cursive::view::Resizable;
use cursive::views::Canvas;
use cursive::{Cursive, Printer};

#[derive(Clone)]
struct GridNode {
    position: (usize, usize),
    display_property: Option<RefCell<String>>,
}

impl GridNode {
    fn new(x: usize, y: usize) -> GridNode {
        GridNode {
            position: (x, y),
            display_property: None,
        }
    }
}

fn main() {
    let (tx, rx) = mpsc::channel();
    
    // Start as usual
    let mut siv = cursive::default();
    siv.add_global_callback('q', |s| s.quit());

    let rxm = Arc::new(Mutex::new(rx));

    let state: Vec<Vec<(&str, (u8, u8, u8), (u8, u8, u8))>> = (0..6)
        .map(|x| (0..6).map(|y| (".", (255, 255, 255), (0, 0, 0))).collect())
        .collect();

    // Start a thread to update the state
    let tx_clone = tx.clone();
    thread::spawn(move || {
        let mut current_state = state.clone();
        loop {
            current_state[0][0].0 = "H";
            if let Err(_) = tx_clone.send(current_state.clone()) {
                break; // Exit on send failure
            }
            thread::sleep(std::time::Duration::from_secs(1)); // Throttle updates
        }
    });

    let my_view = Canvas::new(rxm.clone())
        .with_draw(draw)
        .with_on_event(|_, event| match event {
            Event::Char(c) => EventResult::Consumed(None),
            _ => EventResult::Ignored,
        })
        .fixed_size((6, 6));
    
    siv.add_layer(my_view);
    siv.set_autorefresh(true);
    siv.run();
}

fn draw(rxm: &Arc<Mutex<Receiver<Vec<Vec<(&str, (u8, u8, u8), (u8, u8, u8))>>>>>, p: &Printer) {
    p.print((0, 0), "Rendering...");

    if let Ok(mutex_rx) = rxm.lock() {
        if let Ok(grid) = mutex_rx.recv() {
            let x_max = p.size.x as usize;
            let y_max = p.size.y as usize;
            for x in 0..x_max {
                for y in 0..y_max {
                    let (char, (br, bg, bb), (cr, cg, cb)) = grid[x][y];
                    let style = ColorStyle::new(Color::Rgb(cr, cg, cb), Color::Rgb(br, bg, bb));
                    p.with_color(style, |printer| {
                        printer.print((x as usize, y as usize), char);
                    });
                }
            }
        } else {
            p.print((0, 0), "Error receiving data");
        }
    } else {
        p.print((0, 0), "Error locking receiver");
    }
}
