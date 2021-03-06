// Twang
// Copyright © 2018-2021 Jeron Aldaron Lau.
//
// Licensed under any of:
// - Apache License, Version 2.0 (https://www.apache.org/licenses/LICENSE-2.0)
// - MIT License (https://mit-license.org/)
// - Boost Software License, Version 1.0 (https://www.boost.org/LICENSE_1_0.txt)
// At your choosing (See accompanying files LICENSE_APACHE_2_0.txt,
// LICENSE_MIT.txt and LICENSE_BOOST_1_0.txt).

use crate::sig::Signal;
use core::num::Wrapping;

const SEQUENCE: u64 = 0xb5ad4eceda1ce2a9;

/// White Noise Generator using Middle Square Weyl Sequence PRNG.
#[derive(Default, Clone, Debug)]
#[allow(missing_copy_implementations)]
pub struct White {
    x: Wrapping<u64>,
    w: Wrapping<u64>,
}

impl White {
    /// Create a new White Noise Sampler.
    #[inline(always)]
    pub fn new() -> Self {
        Self::default()
    }

    /// Get next sample of white noise.
    #[inline(always)]
    pub fn noise(&mut self) -> Signal {
        // msws (Middle Square Weyl Sequence) algorithm
        self.x *= self.x;
        self.w += Wrapping(SEQUENCE);
        self.x += self.w;
        self.x = (self.x >> 32) | (self.x << 32);
        Signal::from(
            ((self.x.0 as i32) as f64 + 0.5) * (i32::MAX as f64 + 0.5).recip(),
        )
    }
}
