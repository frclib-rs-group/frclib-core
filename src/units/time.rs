use std::{ops::Neg, time::Duration};

use crate::{unit, unit_conversion, unit_family};

unit!(Hour: float);
unit!(Minute: float);
unit!(Second: float);
unit!(Millisecond: float);
unit!(Microsecond: uint);

unit_conversion!(Second(float) <-> Millisecond(float) ~ second_to_millisecond);
unit_conversion!(Second(float) <-> Microsecond(uint) ~ second_to_microsecond);
unit_conversion!(Millisecond(float) <-> Microsecond(uint) ~ millisecond_to_microsecond);
unit_conversion!(Hour(float) <-> Second(float) ~ hour_to_second);
unit_conversion!(Minute(float) <-> Second(float) ~ minute_to_second);
unit_conversion!(Hour(float) <-> Minute(float) ~ hour_to_minute);
unit_conversion!(Minute(float) <-> Millisecond(float) ~ minute_to_millisecond);
unit_conversion!(Minute(float) <-> Microsecond(uint) ~ minute_to_microsecond);
unit_conversion!(Hour(float) <-> Millisecond(float) ~ hour_to_millisecond);
unit_conversion!(Hour(float) <-> Microsecond(uint) ~ hour_to_microsecond);

//TODO: This is a hack to satisfy unit family
impl Neg for Microsecond {
    type Output = Millisecond;
    #[allow(clippy::cast_precision_loss)]
    fn neg(self) -> Self::Output {
        Millisecond::new(-(self.value() as f64))
    }
}

unit_family!(Time(Second): Hour, Minute, Millisecond, Microsecond);

fn second_to_millisecond(second: f64) -> f64 {
    second * 1000.0
}

#[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
fn second_to_microsecond(second: f64) -> u64 {
    if second.is_sign_negative() {
        0
    } else {
        (second * 1_000_000.0) as u64
    }
}

#[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
fn millisecond_to_microsecond(millisecond: f64) -> u64 {
    if millisecond.is_sign_negative() {
        0
    } else {
        (millisecond * 1000.0) as u64
    }
}

fn hour_to_second(hour: f64) -> f64 {
    hour * 3600.0
}

fn minute_to_second(minute: f64) -> f64 {
    minute * 60.0
}

fn hour_to_minute(hour: f64) -> f64 {
    hour * 60.0
}

fn minute_to_millisecond(minute: f64) -> f64 {
    minute * 60000.0
}

#[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
fn minute_to_microsecond(minute: f64) -> u64 {
    if minute.is_sign_negative() {
        0
    } else {
        (minute * 60_000_000.0) as u64
    }
}

fn hour_to_millisecond(hour: f64) -> f64 {
    second_to_millisecond(hour_to_second(hour))
}

fn hour_to_microsecond(hour: f64) -> u64 {
    second_to_microsecond(hour_to_second(hour))
}

impl From<Duration> for Hour {
    fn from(duration: Duration) -> Self {
        Self::new(duration.as_secs_f64() / 3600.0)
    }
}

impl From<Duration> for Minute {
    fn from(duration: Duration) -> Self {
        Self::new(duration.as_secs_f64() / 60.0)
    }
}

impl From<Duration> for Second {
    fn from(duration: Duration) -> Self {
        Self::new(duration.as_secs_f64())
    }
}

impl From<Duration> for Millisecond {
    fn from(duration: Duration) -> Self {
        Self::new(duration.as_secs_f64() * 1000.0)
    }
}

impl From<Duration> for Microsecond {
    fn from(duration: Duration) -> Self {
        Self::new(u64::try_from(duration.as_micros()).unwrap_or(u64::MAX))
    }
}

impl From<Hour> for Duration {
    fn from(hour: Hour) -> Self {
        Self::from_secs_f64(hour.value() * 3600.0)
    }
}

impl From<Minute> for Duration {
    fn from(minute: Minute) -> Self {
        Self::from_secs_f64(minute.value() * 60.0)
    }
}

impl From<Second> for Duration {
    fn from(second: Second) -> Self {
        Self::from_secs_f64(second.value())
    }
}

impl From<Millisecond> for Duration {
    fn from(millisecond: Millisecond) -> Self {
        Self::from_secs_f64(millisecond.value() / 1000.0)
    }
}

impl From<Microsecond> for Duration {
    fn from(microsecond: Microsecond) -> Self {
        Self::from_micros(microsecond.value())
    }
}
