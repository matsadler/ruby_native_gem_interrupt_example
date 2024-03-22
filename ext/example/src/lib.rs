use std::{thread::sleep, time::Duration};

use magnus::{function, prelude::*, Error, Ruby, Value};

// once called this can not be interrupted
// run example_rust_loop_no_int to see it in action
fn loop_no_int() {
    loop {
        sleep(Duration::from_secs(1))
    }
}

// interruptable version of loop_no_int. may take up a second for interrupt
// run example_rust_loop_int to see it in action
fn loop_int(ruby: &Ruby) -> Result<(), Error> {
    loop {
        // must return the error, the default interrupt handler raises an
        // exception
        ruby.thread_check_ints()?;
        sleep(Duration::from_secs(1))
    }
}

// once called Ruby can not be inuterruped for `secs`
// run example_ruby_loop_delayed_int to see it in action
fn no_int_for(secs: u64) {
    sleep(Duration::from_secs(secs));
}

// swallows all exceptions, including the one raised from interrupt handler
// equivalent to `rescue Exception`
// run example_no_int_block to see it in action
fn swallow_exception(ruby: &Ruby) {
    let _ = ruby.yield_values::<[Value; 0], Value>([]);
}

// swallows only exceptions it's safe to rescue
// equivalent to `rescue StandardError` or bare `rescue`.
// run example_int_block to see it in action
fn swallow_std_err(ruby: &Ruby) -> Result<(), Error> {
    match ruby.yield_values::<[Value; 0], Value>([]) {
        Ok(_) => Ok(()),
        Err(e) if e.is_kind_of(ruby.exception_standard_error()) => Ok(()),
        Err(e) => Err(e),
    }
}

#[magnus::init]
fn init(ruby: &Ruby) -> Result<(), Error> {
    let module = ruby.define_module("Example")?;
    module.define_singleton_method("loop_no_int", function!(loop_no_int, 0))?;
    module.define_singleton_method("loop_int", function!(loop_int, 0))?;
    module.define_singleton_method("no_int_for", function!(no_int_for, 1))?;
    module.define_singleton_method("swallow_exception", function!(swallow_exception, 0))?;
    module.define_singleton_method("swallow_std_err", function!(swallow_std_err, 0))?;
    Ok(())
}
