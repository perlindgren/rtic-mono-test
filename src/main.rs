use rtic_monotonic::*;
struct Timer {}

struct MInit {
    timer: Timer,
}

struct Mono {
    timer: Timer,
}

impl Mono {
    fn new(m: MInit) -> Self {
        Mono { timer: m.timer } // move the actual timer
    }
}

struct Instant {}
struct Duration {}

impl MonoInit for MInit {
    type Instant = Instant;
    type Duration = Duration;
    type Mono = Mono;

    fn now(&mut self) -> Self::Instant {
        todo!()
    }

    /// Turn the Init Type to a monotonic;
    fn into_mono(self) -> Self::Mono {
        Mono::new(self)
    }
}

impl Monotonic for Mono {
    type Instant = Instant;
    type Duration = Duration;

    const DISABLE_INTERRUPT_ON_EMPTY_QUEUE: bool = true;

    /// Get the current time instant
    fn now(&mut self) -> Self::Instant {
        println!("its time");
        Instant {}
    }

    /// Set the compare value of the timer interrupt.
    fn set_compare(&mut self, instant: &Self::Instant) {
        todo!()
    }

    /// Clear the compare interrupt flag.
    fn clear_compare_flag(&mut self) {
        todo!()
    }
}

fn main() {
    let mi = MInit { timer: Timer {} };
    let mut m = mi.into_mono(); // consumes mi

    let i = m.now();
}
