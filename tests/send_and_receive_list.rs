#![allow(clippy::restriction)]

use std::sync::{mpsc, Arc, Mutex};

use libpd_rs::{functions::block_size, Pd};

use libpd_rs::Atom;

#[test]
fn send_and_receive_list() {
    let sample_rate = 44100;
    let output_channels = 2;

    let list_received: Arc<Mutex<Vec<Atom>>> = Arc::new(Mutex::new(vec![]));

    let mut pd = Pd::init_and_configure(0, output_channels, sample_rate).unwrap();
    let ctx = pd.audio_context();

    pd.open_patch("tests/patches/echo.pd").unwrap();

    let list_to_fill = list_received.clone();
    pd.on_list(move |source, list| {
        assert_eq!(source, "list_from_pd");
        for atom in list {
            list_to_fill.lock().unwrap().push(atom.clone());
        }
    })
    .unwrap();
    let receiver_handle = pd.start_listening_from("list_from_pd").unwrap();

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

            let ticks = output_buffer.len() as i32 / (block_size() * output_channels);
            ctx.process_float(ticks, &input_buffer, &mut output_buffer);
            ctx.receive_messages_from_pd();
            match rx.try_recv() {
                Ok(_) => break,
                _ => continue,
            }
        }
    });

    let list_to_send: Vec<Atom> = vec![
        "daisy".into(),
        33.5_f64.into(),
        42_f64.into(),
        "bang".into(),
        12.0_f64.into(),
        0.0_f64.into(),
    ];
    let list_to_compare: Vec<Atom> = vec![
        "daisy".into(),
        33.5_f64.into(),
        42_f64.into(),
        "bang".into(),
        12.0_f64.into(),
        0.0_f64.into(),
        "daisy".into(),
        33.5_f64.into(),
        42_f64.into(),
        "bang".into(),
        12.0_f64.into(),
        0.0_f64.into(),
    ];

    pd.send_list_to("list_from_rust", &list_to_send).unwrap();

    pd.start_message(list_to_send.len() as i32).unwrap();
    pd.add_symbol_to_started_message("daisy").unwrap();
    pd.add_double_to_started_message(33.5_f64).unwrap();
    pd.add_double_to_started_message(42_f64).unwrap();
    pd.add_symbol_to_started_message("bang").unwrap();
    pd.add_double_to_started_message(12.0_f64).unwrap();
    pd.add_double_to_started_message(0.0_f64).unwrap();
    pd.finish_message_as_list_and_send_to("list_from_rust")
        .unwrap();

    std::thread::sleep(std::time::Duration::from_millis(50));

    // Stop pd.
    tx.send(()).unwrap();
    handle.join().unwrap();

    assert_eq!(list_received.lock().unwrap().len(), 12);

    list_received
        .lock()
        .unwrap()
        .iter()
        .zip(list_to_compare.iter())
        .for_each(|(a, b)| {
            assert_eq!(a, b);
        });

    // Stop listening and close handle.
    pd.stop_listening_from(receiver_handle);
    pd.close_patch().unwrap();
}
