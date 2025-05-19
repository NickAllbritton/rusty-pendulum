
fn main() -> Result<(), String> {
    // TODO: Use different window proportions
    let wnd_width: u32 = 1000;
    let wnd_height: u32 = 1000;

    let sdl_context = sdl2::init()?;
    let vid_subsystem = sdl_context.video()?;
    let wnd = vid_subsystem.window("RustyPendulum", wnd_width, wnd_height)
        .build()
        .unwrap();

    let mut canvas = wnd.into_canvas()
        .build()
        .unwrap();

    let mut running: bool = true;
    let mut event_queue = sdl_context.event_pump()?;

    while running {
        // Handle events
        for event in event_queue.poll_iter() {
            match event {
                sdl2::event::Event::Quit {..} => {
                    running = false;
                }
                _ => {}
            }
        }

        // Draw screen
        canvas.set_draw_color(sdl2::pixels::Color::RGB(50, 50, 50));
        canvas.clear();
        canvas.present();
    }
    
    Ok(())
}
