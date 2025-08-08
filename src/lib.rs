#![warn(
    clippy::all,
    clippy::pedantic,
    clippy::restriction,
    clippy::nursery,
    clippy::cargo
)]
#![allow(
    // Group of too restrictive lints
    clippy::allow_attributes_without_reason,
    clippy::undocumented_unsafe_blocks,
    clippy::as_conversions,
    clippy::arithmetic_side_effects,
    clippy::float_arithmetic,
    clippy::blanket_clippy_restriction_lints,
    clippy::implicit_return,
    clippy::enum_glob_use,
    clippy::wildcard_enum_match_arm,
    clippy::pattern_type_mismatch,
    clippy::shadow_reuse,
    clippy::shadow_same,
    clippy::shadow_unrelated,
    clippy::must_use_candidate,
    clippy::clone_on_ref_ptr,
    clippy::multiple_crate_versions,
    clippy::default_numeric_fallback,
    clippy::map_err_ignore,
    clippy::std_instead_of_alloc,
    clippy::question_mark_used,
    clippy::std_instead_of_core,
    clippy::partial_pub_fields,
    clippy::ref_patterns,
    clippy::semicolon_inside_block,
    clippy::semicolon_outside_block,
    clippy::pub_with_shorthand,
    clippy::self_named_module_files,
    clippy::integer_division_remainder_used,
    clippy::min_ident_chars,
    clippy::missing_trait_methods,
    clippy::mem_forget,
    clippy::pub_use,
    clippy::single_call_fn,
    clippy::arbitrary_source_item_ordering,
    clippy::single_char_lifetime_names,
    clippy::unwrap_in_result,
    clippy::expect_used,
    clippy::missing_panics_doc,

    // Expect is fine in relevant cases
    // clippy::expect_used,

    // Too restrictive for the current style
    clippy::missing_inline_in_public_items,
    clippy::exhaustive_structs,
    clippy::exhaustive_enums,
    clippy::module_name_repetitions,
    clippy::unseparated_literal_suffix,

    // Docs
    clippy::missing_docs_in_private_items,

    // Comment these out before submitting a PR
    // clippy::todo,
    // clippy::panic_in_result_fn,
    // clippy::panic,
    // clippy::unimplemented,
    // clippy::unreachable,
)]
#![cfg_attr(all(),
doc = ::embed_doc_image::embed_image!("sine_patch", "assets/sine_patch.png"),
doc = ::embed_doc_image::embed_image!("phasor_patch", "assets/phasor_patch.png"),
)]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/alisomay/libpd-rs/main/assets/logo_transparent.png",
    html_favicon_url = "https://raw.githubusercontent.com/alisomay/libpd-rs/main/assets/favicon/favicon.ico"
)]

//! # A safe wrapper around libpd
//!
//! [Pure Data](https://puredata.info/) (Pd) is a visual programming language developed by
//! [Miller Puckette](https://en.wikipedia.org/wiki/Miller_Puckette) in the 1990s
//! for creating interactive computer music and multimedia works.
//! While Puckette is the main author of the program,
//! Pd is an [open-source project](https://github.com/pure-data/pure-data) with a
//! [large developer base](https://github.com/pure-data/pure-data/graphs/contributors) working on new extensions.
//! It is released under [BSD-3-Clause](https://opensource.org/licenses/BSD-3-Clause).
//!
//! Though pd is designed as a desktop application,
//! [libpd](https://github.com/libpd) is an open source project
//! which exposes it as a C library opening the possibility to
//! embed the functionality of pd to any platform which C can compile to.
//!
//! [libpd-rs](https://github.com/alisomay/libpd-rs) aims to bring [libpd](https://github.com/libpd)
//! to the Rust [ecosystem](https://crates.io/).
//! It aims to expose the full functionality of [libpd](https://github.com/libpd)
//! with some extra additions such as bundling commonly used externals
//! and addition of extra functionality for increased ease of use.
//!
//! It is thoroughly [documented](https://docs.rs/libpd-rs/0.1.9/libpd_rs/#),
//! well [tested](https://github.com/alisomay/libpd-rs/tests) and enriched with
//! various [examples](https://github.com/alisomay/libpd-rs/examples) to get you started right away.
//!
//! Now let's make some sound! 🔔
//!
//! ## Getting Started
//!
//! Add the latest version of [libpd-rs](https://github.com/alisomay/libpd-rs) to your `Cargo.toml`:
//! ```toml
//! [dependencies]
//! libpd-rs = "0.3"
//! cpal = "0.15"
//! ```
//! We also add [cpal](https://github.com/RustAudio/cpal) to our dependencies
//! to get access to the high priority audio callback from the OS.
//!
//! [cpal](https://github.com/RustAudio/cpal) is not a must.
//! You may have used any method to get audio callback from the OS.
//!
//! ## Examples and Usage
//!
//! To start making sound with [libpd-rs](https://github.com/alisomay/libpd-rs), we need to have a pd patch at hand.
//! Pd patches are `.pd` files which could be read by pd desktop [application](https://puredata.info/downloads).
//!
//! Pd patches are not binary files, they are simple files full of pd commands as text.
//! [libpd-rs](https://github.com/alisomay/libpd-rs) provides an additional way to
//! [evaluate](crate::Pd::eval_patch) strings as pd patches.
//!
//! This is the [method](crate::Pd::eval_patch) we'll use in the following examples.
//!
//! ### Initialize, open patch, run
//!
//! Paste the code into your `main.rs`:
//!
//! ⚠️ **Warning** ⚠️: This example will produce audio, so please keep your volume at a reasonable level for safety.
//!
//! ```no_run
//! use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
//! use libpd_rs::{Pd, functions::util::calculate_ticks};
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Initialize cpal
//!     // This could have been another cross platform audio library
//!     // basically anything which gets you the audio callback of the os.
//!     let host = cpal::default_host();
//!
//!     // Currently we're only going to output to the default device
//!     let device = host.default_output_device().unwrap();
//!
//!     // Using the default config
//!     let config = device.default_output_config()?;
//!
//!     // Let's get the default configuration from the audio driver.
//!     let sample_rate = config.sample_rate().0 as i32;
//!     let output_channels = config.channels() as i32;
//!
//!     // Initialize libpd with that configuration,
//!     // with no input channels since we're not going to use them.
//!     let mut pd = Pd::init_and_configure(0, output_channels, sample_rate)?;
//!     let ctx = pd.audio_context();
//!
//!     // Let's evaluate a pd patch.
//!     // We could have opened a `.pd` file also.
//!     // This patch would play a sine wave at 440hz.
//!     pd.eval_patch(
//!         r#"
//!     #N canvas 577 549 158 168 12;
//!     #X obj 23 116 dac~;
//!     #X obj 23 17 osc~ 440;
//!     #X obj 23 66 *~ 0.1;
//!     #X obj 81 67 *~ 0.1;
//!     #X connect 1 0 2 0;
//!     #X connect 1 0 3 0;
//!     #X connect 2 0 0 0;
//!     #X connect 3 0 0 1;
//!         "#,
//!     )?;
//!
//!     // Build the audio stream.
//!     let output_stream = device.build_output_stream(
//!         &config.into(),
//!         move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
//!             // Provide the ticks to advance per iteration for the internal scheduler.
//!             let ticks = calculate_ticks(output_channels, data.len() as i32);
//!
//!             // Here if we had an input buffer
//!             // we could have modified it to do pre-processing.
//!
//!             // Process audio, advance internal scheduler.
//!             ctx.process_float(ticks, &[], data);
//!
//!             // Here we could have done post processing
//!             // after pd processed our output buffer in place.
//!         },
//!         |err| eprintln!("an error occurred on stream: {}", err),
//!         None,
//!     )?;
//!
//!     // Turn audio processing on
//!     pd.activate_audio(true)?;
//!
//!     // Run the stream
//!     output_stream.play()?;
//!
//!     // Wait a bit for listening..
//!     std::thread::sleep(std::time::Duration::from_secs(5));
//!
//!     // Turn audio processing off
//!     pd.activate_audio(false)?;
//!
//!     // Pause the stream
//!     output_stream.pause()?;
//!
//!     // Close the patch
//!     pd.close_patch()?;
//!
//!     // Leave
//!     Ok(())
//! }
//! ```
//!
//! The patch you have just evaluated and listened looks exactly like this in pd desktop [application](https://puredata.info/downloads).
//!
//! ![Sine wave generating pd patch][sine_patch]
//!
//! ### Communicate with the patch
//!
//! Again with a simplistic patch, this time we'll send and receive messages from pd.
//! We'll be monitoring our cpu load average over a minute and 5 minutes, send this data to pd
//! as a list and let it change some parameters in our simplistic patch.
//!
//! As a last thing we'll send the data we've applied to an object in the patch  back to Rust to read it.
//! This is a very simple example of how to send and receive data.
//!
//! Add the following dependencies to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! libpd-rs = "0.3"
//! cpal = "0.15"
//! sys-info = "0.9.1"
//! ```
//! Paste the code into your `main.rs`:
//!
//! ⚠️ **Warning** ⚠️: This example will produce audio, so please keep your volume at a reasonable level for safety.
//!
//! ```no_run
//! use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
//! use libpd_rs::{
//!    Pd, functions::{util::calculate_ticks, receive::on_float, send::send_list_to}
//! };
//! use sys_info::loadavg;
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Initialize cpal
//!     // This could have been another cross platform audio library
//!     // basically anything which gets you the audio callback of the os.
//!     let host = cpal::default_host();
//!
//!     // Currently we're only going to output to the default device
//!     let device = host.default_output_device().unwrap();
//!
//!     // Using the default config
//!     let config = device.default_output_config()?;
//!
//!     // Let's get the default configuration from the audio driver.
//!     let sample_rate = config.sample_rate().0 as i32;
//!     let output_channels = config.channels() as i32;
//!
//!     // Initialize libpd with that configuration,
//!     // with no input channels since we're not going to use them.
//!     let mut pd = Pd::init_and_configure(0, output_channels, sample_rate)?;
//!     let ctx = pd.audio_context();
//!
//!     // Let's evaluate another pd patch.
//!     // We could have opened a `.pd` file also.
//!     pd.eval_patch(
//!         r#"
//!     #N canvas 832 310 625 448 12;
//!     #X obj 18 27 r cpu_load;
//!     #X obj 55 394 s response;
//!     #X obj 13 261 *~;
//!     #X obj 112 240 vline~;
//!     #X obj 118 62 bng 15 250 50 0 empty empty empty 17 7 0 10 -262144 -1
//!     -1;
//!     #X obj 14 395 dac~;
//!     #X obj 50 299 sig~;
//!     #X floatatom 50 268 5 0 0 0 - - -;
//!     #X obj 13 228 phasor~ 120;
//!     #X obj 139 61 metro 2000;
//!     #X obj 139 38 tgl 15 0 empty empty empty 17 7 0 10 -262144 -1 -1 1
//!     1;
//!     #X obj 18 52 unpack f f;
//!     #X obj 14 362 *~ 2;
//!     #X obj 14 336 vcf~ 12;
//!     #X obj 139 12 loadbang;
//!     #X msg 118 86 1 8 \, 0 0 10;
//!     #X obj 149 197 expr (480 + 80) * ($f1 - 8) / (4 - 16) + 480;
//!     #X obj 29 128 * 20;
//!     #X obj 167 273 expr (520 + 120) * ($f1 - 5) / (12 - 5) + 120;
//!     #X connect 0 0 11 0;
//!     #X connect 2 0 13 0;
//!     #X connect 3 0 2 1;
//!     #X connect 4 0 15 0;
//!     #X connect 6 0 13 1;
//!     #X connect 7 0 6 0;
//!     #X connect 8 0 2 0;
//!     #X connect 9 0 15 0;
//!     #X connect 10 0 9 0;
//!     #X connect 11 0 16 0;
//!     #X connect 11 0 18 0;
//!     #X connect 11 1 17 0;
//!     #X connect 12 0 5 0;
//!     #X connect 12 0 5 1;
//!     #X connect 13 0 12 0;
//!     #X connect 14 0 10 0;
//!     #X connect 15 0 3 0;
//!     #X connect 16 0 9 1;
//!     #X connect 17 0 1 0;
//!     #X connect 17 0 13 2;
//!     #X connect 18 0 7 0;
//!         "#,
//!     )?;
//!
//!     // Here we are registering a listener (hook in libpd lingo) for
//!     // float values which are received from the pd patch.
//!     on_float(|source, value| {
//!         if source == "response" {
//!             print!("\r");
//!             print!("Pd says that the q value of the vcf~ is: {value}");
//!         }
//!     });
//!
//!     // Pd can send data to many different endpoints at a time.
//!     // This is why we need to declare our subscription to one or more first.
//!     // In this case we're subscribing to one, but it could have been many,
//!     pd.subscribe_to("response")?;
//!
//!     // Build the audio stream.
//!     let output_stream = device.build_output_stream(
//!         &config.into(),
//!         move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
//!             // Provide the ticks to advance per iteration for the internal scheduler.
//!             let ticks = calculate_ticks(output_channels, data.len() as i32);
//!
//!             // Here if we had an input buffer
//!             // we could have modified it to do pre-processing.
//!
//!             // To receive messages from the pd patch we need to read the ring buffers
//!             // filled by the pd patch repeatedly to check if there are messages there.
//!             // Audio callback is a nice place to do that.
//!             ctx.receive_messages_from_pd();
//!
//!             // Process audio, advance internal scheduler.
//!             ctx.process_float(ticks, &[], data);
//!
//!             // Here we could have done post processing after
//!             // pd processed our output buffer in place.
//!         },
//!         |err| eprintln!("an error occurred on stream: {}", err),
//!         None,
//!     )?;
//!
//!     // Turn audio processing on
//!     pd.activate_audio(true)?;
//!
//!     // Run the stream
//!     output_stream.play()?;
//!
//!     // This program does not terminate.
//!     // You would need to explicitly quit it.
//!     loop {
//!         // We sample in 2 hz.
//!         std::thread::sleep(std::time::Duration::from_millis(500));
//!
//!         // Read the average load of the cpu.
//!         let load = loadavg()?;
//!
//!         let one_minute_cpu_load_average = load.one;
//!         let five_minutes_cpu_load_average = load.five;
//!
//!         // Lists are one of the types we can send to pd.
//!         // Although pd allows for heterogeneous lists,
//!         // even if we're not using them heterogeneously in this example,
//!         // we still need to send it as a list of Atoms.
//!
//!         // Atom is an encapsulating type in pd to unify
//!         // various types of data together under the same umbrella.
//!         // Check out `libpd_rs::types` module for more details.
//!
//!         // Atoms have From trait implemented for them for
//!         // floats and strings.
//!         send_list_to(
//!             "cpu_load",
//!             &[
//!                 one_minute_cpu_load_average.into(),
//!                 five_minutes_cpu_load_average.into(),
//!             ],
//!         )?;
//!     }
//! }
//!```
//!
//! The one minute average load is controlling the center frequency of the `vcf~` and the speed of the pulses.
//! The five minute average load is controlling the q factor of the `vcf~`.
//!
//! The result is a pulse which goes higher in pitch and speed when the load is higher and vice versa.
//!
//! Though not very interesting musically, hope it triggers your curiosity and imagination.
//!
//! The patch you have just evaluated and listened looks like this in pd desktop [application](https://puredata.info/downloads).
//!
//! ![Pd patch with phasor a/d and vcf][phasor_patch]
//!
//! ### Note about examples
//!
//! After these basic initial examples which were aimed to get you started, you may dive into the
//! individual [modules](https://docs.rs/libpd-rs/0.1.9/libpd_rs/#modules)
//! and [items](https://docs.rs/libpd-rs/0.1.9/libpd_rs/all.html) in the documentation.
//! They all have their own examples.
//!
//! You may discover [integration tests](https://github.com/alisomay/libpd-rs/tests)
//! and explore [examples](https://github.com/alisomay/libpd-rs/examples)
//! in the [repository](https://github.com/alisomay/libpd-rs).
//!
//! The [examples](https://github.com/alisomay/libpd-rs/examples)
//! directory in the [repository](https://github.com/alisomay/libpd-rs) is not filled with
//! all the examples imagined yet.
//!
//! On the other hand it'll be updated with variety of
//! [new examples](https://github.com/alisomay/libpd-rs/issues/8) very soon.
//!
//! Enjoy!
//!
//! ## Create Layers
//!
//! This crate is thought as 3 layers.
//!
//! ### Low Level
//! These are the sys bindings. You can use them directly if you'd like to through the re-export of [libpd-sys](https://github.com/alisomay/libpd-sys) from this crate.
//!
//! Since [libpd-rs](https://github.com/alisomay/libpd-rs) exhaustively wraps the sys layer, you don't need to use this layer directly.
//!
//! But if you wish, you can do so and create your own abstractions on top of it.
//!
//! ### Mid Level
//! These are the safe wrappers directly around the sys bindings and sometimes with slight extra functionality.
//!
//! You can reach this layer through the [functions](crate::functions) module.
//!
//! While a little bit more high level, here you would still need an understanding of [libpd](https://github.com/libpd) to use it effectively.
//!
//! This module is very heavily documented and tested so to learn further you can continue from here [functions](crate::functions).
//!
//! ### High Level
//! This layer consists of the [Pd](crate::Pd) struct, its methods and the types around it.
//!
//! Here we map one pure data instance to one [Pd](crate::Pd) instance.
//!
//! There are convenient instance management, drop implementations and other high level functionality here.
//!
//! This is the most advised way to use this library.
//!
//! ### Mixing Layers
//!
//! Low level and mid level layers can be used together since one is just a safe wrapper over the other.
//!
//! Mixed usage of mid and the high level layer requires significant understanding of the library and [libpd](https://github.com/libpd) for some functions.
//!
//! ## Plans and Support
//!
//! Please see [libpd-rs](https://github.com/alisomay/libpd-rs) [support](https://github.com/alisomay/libpd-rs#support) and
//! [road map](https://github.com/alisomay/libpd-rs#road-map) section for more details on these topics.
//!
//! Also don't forget to check [issues](https://github.com/alisomay/libpd-rs/issues), to track the ideas and plans.
//!
//! ## Last words
//!
//! Generative or algorithmic music is a powerful tool for exploration,
//! pumps up creativity and goes very well together with traditional music making approaches also.
//!
//! Making apps which produce meaningful sound is difficult,
//! I wish that this crate would ease your way on doing that and make complicated audio ideas
//! in apps accessible to more people.
//!
//! Don't forget to check the [resources](https://github.com/alisomay/libpd-rs#resources)
//! section to expand your knowledge about pure data.
//!
//! Many thanks to [Başak Ünal](https://basakunal.design) for the logo.
//!
//! Happy patching!
//!
//! ## License
//!
//! [BSD-3-Clause](https://opensource.org/licenses/BSD-3-Clause).
//! See [LICENSE](https://raw.githubusercontent.com/alisomay/libpd-rs/main/LICENCE) file.

/// This module exposes [`PdInstance`] struct which covers all the functionality related to pd instances.
///
/// Instances of pd are stored in a thread local way and there can be only one instance at a time could be active per thread.
///
/// The active instance for the thread can be set by calling `set_as_current` method on the instance.
///
/// [`PdInstance`] also has a `Drop` implementation which frees the resources of the instance when it goes out of scope.
pub mod instance;

/// The functions module could be considered as the mid level layer of the library.
///
/// The exhaustive list of functions here reflect the ones exist in [libpd](https://github.com/libpd) directly.
///
/// Since mutating the state of the active instance can also be done through these functions, mixing the high level layer with this layer is not advised.
///
/// If you're familiar with the inner workings of [libpd](https://github.com/libpd) and the library itself, you can use this layer to create your own abstractions.
///
/// There are some functions exposed here which can be safely mixed with the high level layer and some needs more understanding of the internals.
///
/// As long as you know what you're doing though, you can mix the high level layer with this layer when necessary.
pub mod functions;

/// Types for working with pd
///
/// Pd wraps primitive types such as a float or a string in a type called atom.
/// This enables pd to have heterogenous lists.
///
/// This module exposes the representation of that type as a Rust enum, [`Atom`].
///
/// It also exposes some others to hold file or receiver handles returned from libpd functions.
pub mod types;

/// All errors
///
/// This module contains all the errors which can be returned by the library.
pub mod error;

/// The atom module contains the Atom enum which is used to represent pd's atom type in Rust.
pub mod atom;

use atom::make_atom_list_from_t_atom_list;
use error::{PdError, RecieveError, SendError, SizeError, SubscriptionError, C_STR_FAILURE};
use libffi::high::{
    ClosureMut1, ClosureMut2, ClosureMut3, ClosureMut4, FnPtr1, FnPtr2, FnPtr3, FnPtr4,
};
use libpd_sys::{
    _pdinstance, t_libpd_aftertouchhook, t_libpd_banghook, t_libpd_controlchangehook,
    t_libpd_doublehook, t_libpd_floathook, t_libpd_listhook, t_libpd_messagehook,
    t_libpd_midibytehook, t_libpd_noteonhook, t_libpd_pitchbendhook, t_libpd_polyaftertouchhook,
    t_libpd_printhook, t_libpd_programchangehook, t_libpd_symbolhook,
};
use std::{
    collections::HashMap,
    ffi::CStr,
    path::{Path, PathBuf},
    {fs, os, ptr, slice},
};
use tempfile::NamedTempFile;

use crate::{
    error::PatchLifeCycleError,
    instance::PdInstance,
    types::{PatchFileHandle, ReceiverHandle},
};

pub use atom::Atom;
/// Re-exports of the libpd-sys crate.
pub use libpd_sys;

/// An abstraction provided for convenience to express a pure data instance, track the state and execute some common functions.
///
/// This struct represents a single instance of pd.
///
/// You may create as many instances you like but only one of them can be active at a time.
///
/// **It is strongly advised to keep the very first instance alive through the lifetime of the application.**
///
/// See [`PdInstance`] for more details about this topic.
///
/// After created and registered internally the instance lives in libpd's memory.
/// Dropping this struct will free the resources of the instance.
///
/// It is also important to note that the mid-level layer of this library [`functions`] also can modify the state of this instance.
///
/// To learn more about how instances are created and managed, please see the [`instance`] module level documentation.
///
/// This is why that if you're not very familiar with this library's and libpd's source code you shouldn't mix the high level layer with the mid level layer.
///
/// # Example of an unwanted mix
///
/// ```rust
/// use libpd_rs::Pd;
/// use libpd_rs::functions::util::dsp_off;
///
/// let mut pd = Pd::init_and_configure(1, 2, 44100).unwrap();
///
/// // We call the method of [`Pd`] to activate audio
/// // which calls [`dsp_on`] internally which then sends a message
/// // to globally initialized pd to activate dsp.
/// pd.activate_audio(true).unwrap();
///
/// // So far so good.
/// assert_eq!(pd.audio_active(), true);
///
/// // But we can send messages to globally initialized pd many ways
/// // and here is one of the ways we can do it.
/// dsp_off().unwrap();
///
/// // But now [`Pd`] is not aware of the mutated state
/// // of the instance in the background.
/// // The information it holds is outdated and not true anymore.
/// assert_eq!(pd.audio_active(), true);
/// ```
///
/// To avoid this situation if you use [`Pd`] check its methods, only use them and **not** their function counterparts.
///
/// If you really need to mix the layers, you should read the source of the relevant part before doing so.
pub struct Pd {
    inner: PdInstance,
    audio_active: bool,
    input_channels: i32,
    output_channels: i32,
    sample_rate: i32,
    sent_message_info: Option<SentMessageInfo>,
    /// A store to keep track of puredata callback functions to be dropped when the instance goes out of scope.
    callbacks: Callbacks,
    running_patch: Option<PatchFileHandle>,
    temporary_evaluated_patch: Option<NamedTempFile>,
    /// A store to keep track of subscriptions which are made to senders in pd through the app lifecycle.
    pub subscriptions: HashMap<String, ReceiverHandle>,
    /// A store to keep track of paths which are added to pd search paths through the app lifecycle.
    pub search_paths: Vec<PathBuf>,
}

type PrintHookCodePtr<'a> = *const FnPtr1<'a, *const i8, ()>;
type BangHookCodePtr<'a> = *const FnPtr1<'a, *const i8, ()>;
type FloatHookCodePtr<'a> = *const FnPtr2<'a, *const i8, f32, ()>;
type DoubleHookCodePtr<'a> = *const FnPtr2<'a, *const i8, f64, ()>;
type SymbolHookCodePtr<'a> = *const FnPtr2<'a, *const i8, *const i8, ()>;
type ListHookCodePtr<'a> = *const FnPtr3<'a, *const i8, i32, *mut libpd_sys::_atom, ()>;
type MessageHookCodePtr<'a> =
    *const FnPtr4<'a, *const i8, *const i8, i32, *mut libpd_sys::_atom, ()>;

type MidiNoteOnCodePtr<'a> = *const FnPtr3<'a, i32, i32, i32, ()>;
type MidiControlChangeCodePtr<'a> = *const FnPtr3<'a, i32, i32, i32, ()>;
type MidiProgramChangeCodePtr<'a> = *const FnPtr2<'a, i32, i32, ()>;
type MidiPitchBendCodePtr<'a> = *const FnPtr2<'a, i32, i32, ()>;
type MidiAfterTouchCodePtr<'a> = *const FnPtr2<'a, i32, i32, ()>;
type MidiPolyAfterTouchCodePtr<'a> = *const FnPtr3<'a, i32, i32, i32, ()>;
type MidiByteCodePtr<'a> = *const FnPtr2<'a, i32, i32, ()>;

const GUARD_FROM_CALLBACK_DURING_DSP: bool = false;

impl Pd {
    /// Initializes a pd instance.
    ///
    /// It calls [`PdInstance::new`](crate::instance::PdInstance::new) and [`initialize_audio`](crate::functions::initialize_audio) with the provided arguments and returns an instance where a user can keep simple state and call some convenience methods.
    ///
    /// This method will not set the newly created instance as the active instance for the thread.
    ///
    /// You may crate any number of instances of [`Pd`] but only one of them can be active at a time.
    /// Many of the methods in this struct would set it as the active instance before operating and reset it to the last set active after.
    ///
    /// It is your duty to keep this struct alive as long as you need to use it.
    /// When it goes out of scope, the instance will be dropped and the pd instance it wraps will be destroyed.
    ///
    /// # Examples
    /// ```rust
    /// use libpd_rs::Pd;
    ///
    /// let mut pd = Pd::init_and_configure(1, 2, 44100).unwrap();
    /// ```
    ///
    /// # Errors
    ///
    /// A list of errors that can occur:
    /// - [`InitializationError`](crate::error::InitializationError)
    ///   - [`RingBufferInitializationError`](crate::error::InitializationError::RingBufferInitializationError)
    ///   - [`InitializationFailed`](crate::error::InitializationError::InitializationFailed)
    /// - [`AudioInitializationError`](crate::error::AudioInitializationError)
    ///   - [`InitializationFailed`](crate::error::AudioInitializationError::InitializationFailed)
    pub fn init_and_configure(
        input_channels: i32,
        output_channels: i32,
        sample_rate: i32,
    ) -> Result<Self, PdError> {
        let inner = PdInstance::new()?;
        functions::initialize_audio(input_channels, output_channels, sample_rate)?;
        Ok(Self {
            inner,
            audio_active: false,
            input_channels,
            output_channels,
            sample_rate,
            sent_message_info: None,
            callbacks: Callbacks::new(),
            running_patch: None,
            temporary_evaluated_patch: None,
            subscriptions: HashMap::default(),
            search_paths: vec![],
        })
    }

    /// Returns a reference to the inner pd instance.
    pub const fn inner(&self) -> &PdInstance {
        &self.inner
    }

    /// Returns a mutable reference to the inner pd instance.
    pub const fn inner_mut(&mut self) -> &mut PdInstance {
        &mut self.inner
    }

    /// Creates an audio context for this instance to be easily passed in to the audio thread.
    pub fn audio_context(&self) -> PdAudioContext {
        PdAudioContext {
            instance: self.inner.clone(),
        }
    }

    /// Set this instance as the current active instance for the thread.
    pub fn set_as_current(&self) {
        self.inner.set_as_current();
    }

    /// Returns the number of the instance.
    pub const fn instance_number(&self) -> i32 {
        self.inner.number()
    }

    /// Checks if this instance is the main instance.
    ///
    /// The main instance is always valid.
    pub fn is_main_instance(&self) -> bool {
        self.inner.is_main_instance()
    }

    /// Checks if this instance is the current active instance for the thread.
    pub fn is_current_instance(&self) -> bool {
        self.inner.is_current_instance()
    }

    /// Sets this instance as the active instance for the thread until the returned guard is dropped.
    ///
    /// If the guard is dropped, the previously active instance will be set as the active instance.
    ///
    /// If the previous instance is null this guard will set the main instance as the active instance since that is always valid.
    pub(crate) fn set_as_active_instance(&self) -> ActiveInstanceGuard {
        if self.inner.is_current_instance() {
            // This would render the guard useless and as a no-op on drop which is what we want.
            return ActiveInstanceGuard::wrap(ptr::null_mut::<_pdinstance>());
        }
        let previous_instance = unsafe { libpd_sys::libpd_this_instance() };
        self.inner.set_as_current();
        ActiveInstanceGuard::wrap(previous_instance)
    }

    /// Adds a path to the list of paths where this instance searches in.
    ///
    /// Relative paths are relative to the current working directory.
    /// Unlike the desktop pd application, **no** search paths are set by default.
    ///
    /// # Errors
    ///
    /// A list of errors that can occur:
    /// - [`IoError`](crate::error::IoError)
    ///   - [`PathDoesNotExist`](crate::error::IoError::PathDoesNotExist)
    pub fn add_path_to_search_paths<T: AsRef<Path>>(&mut self, path: T) -> Result<(), PdError> {
        let _guard = self.set_as_active_instance();
        let path = path.as_ref().to_path_buf();
        if !self.search_paths.contains(&path) {
            functions::add_to_search_paths(path.clone())?;
            self.search_paths.push(path);
        }
        Ok(())
    }

    /// Adds many paths to the list of paths where this instance searches in.
    ///
    /// Relative paths are relative to the current working directory.
    /// Unlike the desktop pd application, **no** search paths are set by default.
    ///
    /// # Errors
    ///
    /// A list of errors that can occur:
    /// - [`IoError`](crate::error::IoError)
    ///   - [`PathDoesNotExist`](crate::error::IoError::PathDoesNotExist)
    pub fn add_paths_to_search_paths<T: AsRef<Path>>(
        &mut self,
        paths: &[T],
    ) -> Result<(), PdError> {
        let _guard = self.set_as_active_instance();
        for path in paths {
            if !self.search_paths.contains(&path.as_ref().to_path_buf()) {
                functions::add_to_search_paths(path)?;
                self.search_paths.push(path.as_ref().to_path_buf());
            }
        }
        Ok(())
    }

    /// Clears all the paths where this instance searches for patches and assets.
    pub fn clear_all_search_paths(&mut self) {
        let _guard = self.set_as_active_instance();
        functions::clear_search_paths();
        self.search_paths.clear();
    }

    /// Closes a pd patch for this instance.
    ///
    /// # Errors
    ///
    /// A list of errors that can occur:
    /// - [`PatchLifeCycleError`]
    ///   - [`FailedToClosePatch`](crate::error::PatchLifeCycleError::FailedToClosePatch)
    pub fn close_patch(&mut self) -> Result<(), PdError> {
        let _guard = self.set_as_active_instance();
        if let Some(handle) = self.running_patch.take() {
            functions::close_patch(handle)?;
        }
        self.temporary_evaluated_patch.take();
        Ok(())
    }

    /// Opens a pd patch for this instance.
    ///
    /// The argument should be an absolute path to the patch file.
    /// Absolute and relative paths are supported.
    /// Relative paths and single file names are tried in executable directory and manifest directory.
    ///
    /// Tha function **first** checks the executable directory and **then** the manifest directory.
    ///
    /// # Examples
    /// ```no_run
    /// use libpd_rs::Pd;
    ///
    /// let mut pd = Pd::init_and_configure(1, 2, 44100).unwrap();
    /// assert!(pd.open_patch("tests/patches/sine.pd").is_ok());
    /// ```
    ///
    /// # Errors
    ///
    /// A list of errors that can occur:
    /// - [`PatchLifeCycleError`]
    ///   - [`FailedToClosePatch`](crate::error::PatchLifeCycleError::FailedToClosePatch)
    ///   - [`FailedToOpenPatch`](crate::error::PatchLifeCycleError::FailedToOpenPatch)
    ///   - [`PathDoesNotExist`](crate::error::PatchLifeCycleError::PathDoesNotExist)
    pub fn open_patch<T: AsRef<Path>>(&mut self, path: T) -> Result<(), PdError> {
        let _guard = self.set_as_active_instance();
        if self.running_patch.is_some() {
            self.close_patch()?;
        }
        self.running_patch = Some(functions::open_patch(path)?);
        Ok(())
    }

    /// Evaluate a string as a pd patch for this instance.
    ///
    /// This function creates a temporary file with the contents passed behind the scenes.
    /// and saves it into the [`Pd`] struct holding onto it until the patch is closed or the instantiated [`Pd`] is dropped.
    ///
    /// Note: The patch opened after this evaluation could be closed safely with [`close_patch`](Pd::close_patch).
    ///
    /// # Examples
    /// ```rust
    /// use libpd_rs::Pd;
    ///
    /// let mut pd = Pd::init_and_configure(1, 2, 44100).unwrap();
    ///     
    /// assert!(pd.eval_patch(
    /// r#"
    /// #N canvas 577 549 158 168 12;
    /// #X obj 23 116 dac~;
    /// #X obj 23 17 osc~ 440;
    /// #X obj 23 66 *~ 0.1;
    /// #X obj 81 67 *~ 0.1;
    /// #X connect 1 0 2 0;
    /// #X connect 1 0 3 0;
    /// #X connect 2 0 0 0;
    /// #X connect 3 0 0 1;
    /// "#
    /// ,).is_ok());
    /// ```
    ///
    /// # Errors
    ///
    /// A list of errors that can occur:
    /// - [`PatchLifeCycleError`]
    ///   - [`FailedToEvaluateAsPatch`](crate::error::PatchLifeCycleError::FailedToEvaluateAsPatch)
    ///   - [`FailedToClosePatch`](crate::error::PatchLifeCycleError::FailedToClosePatch)
    ///   - [`FailedToOpenPatch`](crate::error::PatchLifeCycleError::FailedToOpenPatch)
    ///   - [`PathDoesNotExist`](crate::error::PatchLifeCycleError::PathDoesNotExist)
    pub fn eval_patch<T: AsRef<str>>(&mut self, contents: T) -> Result<(), PdError> {
        let _guard = self.set_as_active_instance();
        if self.running_patch.is_some() {
            self.close_patch()?;
        }
        let temp_file =
            NamedTempFile::new().map_err(|err| PatchLifeCycleError::FailedToEvaluateAsPatch {
                content: contents.as_ref().to_owned(),
                msg: err.to_string(),
            })?;
        fs::write(temp_file.path(), contents.as_ref()).map_err(|err| {
            PatchLifeCycleError::FailedToEvaluateAsPatch {
                content: contents.as_ref().to_owned(),
                msg: err.to_string(),
            }
        })?;
        self.running_patch = Some(functions::open_patch(temp_file.path())?);
        self.temporary_evaluated_patch = Some(temp_file);
        Ok(())
    }

    /// Starts listening messages from a source.
    ///
    /// If the source is already being listened to, this function will early return not doing anything without an error.
    ///
    /// # Examples
    /// ```no_run
    /// use libpd_rs::Pd;
    ///
    /// let mut pd = Pd::init_and_configure(1, 2, 44100).unwrap();
    /// pd.open_patch("tests/patches/sine.pd").unwrap();
    /// pd.subscribe_to("sender").unwrap();
    /// ```
    ///
    /// # Errors
    ///
    /// A list of errors that can occur:
    /// - [`SubscriptionError`]
    ///   - [`FailedToSubscribeToSender`](crate::error::SubscriptionError::FailedToSubscribeToSender)
    pub fn subscribe_to<T: AsRef<str>>(&mut self, source: T) -> Result<(), PdError> {
        let _guard = self.set_as_active_instance();
        if self.subscriptions.contains_key(source.as_ref()) {
            return Ok(());
        }
        self.subscriptions.insert(
            source.as_ref().to_owned(),
            functions::receive::start_listening_from(source.as_ref())?,
        );
        Ok(())
    }

    /// Starts listening messages from many sources.
    ///
    /// If the any source is already being listened to, this function will will ignore them.
    ///
    /// # Examples
    /// ```no_run
    /// use libpd_rs::Pd;
    ///
    /// let mut pd = Pd::init_and_configure(1, 2, 44100).unwrap();
    /// pd.open_patch("tests/patches/sine.pd").unwrap();
    /// pd.subscribe_to_many(&["sender", "other_sender"]).unwrap();
    /// ```
    ///
    /// # Errors
    ///
    /// A list of errors that can occur:
    /// - [`SubscriptionError`]
    ///   - [`FailedToSubscribeToSender`](crate::error::SubscriptionError::FailedToSubscribeToSender)
    pub fn subscribe_to_many<T: AsRef<str>>(&mut self, sources: &[T]) -> Result<(), PdError> {
        let _guard = self.set_as_active_instance();
        for source in sources {
            if self.subscriptions.contains_key(source.as_ref()) {
                continue;
            }
            self.subscriptions.insert(
                source.as_ref().to_owned(),
                functions::receive::start_listening_from(source.as_ref())?,
            );
        }
        Ok(())
    }

    /// Stops listening messages from a source.
    ///
    /// # Examples
    /// ```no_run
    /// use libpd_rs::Pd;
    ///
    /// let mut pd = Pd::init_and_configure(1, 2, 44100).unwrap();
    /// pd.open_patch("tests/patches/sine.pd").unwrap();
    /// pd.subscribe_to("sender").unwrap();
    /// pd.unsubscribe_from("sender");
    /// ```
    pub fn unsubscribe_from<T: AsRef<str>>(&mut self, source: T) {
        let _guard = self.set_as_active_instance();
        if let Some(handle) = self.subscriptions.remove(source.as_ref()) {
            functions::receive::stop_listening_from(handle);
        }
    }

    /// Stops listening messages from many sources.
    ///
    /// # Examples
    /// ```no_run
    /// use libpd_rs::Pd;
    ///
    /// let mut pd = Pd::init_and_configure(1, 2, 44100).unwrap();
    /// pd.open_patch("tests/patches/sine.pd").unwrap();
    /// pd.subscribe_to_many(&["sender", "other_sender"]).unwrap();
    ///
    /// pd.unsubscribe_from_many(&["sender", "other_sender"]);
    /// ```
    pub fn unsubscribe_from_many<T: AsRef<str>>(&mut self, sources: &[T]) {
        let _guard = self.set_as_active_instance();
        for source in sources {
            if let Some(handle) = self.subscriptions.remove(source.as_ref()) {
                functions::receive::stop_listening_from(handle);
            }
        }
    }

    /// Stops listening from all sources.
    ///
    /// # Examples
    /// ```no_run
    /// use libpd_rs::Pd;
    ///
    /// let mut pd = Pd::init_and_configure(1, 2, 44100).unwrap();
    /// pd.open_patch("tests/patches/sine.pd").unwrap();
    /// pd.subscribe_to_many(&["sender", "other_sender"]).unwrap();
    ///
    /// pd.unsubscribe_from_all();
    /// ```
    pub fn unsubscribe_from_all(&mut self) {
        let _guard = self.set_as_active_instance();
        let sources: Vec<String> = self.subscriptions.keys().cloned().collect();
        for source in &sources {
            if let Some(handle) = self.subscriptions.remove(source) {
                functions::receive::stop_listening_from(handle);
            }
        }
    }

    /// Gets the `$0` of the running patch.
    ///
    /// `$0` id in pd could be thought as a auto generated unique identifier for the patch.
    ///
    /// # Errors
    ///
    /// A list of errors that can occur:
    /// - [`PatchLifeCycleError`]
    ///   - [`PatchIsNotOpen`](crate::error::PatchLifeCycleError::PatchIsNotOpen)
    pub fn dollar_zero(&mut self) -> Result<i32, PdError> {
        let _guard = self.set_as_active_instance();
        if let Some(ref patch) = self.running_patch {
            let dollar_zero = functions::get_dollar_zero(patch)?;
            return Ok(dollar_zero);
        }
        Err(PatchLifeCycleError::PatchIsNotOpen.into())
    }

    /// Checks if the audio is active.
    ///
    /// # Important
    ///
    /// The state is tracked by libpd internally for the instance and we expose other functions to modify this state.
    ///
    /// If messages sent to pd previously used another way to modify this information, this state might not reflect reality.
    pub const fn audio_active(&self) -> bool {
        self.audio_active
    }

    /// Activates or deactivates audio in pd.
    ///
    /// # Errors
    ///
    /// A list of errors that can occur:
    /// - [`SendError`]
    ///   - [`MissingDestination`](crate::error::SendError::MissingDestination)
    /// - [`SizeError`]
    ///   - [`TooLarge`](crate::error::SizeError::TooLarge)
    pub fn activate_audio(&mut self, on: bool) -> Result<(), PdError> {
        if on {
            self.dsp_on()
        } else {
            self.dsp_off()
        }
    }

    /// Activates audio in pd.
    ///
    /// # Errors
    ///
    /// A list of errors that can occur:
    /// - [`SendError`]
    ///   - [`MissingDestination`](crate::error::SendError::MissingDestination)
    /// - [`SizeError`]
    ///   - [`TooLarge`](crate::error::SizeError::TooLarge)
    pub fn dsp_on(&mut self) -> Result<(), PdError> {
        if self.audio_active {
            return Ok(());
        }

        let _guard = self.set_as_active_instance();
        functions::util::dsp_on()?;
        self.audio_active = true;
        Ok(())
    }

    /// Deactivates audio in pd.
    ///
    /// # Errors
    ///
    /// A list of errors that can occur:
    /// - [`SendError`]
    ///   - [`MissingDestination`](crate::error::SendError::MissingDestination)
    /// - [`SizeError`]
    ///   - [`TooLarge`](crate::error::SizeError::TooLarge)
    pub fn dsp_off(&mut self) -> Result<(), PdError> {
        if !self.audio_active {
            return Ok(());
        }

        let _guard = self.set_as_active_instance();
        functions::util::dsp_off()?;
        self.audio_active = false;
        Ok(())
    }

    /// Gets the sample rate which pd is configured with.
    ///
    /// # Important
    ///
    /// The state is tracked by libpd internally for the instance and we expose other functions to modify this state.
    ///
    /// If messages sent to pd previously used another way to modify this information, this state might not reflect reality.
    #[must_use]
    pub const fn sample_rate(&self) -> i32 {
        self.sample_rate
    }

    /// Gets the number of input channels which pd is configured with.
    ///
    /// # Important
    ///
    /// The state is tracked by libpd internally for the instance and we expose other functions to modify this state.
    ///
    /// If messages sent to pd previously used another way to modify this information, this state might not reflect reality.
    #[must_use]
    pub const fn input_channels(&self) -> i32 {
        self.input_channels
    }

    /// Gets the number of output channels which pd is configured with.
    ///
    /// # Important
    ///
    /// The state is tracked by libpd internally for the instance and we expose other functions to modify this state.
    ///
    /// If messages sent to pd previously used another way to modify this information, this state might not reflect reality.
    #[must_use]
    pub const fn output_channels(&self) -> i32 {
        self.output_channels
    }

    /// Calls [`send_bang_to`](crate::functions::send::send_bang_to) for this instance.
    ///
    /// # Errors
    /// - See [`send_bang_to`](crate::functions::send::send_bang_to).
    pub fn send_bang_to<T: AsRef<str>>(&self, receiver: T) -> Result<(), SendError> {
        let _guard = self.set_as_active_instance();
        functions::send::send_bang_to(receiver)
    }

    /// Calls [`send_float_to`](crate::functions::send::send_float_to) for this instance.
    ///
    /// # Errors
    /// - See [`send_float_to`](crate::functions::send::send_float_to).
    pub fn send_float_to<T: AsRef<str>>(&self, receiver: T, value: f32) -> Result<(), SendError> {
        let _guard = self.set_as_active_instance();
        functions::send::send_float_to(receiver, value)
    }

    /// Calls [`send_double_to`](crate::functions::send::send_double_to) for this instance.
    ///
    /// # Errors
    /// - See [`send_double_to`](crate::functions::send::send_double_to).
    pub fn send_double_to<T: AsRef<str>>(&self, receiver: T, value: f64) -> Result<(), SendError> {
        let _guard = self.set_as_active_instance();
        functions::send::send_double_to(receiver, value)
    }

    /// Calls [`send_symbol_to`](crate::functions::send::send_symbol_to) for this instance.
    ///
    /// # Errors
    /// - See [`send_symbol_to`](crate::functions::send::send_symbol_to).
    pub fn send_symbol_to<T: AsRef<str>, S: AsRef<str>>(
        &self,
        receiver: T,
        value: S,
    ) -> Result<(), SendError> {
        let _guard = self.set_as_active_instance();
        functions::send::send_symbol_to(receiver, value)
    }

    /// Calls [`start_message`](crate::functions::send::start_message) for this instance.
    /// Will panic if length is negative.
    ///
    /// # Errors
    /// - See [`start_message`](crate::functions::send::start_message).
    pub fn start_message(&mut self, length: i32) -> Result<(), PdError> {
        if self.sent_message_info.is_some() {
            return Err(SendError::MessageAlreadyStarted.into());
        }

        let _guard = self.set_as_active_instance();
        functions::send::start_message(length)?;

        self.sent_message_info = Some(SentMessageInfo::new(length)?);
        Ok(())
    }

    /// Calls [`add_float_to_started_message`](crate::functions::send::add_float_to_started_message) for this instance.
    ///
    /// # Errors
    ///
    /// A list of errors that can occur in addition to [`add_float_to_started_message`](crate::functions::send::add_float_to_started_message):
    /// - [`SendError`]
    ///    - [`MessageNotStarted`](crate::error::SendError::MessageNotStarted) if [`Self::start_message`] was never called.
    ///    - [`OutOfRange`](crate::error::SendError::OutOfRange) if the number of elements added to the started message is above its capacity.
    pub fn add_float_to_started_message(&mut self, value: f32) -> Result<(), SendError> {
        self.sent_message_info
            .as_mut()
            .ok_or(SendError::MessageNotStarted)?
            .increment()?;

        let _guard = self.set_as_active_instance();

        functions::send::add_float_to_started_message(value);

        Ok(())
    }

    /// Calls [`add_double_to_started_message`](crate::functions::send::add_double_to_started_message) for this instance.
    ///
    /// # Errors
    ///
    /// A list of errors that can occur in addition to [`add_double_to_started_message`](crate::functions::send::add_double_to_started_message):
    /// - [`SendError`]
    ///    - [`MessageNotStarted`](crate::error::SendError::MessageNotStarted) if [`Self::start_message`] was never called.
    ///    - [`OutOfRange`](crate::error::SendError::OutOfRange) if the number of elements added to the started message is above its capacity.
    pub fn add_double_to_started_message(&mut self, value: f64) -> Result<(), SendError> {
        self.sent_message_info
            .as_mut()
            .ok_or(SendError::MessageNotStarted)?
            .increment()?;

        let _guard = self.set_as_active_instance();

        functions::send::add_double_to_started_message(value);

        Ok(())
    }

    /// Calls [`add_symbol_to_started_message`](crate::functions::send::add_symbol_to_started_message) for this instance.
    ///
    /// # Errors
    ///
    /// A list of errors that can occur in addition to [`add_symbol_to_started_message`](crate::functions::send::add_symbol_to_started_message):
    /// - [`SendError`]
    ///    - [`MessageNotStarted`](crate::error::SendError::MessageNotStarted) if [`Self::start_message`] was never called.
    ///    - [`OutOfRange`](crate::error::SendError::OutOfRange) if the number of elements added to the started message is above its capacity.
    pub fn add_symbol_to_started_message<T: AsRef<str>>(
        &mut self,
        value: T,
    ) -> Result<(), SendError> {
        self.sent_message_info
            .as_mut()
            .ok_or(SendError::MessageNotStarted)?
            .increment()?;

        let _guard = self.set_as_active_instance();

        functions::send::add_symbol_to_started_message(value)
    }

    /// Calls [`finish_message_as_list_and_send_to`](crate::functions::send::finish_message_as_list_and_send_to) for this instance.
    ///
    /// # Errors
    ///
    /// A list of errors that can occur in addition to [`finish_message_as_list_and_send_to`](crate::functions::send::finish_message_as_list_and_send_to):
    /// - [`SendError`]
    ///    - [`MessageNotStarted`](crate::error::SendError::MessageNotStarted) if [`Self::start_message`] was never called.
    pub fn finish_message_as_list_and_send_to<T: AsRef<str>>(
        &mut self,
        receiver: T,
    ) -> Result<(), SendError> {
        // Ensure that the message was actually started before sending.
        self.sent_message_info
            .take()
            .ok_or(SendError::MessageNotStarted)?;

        let _guard = self.set_as_active_instance();
        functions::send::finish_message_as_list_and_send_to(receiver)
    }

    /// Calls [`finish_message_as_typed_message_and_send_to`](crate::functions::send::finish_message_as_typed_message_and_send_to) for this instance.
    ///
    /// # Errors
    ///
    /// A list of errors that can occur in addition to [`finish_message_as_typed_message_and_send_to`](crate::functions::send::finish_message_as_typed_message_and_send_to):
    /// - [`SendError`]
    ///    - [`MessageNotStarted`](crate::error::SendError::MessageNotStarted) if [`Self::start_message`] was never called.
    ///    - [`OutOfRange`](crate::error::SendError::OutOfRange) If more than four items are sent. Messages above four elements cannot be typed.
    pub fn finish_message_as_typed_message_and_send_to<T: AsRef<str>, S: AsRef<str>>(
        &mut self,
        receiver: T,
        message_header: S,
    ) -> Result<(), SendError> {
        if self
            .sent_message_info
            .take()
            .ok_or(SendError::MessageNotStarted)?
            .length
            > 4
        {
            return Err(SendError::OutOfRange);
        }

        let _guard = self.set_as_active_instance();
        functions::send::finish_message_as_typed_message_and_send_to(receiver, message_header)
    }

    /// Calls [`send_list_to`](crate::functions::send::send_list_to) for this instance.
    ///
    /// # Errors
    /// - See [`send_list_to`](crate::functions::send::send_list_to).
    pub fn send_list_to<T: AsRef<str>>(&self, receiver: T, list: &[Atom]) -> Result<(), PdError> {
        let _guard = self.set_as_active_instance();
        functions::send::send_list_to(receiver, list)
    }

    /// Calls [`send_message_to`](crate::functions::send::send_message_to) for this instance.
    ///
    /// # Errors
    /// - See [`send_message_to`](crate::functions::send::send_message_to).
    pub fn send_message_to<T: AsRef<str>>(
        &self,
        receiver: T,
        message: T,
        list: &[Atom],
    ) -> Result<(), PdError> {
        let _guard = self.set_as_active_instance();
        functions::send::send_message_to(receiver, message, list)
    }

    /// Calls [`send_note_on`](crate::functions::send::send_note_on) for this instance.
    ///
    /// # Errors
    /// - See [`send_note_on`](crate::functions::send::send_note_on).
    pub fn send_note_on(&self, channel: i32, pitch: i32, velocity: i32) -> Result<(), SendError> {
        let _guard = self.set_as_active_instance();
        functions::send::send_note_on(channel, pitch, velocity)
    }

    /// Calls [`send_control_change`](crate::functions::send::send_control_change) for this instance.
    ///
    /// # Errors
    /// - See [`send_control_change`](crate::functions::send::send_control_change).
    pub fn send_control_change(
        &self,
        channel: i32,
        controller: i32,
        value: i32,
    ) -> Result<(), SendError> {
        let _guard = self.set_as_active_instance();
        functions::send::send_control_change(channel, controller, value)
    }

    /// Calls [`send_program_change`](crate::functions::send::send_program_change) for this instance.
    ///
    /// # Errors
    /// - See [`send_program_change`](crate::functions::send::send_program_change).
    pub fn send_program_change(&self, channel: i32, value: i32) -> Result<(), SendError> {
        let _guard = self.set_as_active_instance();
        functions::send::send_program_change(channel, value)
    }

    /// Calls [`send_pitch_bend`](crate::functions::send::send_pitch_bend) for this instance.
    ///
    /// # Errors
    /// - See [`send_pitch_bend`](crate::functions::send::send_pitch_bend).
    pub fn send_pitch_bend(&self, channel: i32, value: i32) -> Result<(), SendError> {
        let _guard = self.set_as_active_instance();
        functions::send::send_pitch_bend(channel, value)
    }

    /// Calls [`send_after_touch`](crate::functions::send::send_after_touch) for this instance.
    ///
    /// # Errors
    /// - See [`send_after_touch`](crate::functions::send::send_after_touch).
    pub fn send_after_touch(&self, channel: i32, value: i32) -> Result<(), SendError> {
        let _guard = self.set_as_active_instance();
        functions::send::send_after_touch(channel, value)
    }

    /// Calls [`send_poly_after_touch`](crate::functions::send::send_poly_after_touch) for this instance.
    ///
    /// # Errors
    /// - See [`send_poly_after_touch`](crate::functions::send::send_poly_after_touch).
    pub fn send_poly_after_touch(
        &self,
        channel: i32,
        pitch: i32,
        value: i32,
    ) -> Result<(), SendError> {
        let _guard = self.set_as_active_instance();
        functions::send::send_poly_after_touch(channel, pitch, value)
    }

    /// Calls [`send_midi_byte`](crate::functions::send::send_midi_byte) for this instance.
    ///
    /// # Errors
    /// - See [`send_midi_byte`](crate::functions::send::send_midi_byte).
    pub fn send_midi_byte(&self, port: i32, byte: i32) -> Result<(), SendError> {
        let _guard = self.set_as_active_instance();
        functions::send::send_midi_byte(port, byte)
    }

    /// Calls [`send_sysex`](crate::functions::send::send_sysex) for this instance.
    ///
    /// # Errors
    /// - See [`send_sysex`](crate::functions::send::send_sysex).
    pub fn send_sysex(&self, port: i32, byte: i32) -> Result<(), SendError> {
        let _guard = self.set_as_active_instance();
        functions::send::send_sysex(port, byte)
    }

    /// Calls [`send_sys_realtime`](crate::functions::send::send_sys_realtime) for this instance.
    ///
    /// # Errors
    /// - See [`send_sys_realtime`](crate::functions::send::send_sys_realtime).
    pub fn send_sys_realtime(&self, port: i32, byte: i32) -> Result<(), SendError> {
        let _guard = self.set_as_active_instance();
        functions::send::send_sys_realtime(port, byte)
    }

    /// Calls [`start_listening_from`](crate::functions::receive::start_listening_from) for this instance.
    ///
    /// # Errors
    /// - See [`start_listening_from`](crate::functions::receive::start_listening_from).
    pub fn start_listening_from<T: AsRef<str>>(
        &self,
        sender: T,
    ) -> Result<ReceiverHandle, SubscriptionError> {
        let _guard = self.set_as_active_instance();
        functions::receive::start_listening_from(sender)
    }

    /// Calls [`stop_listening_from`](crate::functions::receive::stop_listening_from) for this instance.
    pub fn stop_listening_from(&self, source: ReceiverHandle) {
        let _guard = self.set_as_active_instance();
        functions::receive::stop_listening_from(source);
    }

    /// Calls [`source_to_listen_from_exists`](crate::functions::receive::source_to_listen_from_exists) for this instance.
    ///
    /// # Errors
    /// - See [`source_to_listen_from_exists`](crate::functions::receive::source_to_listen_from_exists).
    pub fn source_to_listen_from_exists<T: AsRef<str>>(
        &self,
        sender: T,
    ) -> Result<bool, SubscriptionError> {
        let _guard = self.set_as_active_instance();
        functions::receive::source_to_listen_from_exists(sender)
    }

    /// Instance-safe version of [`on_print`](crate::functions::receive::on_print) which doesn't leak memory.
    ///
    /// # Errors
    /// - [`RecieveError`]
    ///    - [`DspActive`](crate::error::RecieveError::DspActive)
    pub fn on_print<'a, F: FnMut(&str) + 'a>(
        &'a mut self,
        mut callback: F,
    ) -> Result<(), RecieveError> {
        if GUARD_FROM_CALLBACK_DURING_DSP && self.audio_active {
            return Err(RecieveError::DspActive);
        }

        let _guard = self.set_as_active_instance();

        let callback = self.callbacks.add_callback(
            move |out: *const os::raw::c_char| {
                callback(str_from_ptr(out));
            },
            ClosureMut1::new,
        );

        let code = callback.code_ptr() as PrintHookCodePtr<'a>;
        let ptr = unsafe { *code.cast::<t_libpd_printhook>() };

        unsafe {
            libpd_sys::libpd_set_queued_printhook(Some(libpd_sys::libpd_print_concatenator));
        };

        // Always concatenate
        unsafe {
            libpd_sys::libpd_set_concatenated_printhook(ptr);
        }

        Ok(())
    }

    /// Instance-safe version of [`on_bang`](crate::functions::receive::on_bang) which doesn't leak memory.
    ///
    /// # Errors
    /// - [`RecieveError`]
    ///    - [`DspActive`](crate::error::RecieveError::DspActive)
    pub fn on_bang<'a, F: FnMut(&str) + 'a>(
        &'a mut self,
        mut callback: F,
    ) -> Result<(), RecieveError> {
        if GUARD_FROM_CALLBACK_DURING_DSP && self.audio_active {
            return Err(RecieveError::DspActive);
        }

        let _guard = self.set_as_active_instance();

        let callback = self.callbacks.add_callback(
            move |source: *const os::raw::c_char| {
                let source = unsafe { CStr::from_ptr(source).to_str().expect(C_STR_FAILURE) };
                callback(source);
            },
            ClosureMut1::new,
        );

        let code = callback.code_ptr() as BangHookCodePtr<'a>;
        let ptr = unsafe { *code.cast::<t_libpd_banghook>() };

        unsafe {
            libpd_sys::libpd_set_queued_banghook(ptr);
        }

        Ok(())
    }

    /// Instance-safe version of [`on_float`](crate::functions::receive::on_float) which doesn't leak memory.
    ///
    /// # Errors
    /// - [`RecieveError`]
    ///    - [`DspActive`](crate::error::RecieveError::DspActive)
    pub fn on_float<'a, F: FnMut(&str, f32) + 'a>(
        &'a mut self,
        mut callback: F,
    ) -> Result<(), RecieveError> {
        if GUARD_FROM_CALLBACK_DURING_DSP && self.audio_active {
            return Err(RecieveError::DspActive);
        }

        let _guard = self.set_as_active_instance();

        let callback = self.callbacks.add_callback(
            move |source: *const os::raw::c_char, float: f32| {
                let source = unsafe { CStr::from_ptr(source).to_str().expect(C_STR_FAILURE) };
                callback(source, float);
            },
            ClosureMut2::new,
        );

        let code = callback.code_ptr() as FloatHookCodePtr<'a>;
        let ptr = unsafe { *code.cast::<t_libpd_floathook>() };

        unsafe {
            libpd_sys::libpd_set_queued_floathook(ptr);
        }

        Ok(())
    }

    /// Instance-safe version of [`on_double`](crate::functions::receive::on_double) which doesn't leak memory.
    ///
    /// # Errors
    /// - [`RecieveError`]
    ///    - [`DspActive`](crate::error::RecieveError::DspActive)
    pub fn on_double<'a, F: FnMut(&str, f64) + 'a>(
        &'a mut self,
        mut callback: F,
    ) -> Result<(), RecieveError> {
        if GUARD_FROM_CALLBACK_DURING_DSP && self.audio_active {
            return Err(RecieveError::DspActive);
        }

        let _guard = self.set_as_active_instance();

        let callback = self.callbacks.add_callback(
            move |source: *const os::raw::c_char, double: f64| {
                callback(str_from_ptr(source), double);
            },
            ClosureMut2::new,
        );

        let code = callback.code_ptr() as DoubleHookCodePtr<'a>;
        let ptr = unsafe { *code.cast::<t_libpd_doublehook>() };

        unsafe {
            libpd_sys::libpd_set_queued_doublehook(ptr);
        }

        Ok(())
    }

    /// Instance-safe version of [`on_symbol`](crate::functions::receive::on_symbol) which doesn't leak memory.
    ///
    /// # Errors
    /// - [`RecieveError`]
    ///    - [`DspActive`](crate::error::RecieveError::DspActive)
    pub fn on_symbol<'a, F: FnMut(&str, &str) + 'a>(
        &'a mut self,
        mut callback: F,
    ) -> Result<(), RecieveError> {
        if GUARD_FROM_CALLBACK_DURING_DSP && self.audio_active {
            return Err(RecieveError::DspActive);
        }

        let _guard = self.set_as_active_instance();

        let callback = self.callbacks.add_callback(
            move |source: *const os::raw::c_char, symbol: *const os::raw::c_char| {
                callback(str_from_ptr(source), str_from_ptr(symbol));
            },
            ClosureMut2::new,
        );

        let code = callback.code_ptr() as SymbolHookCodePtr<'a>;
        let ptr = unsafe { *code.cast::<t_libpd_symbolhook>() };

        unsafe {
            libpd_sys::libpd_set_queued_symbolhook(ptr);
        }

        Ok(())
    }

    /// Instance-safe version of [`on_list`](crate::functions::receive::on_list) which doesn't leak memory.
    ///
    /// # Errors
    /// - [`RecieveError`]
    ///    - [`DspActive`](crate::error::RecieveError::DspActive)
    pub fn on_list<'a, F: FnMut(&str, &[Atom]) + 'a>(
        &'a mut self,
        mut callback: F,
    ) -> Result<(), RecieveError> {
        if GUARD_FROM_CALLBACK_DURING_DSP && self.audio_active {
            return Err(RecieveError::DspActive);
        }

        let _guard = self.set_as_active_instance();

        let callback = self.callbacks.add_callback(
            move |source: *const os::raw::c_char,
                  list_length: i32,
                  atom_list: *mut libpd_sys::t_atom| {
                callback(
                    str_from_ptr(source),
                    &atoms_from_raw(list_length, atom_list),
                );
            },
            ClosureMut3::new,
        );

        let code = callback.code_ptr() as ListHookCodePtr<'a>;
        let ptr = unsafe { *code.cast::<t_libpd_listhook>() };

        unsafe {
            libpd_sys::libpd_set_queued_listhook(ptr);
        }

        Ok(())
    }

    /// Instance-safe version of [`on_message`](crate::functions::receive::on_message) which doesn't leak memory.
    ///
    /// # Errors
    /// - [`RecieveError`]
    ///    - [`DspActive`](crate::error::RecieveError::DspActive)
    pub fn on_message<'a, F: FnMut(&str, &str, &[Atom]) + 'a>(
        &'a mut self,
        mut callback: F,
    ) -> Result<(), RecieveError> {
        if GUARD_FROM_CALLBACK_DURING_DSP && self.audio_active {
            return Err(RecieveError::DspActive);
        }

        let _guard = self.set_as_active_instance();

        let callback = self.callbacks.add_callback(
            move |source: *const os::raw::c_char,
                  message: *const os::raw::c_char,
                  list_length: i32,
                  atom_list: *mut libpd_sys::t_atom| {
                callback(
                    str_from_ptr(source),
                    str_from_ptr(message),
                    &atoms_from_raw(list_length, atom_list),
                );
            },
            ClosureMut4::new,
        );

        let code = callback.code_ptr() as MessageHookCodePtr<'a>;
        let ptr = unsafe { *code.cast::<t_libpd_messagehook>() };

        unsafe {
            libpd_sys::libpd_set_queued_messagehook(ptr);
        }

        Ok(())
    }

    /// Instance-safe version of [`on_midi_note_on`](crate::functions::receive::on_midi_note_on) which doesn't leak memory.
    pub fn on_midi_note_on<'a, F: FnMut(i32, i32, i32) + 'a>(&'a mut self, callback: F) {
        let _guard = self.set_as_active_instance();

        let callback = self.callbacks.add_callback(callback, ClosureMut3::new);

        let code = callback.code_ptr() as MidiNoteOnCodePtr<'a>;
        let ptr = unsafe { *code.cast::<t_libpd_noteonhook>() };

        unsafe {
            libpd_sys::libpd_set_queued_noteonhook(ptr);
        }
    }

    /// Instance-safe version of [`on_midi_control_change`](crate::functions::receive::on_midi_control_change) which doesn't leak memory.
    pub fn on_midi_control_change<'a, F: FnMut(i32, i32, i32) + 'a>(&'a mut self, callback: F) {
        let _guard = self.set_as_active_instance();

        let callback = self.callbacks.add_callback(callback, ClosureMut3::new);

        let code = callback.code_ptr() as MidiControlChangeCodePtr<'a>;
        let ptr = unsafe { *code.cast::<t_libpd_controlchangehook>() };

        unsafe {
            libpd_sys::libpd_set_queued_controlchangehook(ptr);
        }
    }

    /// Instance-safe version of [`on_midi_program_change`](crate::functions::receive::on_midi_program_change) which doesn't leak memory.
    pub fn on_midi_program_change<'a, F: FnMut(i32, i32) + 'a>(&'a mut self, callback: F) {
        let _guard = self.set_as_active_instance();

        let callback = self.callbacks.add_callback(callback, ClosureMut2::new);

        let code = callback.code_ptr() as MidiProgramChangeCodePtr<'a>;
        let ptr = unsafe { *code.cast::<t_libpd_programchangehook>() };

        unsafe {
            libpd_sys::libpd_set_queued_programchangehook(ptr);
        }
    }

    /// Instance-safe version of [`on_midi_pitch_bend`](crate::functions::receive::on_midi_pitch_bend) which doesn't leak memory.
    pub fn on_midi_pitch_bend<'a, F: FnMut(i32, i32) + 'a>(&'a mut self, callback: F) {
        let _guard = self.set_as_active_instance();

        let callback = self.callbacks.add_callback(callback, ClosureMut2::new);

        let code = callback.code_ptr() as MidiPitchBendCodePtr<'a>;
        let ptr = unsafe { *code.cast::<t_libpd_pitchbendhook>() };

        unsafe {
            libpd_sys::libpd_set_queued_pitchbendhook(ptr);
        }
    }

    /// Instance-safe version of [`on_midi_after_touch`](crate::functions::receive::on_midi_after_touch) which doesn't leak memory.
    pub fn on_midi_after_touch<'a, F: FnMut(i32, i32) + 'a>(&'a mut self, callback: F) {
        let _guard = self.set_as_active_instance();

        let callback = self.callbacks.add_callback(callback, ClosureMut2::new);

        let code = callback.code_ptr() as MidiAfterTouchCodePtr<'a>;
        let ptr = unsafe { *code.cast::<t_libpd_aftertouchhook>() };

        unsafe {
            libpd_sys::libpd_set_queued_aftertouchhook(ptr);
        }
    }

    /// Instance-safe version of [`on_midi_poly_after_touch`](crate::functions::receive::on_midi_poly_after_touch) which doesn't leak memory.
    pub fn on_midi_poly_after_touch<'a, F: FnMut(i32, i32, i32) + 'a>(&'a mut self, callback: F) {
        let _guard = self.set_as_active_instance();

        let callback = self.callbacks.add_callback(callback, ClosureMut3::new);

        //let callback = ClosureMut3::new(closure);
        let code = callback.code_ptr() as MidiPolyAfterTouchCodePtr<'a>;
        let ptr = unsafe { *code.cast::<t_libpd_polyaftertouchhook>() };

        unsafe {
            libpd_sys::libpd_set_queued_polyaftertouchhook(ptr);
        }
    }

    /// Instance-safe version of [`on_midi_poly_after_touch`](crate::functions::receive::on_midi_poly_after_touch) which doesn't leak memory.
    pub fn on_midi_byte<'a, F: FnMut(i32, i32) + 'a>(&'a mut self, callback: F) {
        let _guard = self.set_as_active_instance();

        let callback = self.callbacks.add_callback(callback, ClosureMut2::new);

        let code = callback.code_ptr() as MidiByteCodePtr<'a>;
        let ptr = unsafe { *code.cast::<t_libpd_midibytehook>() };

        unsafe {
            libpd_sys::libpd_set_queued_midibytehook(ptr);
        }
    }
}

/// This struct encapsulates a clone of the [`PdInstance`] to be used in the audio thread.
///
/// Since the instances are thread local, this is just a convenience struct to ensure that the instance is set as the current one before calling any functions.
///
/// If you don't set at least one instance as the current one, the functions in the library will panic.
#[derive(Debug, Clone)]
pub struct PdAudioContext {
    instance: PdInstance,
}

impl PdAudioContext {
    /// Sets the instance as the current one and calls [`receive_messages_from_pd`](crate::functions::receive::receive_messages_from_pd).
    pub fn receive_messages_from_pd(&self) {
        self.instance.set_as_current();
        functions::receive::receive_messages_from_pd();
    }

    /// Sets the instance as the current one and calls [`receive_midi_messages_from_pd`](crate::functions::receive::receive_midi_messages_from_pd).
    pub fn receive_midi_messages_from_pd(&self) {
        self.instance.set_as_current();
        functions::receive::receive_midi_messages_from_pd();
    }

    /// Sets the instance as the current one and calls [`process_float`](crate::functions::process::process_float).
    pub fn process_float(&self, ticks: i32, input: &[f32], output: &mut [f32]) {
        self.instance.set_as_current();
        functions::process::process_float(ticks, input, output);
    }

    /// Sets the instance as the current one and calls [`process_double`](crate::functions::process::process_double).
    pub fn process_double(&self, ticks: i32, input: &[f64], output: &mut [f64]) {
        self.instance.set_as_current();
        functions::process::process_double(ticks, input, output);
    }

    /// Sets the instance as the current one and calls [`process_short`](crate::functions::process::process_short).
    pub fn process_short(&self, ticks: i32, input: &[i16], output: &mut [i16]) {
        self.instance.set_as_current();
        functions::process::process_short(ticks, input, output);
    }

    /// Sets the instance as the current one and calls [`process_raw`](crate::functions::process::process_raw).
    pub fn process_raw(&self, input: &[f32], output: &mut [f32]) {
        self.instance.set_as_current();
        functions::process::process_raw(input, output);
    }

    /// Sets the instance as the current one and calls [`process_raw_short`](crate::functions::process::process_raw_short).
    pub fn process_raw_short(&self, input: &[i16], output: &mut [i16]) {
        self.instance.set_as_current();
        functions::process::process_raw_short(input, output);
    }

    /// Sets the instance as the current one and calls [`process_raw_double`](crate::functions::process::process_raw_double).
    pub fn process_raw_double(&self, input: &[f64], output: &mut [f64]) {
        self.instance.set_as_current();
        functions::process::process_raw_double(input, output);
    }
}

/// When an instance is set as the active instance for the thread, this guard is returned.
///
/// When the guard is dropped, the previously active instance will be set as the active instance.
struct ActiveInstanceGuard {
    previous_instance: *mut _pdinstance,
}

impl ActiveInstanceGuard {
    const fn wrap(previous_instance: *mut _pdinstance) -> Self {
        Self { previous_instance }
    }
}

impl Drop for ActiveInstanceGuard {
    fn drop(&mut self) {
        if self.previous_instance.is_null() {
            // Main instance is always valid.
            let main_instance = unsafe { libpd_sys::libpd_main_instance() };
            unsafe {
                libpd_sys::libpd_set_instance(main_instance);
            }
            return;
        }
        unsafe {
            libpd_sys::libpd_set_instance(self.previous_instance);
        }
    }
}

// Tracking for ensuring that resources created to handle the callbacks are cleaned up when `Pd` is dropped.
struct Callbacks {
    callbacks: Vec<CallbackDtor>,
}

impl Callbacks {
    const fn new() -> Self {
        Self {
            callbacks: Vec::new(),
        }
    }

    fn add_callback<'a, A: 'a, B, F: Fn(&'a mut A) -> B + 'a>(
        &mut self,
        callback: A,
        closure_mut_fn: F,
    ) -> &'a mut B {
        let boxed = Box::new(callback);
        let raw = Box::into_raw(boxed);
        let drop = CallbackDtor::drop_inner::<A>;

        self.callbacks.push(CallbackDtor {
            drop,
            data: raw.cast(),
        });

        let callback = unsafe { raw.as_mut().expect("callback is nullptr") };

        let closure = Box::new(closure_mut_fn(callback));
        let raw = Box::into_raw(closure);
        let drop = CallbackDtor::drop_inner::<B>;

        self.callbacks.push(CallbackDtor {
            drop,
            data: raw.cast(),
        });

        unsafe { raw.as_mut().expect("callback is nullptr") }
    }
}

// The smallest amount of information required to clean up an arbitrary Box
struct CallbackDtor {
    drop: unsafe fn(*mut ()),
    data: *mut (),
}

impl Drop for CallbackDtor {
    fn drop(&mut self) {
        unsafe { (self.drop)(self.data) }
    }
}

impl CallbackDtor {
    fn drop_inner<T>(p: *mut ()) {
        unsafe { drop(Box::<T>::from_raw(p.cast())) }
    }
}

fn atoms_from_raw(list_length: i32, atom_list: *mut libpd_sys::t_atom) -> Vec<Atom> {
    #[expect(
        clippy::cast_sign_loss,
        reason = "We're trusting Pd to not send a negative list length. I think this is sane enough."
    )]
    let atom_list = unsafe { slice::from_raw_parts(atom_list, list_length as usize) };
    make_atom_list_from_t_atom_list(atom_list)
}

fn str_from_ptr<'a>(s: *const i8) -> &'a str {
    unsafe { CStr::from_ptr(s).to_str().expect(C_STR_FAILURE) }
}

/// Type to assist with checking for the validity of sending a message.
struct SentMessageInfo {
    capacity: i32,
    length: i32,
}

impl SentMessageInfo {
    const fn increment(&mut self) -> Result<(), SendError> {
        self.length += 1;

        if self.length > self.capacity {
            Err(SendError::OutOfRange)
        } else {
            Ok(())
        }
    }

    const fn new(capacity: i32) -> Result<Self, SizeError> {
        if capacity < 0 {
            return Err(SizeError::TooLarge);
        }

        Ok(Self {
            capacity,
            length: 0,
        })
    }
}
