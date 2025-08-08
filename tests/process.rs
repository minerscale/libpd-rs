#![allow(clippy::restriction)]

use std::any::type_name;

use libpd_rs::{functions::block_size, Pd};

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

#[test]
fn all_process_functions() {
    let sample_rate = 44100;
    let output_channels = 2;

    let mut pd = Pd::init_and_configure(0, output_channels, sample_rate).unwrap();

    pd.dsp_on().unwrap();

    pd.open_patch("tests/patches/sine.pd").unwrap();

    let ctx = pd.audio_context();

    // Float
    let input_buffer = [0.0f32; 512];
    let mut output_buffer = [0.0f32; 1024];

    for _ in 0..(44100 / block_size() * output_channels) {
        let ticks = output_buffer.len() as i32 / (block_size() * output_channels);
        ctx.process_float(ticks, &input_buffer, &mut output_buffer);
    }

    let sum = output_buffer.iter().fold(0_f32, |mut acc, element| {
        acc += element;
        acc
    });

    assert_eq!(type_of(output_buffer[0]), "f32");
    assert_ne!(sum, 0.0);

    // Double
    let input_buffer = [0.0f64; 512];
    let mut output_buffer = [0.0f64; 1024];

    for _ in 0..(44100 / block_size() * output_channels) {
        let ticks = output_buffer.len() as i32 / (block_size() * output_channels);
        ctx.process_double(ticks, &input_buffer, &mut output_buffer);
    }

    let sum = output_buffer.iter().fold(0_f64, |mut acc, element| {
        acc += element;
        acc
    });

    assert_eq!(type_of(output_buffer[0]), "f64");
    assert_ne!(sum, 0.0);

    // Short
    let input_buffer = [0i16; 512];
    let mut output_buffer = [0i16; 1024];

    for _ in 0..(44100 / block_size() * output_channels) {
        let ticks = output_buffer.len() as i32 / (block_size() * output_channels);
        ctx.process_short(ticks, &input_buffer, &mut output_buffer);
    }

    let sum = output_buffer.iter().fold(0_i32, |mut acc, element| {
        acc += *element as i32;
        acc
    });

    assert_eq!(type_of(output_buffer[0]), "i16");
    assert_ne!(sum, 0);

    // Float Raw
    let input_buffer = [0.0f32; 512];
    let mut output_buffer = [0.0f32; 1024];

    for _ in 0..(44100 / block_size() * output_channels) {
        ctx.process_raw(&input_buffer, &mut output_buffer);
    }

    let sum = output_buffer.iter().fold(0_f32, |mut acc, element| {
        acc += element;
        acc
    });

    assert_eq!(type_of(output_buffer[0]), "f32");
    assert_ne!(sum, 0.0);

    // Double Raw
    let input_buffer = [0.0f64; 512];
    let mut output_buffer = [0.0f64; 1024];

    for _ in 0..(44100 / block_size() * output_channels) {
        ctx.process_raw_double(&input_buffer, &mut output_buffer);
    }

    let sum = output_buffer.iter().fold(0_f64, |mut acc, element| {
        acc += element;
        acc
    });

    assert_eq!(type_of(output_buffer[0]), "f64");
    assert_ne!(sum, 0.0);

    // Short Raw
    let input_buffer = [0_i16; 512];
    let mut output_buffer = [0_i16; 1024];

    for _ in 0..(44100 / block_size() * output_channels) {
        ctx.process_raw_short(&input_buffer, &mut output_buffer);
    }

    let sum = output_buffer.iter().fold(0_i32, |mut acc, element| {
        acc += *element as i32;
        acc
    });

    assert_eq!(type_of(output_buffer[0]), "i16");
    assert_ne!(sum, 0);

    pd.close_patch().unwrap();
}
