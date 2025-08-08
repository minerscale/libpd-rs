#![allow(clippy::restriction)]

use std::sync::{mpsc, Arc, Mutex};

use libpd_rs::{
    functions::{block_size, verbose_print_state},
    Pd,
};

#[test]
fn send_and_receive_typed_message() {
    let sample_rate = 44100;
    let output_channels = 2;

    let mut pd = Pd::init_and_configure(0, output_channels, sample_rate).unwrap();
    let ctx = pd.audio_context();

    pd.dsp_on().unwrap();

    verbose_print_state(true);

    pd.open_patch("tests/patches/echo.pd").unwrap();

    let message_count = Arc::new(Mutex::new(0));
    let mc = message_count.clone();
    pd.on_message(move |source, selector, _list| {
        assert_eq!(source, "pd");
        assert_eq!(selector, "audiostatus");
        *mc.lock().unwrap() += 1;
    })
    .unwrap();
    let receiver_handle = pd.start_listening_from("pd").unwrap();

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

            let ticks = output_buffer.len() as i32 / (block_size() * output_channels);
            ctx.process_float(ticks, &input_buffer, &mut output_buffer);
            ctx.receive_messages_from_pd();
            match rx.try_recv() {
                Ok(_) => break,
                _ => continue,
            }
        }
    });

    pd.start_message(0).unwrap();
    pd.finish_message_as_typed_message_and_send_to("pd", "audiostatus")
        .unwrap();

    pd.send_message_to("pd", "audiostatus", &[]).unwrap();

    std::thread::sleep(std::time::Duration::from_millis(1000));

    assert_eq!(*message_count.lock().unwrap(), 2);
    // Stop pd.
    tx.send(()).unwrap();
    handle.join().unwrap();

    // Stop listening and close handle.
    pd.stop_listening_from(receiver_handle);
    pd.close_patch().unwrap();
}
