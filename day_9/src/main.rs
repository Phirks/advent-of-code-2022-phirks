use std::cell::RefCell;

use cursive::event::{EventResult, Key};
use cursive::style::{Color, ColorStyle, Rgb};
use cursive::view::Resizable;
use cursive::views::Canvas;
use cursive::{Cursive,Printer};
use cursive_core::event::Event;

#[derive(Clone)]
struct GridNode {
    position: (usize,usize),
    display_property: Option<RefCell<String>>,
}

impl GridNode{
    fn new(x: usize, y: usize) -> GridNode {
        GridNode{
            position: (x,y),
            display_property: None,
        }
    }
}

#[derive(Clone, Copy)]
struct Grid {
    grid_objects: Vec<Vec<Option<GridNode>>>,
    x_size: usize,
    y_size: usize,
}

impl Grid {
    fn new(x_size: usize, y_size: usize) -> Grid {
        let mut grid_objects: Vec<Vec<Option<GridNode>>> = vec![vec![]];
        for y in 0..y_size{
            for x in 0..x_size{
                grid_objects[y].push(None);
            }
            grid_objects.push(vec![]);
        }
        Grid {
            grid_objects,
            x_size,
            y_size, 
        }
    }
    fn add(&mut self, x: usize, y: usize, display_property: String){
        let mut new_node = GridNode::new(x, y);
        new_node.display_property = Some(RefCell::new(display_property));
        new_node.position = (x,y);
        self.grid_objects[x][y] = Some(new_node);
    } 
}

fn main() {
    let mut grid: Grid = Grid::new(6, 6);
    for y in 0..6{
        for x in 0..6{
            grid.add(x, y, ".".to_string());
        }
    }
    // Start as usual
    let mut siv = cursive::default();
    siv.add_global_callback('q', |s| s.quit());

    // Canvas lets us easily override any method.
    // Canvas can have states, but we don't need any here, so we use `()`.
    let my_view = Canvas::new(grid)
        .with_draw(draw)
        .with_on_event(|grid: &mut Grid, event| match event {
            Event::Key(Key::Right) => {
                EventResult::Consumed(None)
            }
            Event::Key(Key::Left) => {
                EventResult::Consumed(None)
            }
            Event::Key(Key::Up) => {
                EventResult::Consumed(None)
            }
            Event::Key(Key::Down) => {
                EventResult::Consumed(None)
            }
            _ => EventResult::Consumed(None),
        })
        .fixed_size((6, 6));
    siv.add_layer(my_view);
    siv.set_autorefresh(true);
    siv.run();
}

fn draw(grid: &Grid, p: &Printer) {
    // We use the view size to calibrate the color
    let x_max = p.size.x as u8;
    let y_max = p.size.y as u8;

    // Print each cell individually
    for x in 0..x_max {
        for y in 0..y_max {
            // We'll use a different style for each cell
            let style = ColorStyle::new(Color::Rgb(0, 0, 0), Color::Rgb(255, 255, 255));
            p.with_color(style, |printer| {
                printer.print((x, y), ".");
            });
        }
    };
}
