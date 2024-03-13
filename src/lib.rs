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

use std::ffi::CString;

struct Processor;

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
    pub struct {{project-name|upper_camel_case}};

    impl {{project-name|upper_camel_case}} {
        #[bang]
        pub fn bang(&mut self) {
            let m = CString::new(format!("hello").to_string()).expect("CString::new failed");
            pd::post(m);
        }

        #[sel(defaults=1)]
        pub fn foo(&mut self, arg1: pd_sys::t_float, arg2: pd_ext::symbol::Symbol) {
            let m =
                CString::new(format!("got foo {} {}", arg1, arg2).to_string()).expect("CString::new failed");
            pd::post(m);
        }

        #[sel(defaults=1)]
        pub fn bar(&mut self, arg1: pd_sys::t_float) {
            let m =
                CString::new(format!("got bar {}", arg1).to_string()).expect("CString::new failed");
            pd::post(m);
        }

        #[sel(defaults=1)]
        pub fn baz(&mut self, arg1: pd_ext::symbol::Symbol) {
            let m =
                CString::new(format!("got baz {}", arg1)).expect("CString::new failed");
            pd::post(m);
        }
    }

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
