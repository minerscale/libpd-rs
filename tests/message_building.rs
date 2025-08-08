#![allow(clippy::restriction)]

use libpd_rs::{error::SendError, Pd};

#[test]
fn message_building() {
    let sample_rate = 44100;
    let output_channels = 2;

    let mut pd = Pd::init_and_configure(0, output_channels, sample_rate).unwrap();
    pd.activate_audio(true).unwrap();

    pd.open_patch("tests/patches/echo.pd").unwrap();

    assert!(matches!(
        pd.finish_message_as_list_and_send_to("not_started"),
        Err(libpd_rs::error::SendError::MessageNotStarted)
    ));

    pd.start_message(1).unwrap();
    assert!(pd.add_float_to_started_message(0.0).is_ok());
    assert!(matches!(
        pd.add_float_to_started_message(1.0),
        Err(libpd_rs::error::SendError::OutOfRange)
    ));

    assert!(pd.finish_message_as_list_and_send_to("no_land").is_err());

    pd.start_message(5).unwrap();

    for _ in 0..5 {
        pd.add_float_to_started_message(0.0).unwrap();
    }

    assert!(matches!(
        pd.finish_message_as_typed_message_and_send_to("jeff", "yuh_oh"),
        Err(SendError::OutOfRange)
    ));

    // The implementation in libpd looks like, where maxlen is the length of the message:
    // t_atom *v = realloc(s_argv, maxlen * sizeof(t_atom));
    // if (v)
    // {
    //   s_argv = v;
    //   s_argm = maxlen;
    // }
    // else
    // {
    //   return -1;
    // }
    //
    // So it is platform dependent for example, it depends on how much memory a process can allocate I guess if this function errors or not.
    // It would be wise to handle the result.
    if pd.start_message(i32::MAX).is_ok() {
        pd.add_double_to_started_message(0.23).unwrap();

        let result = pd.finish_message_as_list_and_send_to("no_land");
        assert!(result.is_err());
        let result = pd.finish_message_as_typed_message_and_send_to("no_land", "no_where");
        assert!(result.is_err());
    } else {
        pd.start_message(1).unwrap();
        pd.add_double_to_started_message(0.23).unwrap();

        let result = pd.finish_message_as_list_and_send_to("no_land");
        assert!(result.is_err());
        let result = pd.finish_message_as_typed_message_and_send_to("no_land", "no_where");
        assert!(result.is_err());
    }

    pd.close_patch().unwrap();
}
