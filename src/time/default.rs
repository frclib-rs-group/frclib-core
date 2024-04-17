use std::{
    sync::RwLock,
    time::{Duration, Instant},
};

#[ctor::ctor]
pub(super) static DEFAULT_TIME_IMPL_STATICS: RwLock<(Instant, Duration, Option<Instant>)> =
    RwLock::new((Instant::now(), Duration::from_secs(0), None));

pub(super) fn default_uptime_source() -> u64 {
    if let Ok(time_statics) = DEFAULT_TIME_IMPL_STATICS.read() {
        let abs_uptime = time_statics.0.elapsed().as_micros();
        let paused_uptime = time_statics
            .2
            .map_or(0, |start| start.elapsed().as_micros());
        let elapsed_paused_uptime = time_statics.1.as_micros();
        u64::try_from(abs_uptime - (paused_uptime + elapsed_paused_uptime)).unwrap_or(u64::MAX)
    } else {
        unreachable!("DEFAULT_TIME_IMPL_STATICS lock poisoned")
    }
}

pub(super) fn default_uptime_pause(should_pause: bool) {
    if let Ok(mut time_statics) = DEFAULT_TIME_IMPL_STATICS.write() {
        if should_pause {
            if time_statics.2.is_none() {
                time_statics.2 = Some(Instant::now());
            }
        } else if let Some(start) = time_statics.2 {
            time_statics.1 += start.elapsed();
            time_statics.2 = None;
        }
    } else {
        unreachable!("DEFAULT_TIME_IMPL_STATICS lock poisoned")
    }
}

pub(super) const DEFAULT_IMPL_NAME: &str = "Default";
