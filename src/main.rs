#![windows_subsystem = "windows"]
use rand::Rng;
use minifb::{Window, WindowOptions, Scale};


const WIDTH: usize = 700;
const HEIGHT: usize = 450;

fn main() {
    let mut grid = vec![0; WIDTH * HEIGHT]; 
    let mut rng = rand::thread_rng();
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            if rng.gen_range(0..101) < 33{
                grid[y * WIDTH + x] = 1;
            }
        }
    }

    let mut window = Window::new(
        "Game of Life",
        WIDTH,
        HEIGHT,
        WindowOptions {
            scale: Scale::X2,
            ..WindowOptions::default()
        },
    )
    .unwrap();

    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    while window.is_open() {
            for y in 0..HEIGHT {
                for x in 0..WIDTH {
                    let color = if grid[y * WIDTH + x] == 1 { 0xFFFFFF } else { 0x000000 };
                    buffer[y * WIDTH + x] = color;
                }
            }
            window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
        
        let mut new_grid = vec![0; WIDTH * HEIGHT]; 
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let mut count = 0;

                for cell_y in -1..=1 {
                    for cell_x in -1..=1 {
                        if cell_y == 0 && cell_x == 0 {
                            continue;
                        }

                        let ny = (y as isize + cell_y).rem_euclid(HEIGHT as isize) as usize;
                        let nx = (x as isize + cell_x).rem_euclid(WIDTH as isize) as usize;

                        if grid[ny * WIDTH + nx] == 1 {
                            count += 1;
                        }
                    }
                }

                if grid[y * WIDTH + x] == 1 {
                    if count == 2 || count == 3 {
                        new_grid[y * WIDTH + x] = 1;
                    } else {
                        new_grid[y * WIDTH + x] = 0;
                    }
                } else {
                    if count == 3 {
                        new_grid[y * WIDTH + x] = 1;
                    } else {
                        new_grid[y * WIDTH + x] = 0;
                    }
                }
            }
        }
        grid = new_grid;
    }
}