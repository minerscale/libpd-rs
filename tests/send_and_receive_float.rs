#![allow(clippy::restriction)]

use std::sync::{mpsc, Arc, Mutex};

use libpd_rs::{functions::block_size, Pd};

#[test]
fn send_and_receive_float() {
    let sample_rate = 44100;
    let output_channels = 2;

    let floats: Arc<Mutex<Vec<f32>>> = Arc::new(Mutex::new(vec![]));

    let mut pd = Pd::init_and_configure(0, output_channels, sample_rate).unwrap();
    let ctx = pd.audio_context();

    pd.open_patch("tests/patches/echo.pd").unwrap();

    let floats_to_fill = floats.clone();
    pd.on_float(move |source, value| {
        assert_eq!(source, "float_from_pd");
        floats_to_fill.lock().unwrap().push(value);
    })
    .unwrap();
    let receiver_handle = pd.start_listening_from("float_from_pd").unwrap();

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

    let mut float: f32 = 2.0;
    // Send 5 floats in sequence.
    for _ in 0..5 {
        pd.send_float_to("float_from_rust", float).unwrap();
        float *= 2.0;
    }
    pd.send_float_to("float_from_rust", f32::MAX).unwrap();
    pd.send_float_to("float_from_rust", f32::MIN).unwrap();
    std::thread::sleep(std::time::Duration::from_millis(50));

    // Stop pd.
    tx.send(()).unwrap();
    handle.join().unwrap();

    let floats_to_compare: Vec<f32> = vec![2.0, 4.0, 8.0, 16.0, 32.0];

    assert_eq!(floats.lock().unwrap().len(), 7);

    floats
        .lock()
        .unwrap()
        .iter()
        .zip(floats_to_compare[..5].iter())
        .for_each(|(a, b)| {
            assert_eq!((a - b) as i32, 0);
        });
    assert_eq!(f32::MAX, floats.lock().unwrap()[5]);
    assert_eq!(f32::MIN, floats.lock().unwrap()[6]);

    // Stop listening and close handle.
    pd.stop_listening_from(receiver_handle);
    pd.close_patch().unwrap();
}
