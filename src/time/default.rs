#![allow(clippy::assertions_on_constants)]

use std::{
    sync::Mutex,
    time::{Duration, Instant},
};

#[ctor::ctor]
pub(super) static DEFAULT_TIME_IMPL_STATICS: Mutex<(Instant, Duration, Option<Instant>)> =
    Mutex::new((Instant::now(), Duration::from_secs(0), None));

pub(super) fn default_uptime_source() -> u64 {
    if let Ok(time_statics) = DEFAULT_TIME_IMPL_STATICS.lock() {
        let abs_uptime = time_statics.0.elapsed().as_micros();
        let paused_uptime = time_statics
            .2
            .map_or(0, |start| start.elapsed().as_micros());
        let elapsed_paused_uptime = time_statics.1.as_micros();
        u64::try_from(abs_uptime - (paused_uptime + elapsed_paused_uptime)).unwrap_or(u64::MAX)
    } else {
        // use asserts instead of panics because of linting, i know i shouldn't be trying to
        // go around the linter but this will so rarely happen that i don't want to have to
        assert!(false, "DEFAULT_TIME_IMPL_STATICS mutex poisoned");
        0u64
    }
}

pub(super) fn default_uptime_pause(should_pause: bool) {
    if let Ok(mut time_statics) = DEFAULT_TIME_IMPL_STATICS.lock() {
        if should_pause {
            if time_statics.2.is_none() {
                time_statics.2 = Some(Instant::now());
            }
        } else if let Some(start) = time_statics.2 {
            time_statics.1 += start.elapsed();
            time_statics.2 = None;
        }
    } else {
        // use asserts instead of panics because of linting, i know i shouldn't be trying to
        // go around the linter but this will so rarely happen that i don't want to have to
        assert!(false, "DEFAULT_TIME_IMPL_STATICS mutex poisoned");
    }
}

pub(super) const DEFAULT_IMPL_NAME: &str = "Default";
