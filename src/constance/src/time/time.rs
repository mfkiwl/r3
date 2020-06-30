use core::{convert::TryInto, fmt};

use crate::{
    time::Duration,
    utils::{Init, ZeroInit},
};

/// Represents a timestamp used by the API surface of the Constance
/// RTOS.
///
/// The origin is application-defined. If an application desires to represent a
/// calender time using `Time`, it's recommended to use the midnight UTC on
/// January 1, 1970 (a.k.a. “UNIX timestamp”) as the origin.
///
/// `Time` is backed by `u64` and can represent up to 213,503,982 days with
/// microsecond precision.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct Time {
    micros: u64,
}

impl Init for Time {
    const INIT: Self = Self::ZERO;
}

// Safety: `Time` is `repr(transparent)` and the only inner field is `u64`,
//         which is `ZeroInit`
unsafe impl ZeroInit for Time {}

impl Default for Time {
    fn default() -> Self {
        Self::INIT
    }
}

impl Time {
    /// Zero (the origin).
    pub const ZERO: Self = Time { micros: 0 };

    /// The large representable timestamp.
    pub const MAX: Self = Time { micros: u64::MAX };

    /// Construct a new `Time` from the specified number of microseconds.
    #[inline]
    pub const fn from_micros(micros: u64) -> Self {
        Self { micros }
    }

    /// Construct a new `Time` from the specified number of milliseconds.
    ///
    /// Pancis if `millis` overflows the representable range of `Time`.
    #[inline]
    pub const fn from_millis(millis: u64) -> Self {
        // FIXME: `Option::expect` is not `const fn` yet
        Self::from_micros(if let Some(x) = millis.checked_mul(1_000) {
            x
        } else {
            panic!("duration overflow");
        })
    }

    /// Construct a new `Time` from the specified number of seconds.
    ///
    /// Pancis if `secs` overflows the representable range of `Time`.
    #[inline]
    pub const fn from_secs(secs: u64) -> Self {
        // FIXME: `Option::expect` is not `const fn` yet
        Self::from_micros(if let Some(x) = secs.checked_mul(1_000_000) {
            x
        } else {
            panic!("duration overflow");
        })
    }

    /// Get the total number of whole microseconds contained in the time span
    /// between this `Time` and [`Self::ZERO`].
    #[inline]
    pub const fn as_micros(self) -> u64 {
        self.micros
    }

    /// Get the total number of whole milliseconds contained in the time span
    /// between this `Time` and [`Self::ZERO`].
    #[inline]
    pub const fn as_millis(self) -> u64 {
        self.micros / 1_000
    }

    /// Get the total number of whole seconds contained in the time span
    /// between this `Time` and [`Self::ZERO`].
    #[inline]
    pub const fn as_secs(self) -> u64 {
        self.micros / 1_000_000
    }

    /// Get the total number of seconds contained in the time span between this
    /// `Time` and [`Self::ZERO`] as `f64`.
    ///
    /// # Examples
    ///
    /// ```
    /// use constance::time::Time;
    ///
    /// let dur = Time::from_micros(1_201_250_000);
    /// assert_eq!(dur.as_secs_f64(), 1201.25);
    /// ```
    #[inline]
    pub const fn as_secs_f64(self) -> f64 {
        self.micros as f64 / 1_000_000.0
    }

    /// Get the total number of seconds contained in the time span between this
    /// `Time` and [`Self::ZERO`] as `f32`.
    ///
    /// # Examples
    ///
    /// ```
    /// use constance::time::Time;
    ///
    /// let dur = Time::from_micros(1_201_250_000);
    /// assert_eq!(dur.as_secs_f32(), 1201.25);
    /// ```
    #[inline]
    pub const fn as_secs_f32(self) -> f32 {
        // An integer larger than 16777216 can't be converted to `f32`
        // accurately. Split `self` into an integer part and fractional part and
        // convert them separately so that integral values are preserved
        // during the conversion.
        (self.micros / 1_000_000) as f32 + (self.micros % 1_000_000) as f32 / 1_000_000.0
    }

    /// Get the duration since the origin as [`::core::time::Duration`].
    #[inline]
    pub const fn core_duration_since_origin(self) -> core::time::Duration {
        core::time::Duration::from_micros(self.micros)
    }

    /// Get the duration since the specified timestamp as
    /// [`::core::time::Duration`]. Returns `None` if `self` < `reference`.
    #[inline]
    pub const fn core_duration_since(self, reference: Self) -> Option<core::time::Duration> {
        if self.micros >= reference.micros {
            Some(core::time::Duration::from_micros(self.micros))
        } else {
            None
        }
    }

    /// Get the duration since the specified timestamp as [`Duration`]. Returns
    /// `None` if the result overflows the representable range of `Duration`.
    #[inline]
    pub fn duration_since(self, reference: Self) -> Option<Duration> {
        // FIXME: `?` is not allowed in `const fn` yet
        // FIXME: `try_into` is not supported in `const fn` yet
        Some(Duration::from_micros(
            (self.micros as i128 - reference.micros as i128)
                .try_into()
                .ok()?,
        ))
    }
}

impl fmt::Debug for Time {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.core_duration_since_origin().fmt(f)
    }
}

// TODO: Add more tests
// TODO: Interoperation with `::chrono`
