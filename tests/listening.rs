#![allow(clippy::restriction)]
#![allow(clippy::unnecessary_cast)]

use libpd_rs::types::ReceiverHandle;
use libpd_rs::Pd;

#[test]
fn listening() {
    let sample_rate = 44100;
    let output_channels = 2;

    let mut pd = Pd::init_and_configure(0, output_channels, sample_rate).unwrap();
    pd.dsp_on().unwrap();

    pd.open_patch("tests/patches/echo.pd").unwrap();

    let result = pd.start_listening_from("list_from_pd");
    assert!(result.is_ok());
    // Just crates the endpoint.
    let result = pd.start_listening_from("non_existent");
    assert!(result.is_ok());

    let handle_1 = pd.start_listening_from("list_from_pd").unwrap();
    let handle_2 = pd.start_listening_from("list_from_pd").unwrap();

    pd.stop_listening_from(handle_1);
    pd.stop_listening_from(handle_2);
    // It will just ignore the null pointer.
    let handle: ReceiverHandle = (std::ptr::null_mut() as *mut std::ffi::c_void).into();
    pd.stop_listening_from(handle);

    assert!(pd.source_to_listen_from_exists("list_from_pd").unwrap());
    assert!(!pd
        .source_to_listen_from_exists("endpoint_not_created")
        .unwrap());

    pd.close_patch().unwrap();
}
