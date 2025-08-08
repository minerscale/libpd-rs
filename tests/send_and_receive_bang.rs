#![allow(clippy::restriction)]

use std::sync::{mpsc, Arc, Mutex};

use libpd_rs::{functions::block_size, Pd};

#[test]
fn send_and_receive_bang() {
    let sample_rate = 44100;
    let output_channels = 2;

    let bangs: Arc<Mutex<Vec<&str>>> = Arc::new(Mutex::new(vec![]));

    let mut pd = Pd::init_and_configure(0, output_channels, sample_rate).unwrap();
    let ctx = pd.audio_context();

    pd.open_patch("tests/patches/echo.pd").unwrap();

    let bangs_to_fill = bangs.clone();
    pd.on_bang(move |source| {
        assert_eq!(source, "bang_from_pd");
        bangs_to_fill.lock().unwrap().push("bang");
    })
    .unwrap();

    let receiver_handle = pd.start_listening_from("bang_from_pd").unwrap();
    pd.dsp_on().unwrap();

    let (tx, rx) = mpsc::channel::<()>();

    let handle = std::thread::spawn(move || {
        // Mimic audio callback buffers.
        let input_buffer = [0.0f32; 512];
        let mut output_buffer = [0.0f32; 1024];

        // Run pd
        loop {
            // Mimic an audio callback.
            let approximate_buffer_duration =
                (output_buffer.len() as f32 / sample_rate as f32) * 1000.0;
            std::thread::sleep(std::time::Duration::from_millis(
                approximate_buffer_duration as u64,
            ));

            ctx.receive_messages_from_pd();
            let ticks = output_buffer.len() as i32 / (block_size() * output_channels);
            ctx.process_float(ticks, &input_buffer, &mut output_buffer);
            match rx.try_recv() {
                Ok(_) => break,
                _ => continue,
            }
        }
    });

    // Send 5 bangs.
    for _ in 0..5 {
        pd.send_bang_to("bang_from_rust").unwrap();
    }

    std::thread::sleep(std::time::Duration::from_millis(50));

    // Stop pd.
    tx.send(()).unwrap();
    handle.join().unwrap();

    assert_eq!(bangs.lock().unwrap().len(), 5);

    // Stop listening and close handle.
    pd.stop_listening_from(receiver_handle);
    pd.close_patch().unwrap();
}
