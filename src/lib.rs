//! This is the (example) documentation for a pure data external.
//!
//! It is based on the [puredata-rust-template](https://github.com/madskjeldgaard/puredata-rust-template/) by
//! [Mads Kjeldgaard](https://madskjeldgaard.dk/) and leverages
//! [puredata-rust](https://github.com/x37v/puredata-rust). An external
//! implements a `Processor` to handle different kinds of data retrieved
//! from the inputs and moved to the outputs.
use pd_ext::builder::SignalProcessorExternalBuilder;
use pd_ext::external::{SignalProcessor, SignalProcessorExternal};
use pd_ext::pd;

use pd_ext_macros::external;

/// The type of strings to be displayed in the Pure Data log console.
use std::ffi::CString;

/// The main data-structure to hold all data that is required beyond
/// each single frame being processed.
struct Processor;

/// Implement the required `SignalProcessor` trait to handle data send
/// to and put out by the external.
impl SignalProcessor for Processor {
    fn process(
        &mut self,
        _frames: usize,
        inputs: &[&mut [pd_sys::t_float]],
        outputs: &mut [&mut [pd_sys::t_float]],
    ) {
        for (output, input) in outputs.iter_mut().zip(inputs.iter()) {
            output.copy_from_slice(input);
        }
    }
}

external! {
    /// Generate a (unique) name for the exernal based on the project title.
    pub struct {{project-name|upper_camel_case}};

    impl {{project-name|upper_camel_case}} {
        /// Handle a bang being send to the Pure Data external.
        #[bang]
        pub fn bang(&mut self) {
            let m = CString::new(format!("hello").to_string()).expect("CString::new failed");
            pd::post(m);
        }

        /// Handle a float and a symbol being send to the Pure Data external.
        #[sel(defaults=1)]
        pub fn foo(&mut self, arg1: pd_sys::t_float, arg2: pd_ext::symbol::Symbol) {
            let m =
                CString::new(format!("got foo {} {}", arg1, arg2).to_string()).expect("CString::new failed");
            pd::post(m);
        }

        /// Handle a float being send to the Pure Data external.
        #[sel(defaults=1)]
        pub fn bar(&mut self, arg1: pd_sys::t_float) {
            let m =
                CString::new(format!("got bar {}", arg1).to_string()).expect("CString::new failed");
            pd::post(m);
        }

        /// Handle a symbol being send to the Pure Data external.
        #[sel(defaults=1)]
        pub fn baz(&mut self, arg1: pd_ext::symbol::Symbol) {
            let m =
                CString::new(format!("got baz {}", arg1)).expect("CString::new failed");
            pd::post(m);
        }
    }

    /// Create a (new) external with two inlets and three outlets.
    impl SignalProcessorExternal for {{project-name|upper_camel_case}} {
        fn new(builder: &mut dyn SignalProcessorExternalBuilder<Self>) -> Result<(Self, Box<dyn SignalProcessor>), String> {
            builder.new_signal_outlet();
            builder.new_signal_outlet();
            builder.new_signal_outlet();
            builder.new_signal_inlet();
            builder.new_signal_inlet();
            Ok((Self { }, Box::new(Processor)))
        }
    }
}
