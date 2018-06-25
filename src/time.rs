use core::ops;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Debug, Ord, Hash, Default)]
pub struct Duration(usize);

impl Duration {
    pub fn zero() -> Duration {
        Duration(0)
    }

    pub fn from_millis(millis: usize) -> Duration {
        Duration(millis)
    }

    pub fn from_secs(secs: usize) -> Duration {
        Duration(secs.checked_mul(1_000)
            .expect("overflow when converting seconds to millis for duration"))
    }

    pub fn from_mins(mins: usize) -> Duration {
        Duration(mins.checked_mul(60 * 1_000)
            .expect("overflow when converting minutes to millis for duration"))
    }

    pub fn as_millis(&self) -> usize {
        self.0
    }

    pub fn as_secs(&self) -> usize {
        self.0 / 1_000
    }

    pub fn as_mins(&self) -> usize {
        self.0 / (60 * 1_000)
    }

    pub fn checked_add(self, rhs: Duration) -> Option<Duration> {
        self.0.checked_add(rhs.0).map(|r| Duration(r))
    }

    pub fn checked_sub(self, rhs: Duration) -> Option<Duration> {
        self.0.checked_sub(rhs.0).map(|r| Duration(r))
    }

    pub fn checked_mul(self, rhs: usize) -> Option<Duration> {
        self.0.checked_mul(rhs).map(|r| Duration(r))
    }

    pub fn checked_div(self, rhs: usize) -> Option<Duration> {
        self.0.checked_div(rhs).map(|r| Duration(r))
    }
}

impl ops::Add for Duration {
    type Output = Duration;

    fn add(self, rhs: Duration) -> Duration {
        self.checked_add(rhs).expect("overflow when adding durations")
    }
}

impl ops::AddAssign for Duration {
    fn add_assign(&mut self, rhs: Duration) {
        *self = *self + rhs;
    }
}

impl ops::Sub for Duration {
    type Output = Duration;

    fn sub(self, rhs: Duration) -> Duration {
        self.checked_sub(rhs).expect("overflow when subtracting durations")
    }
}

impl ops::SubAssign for Duration {
    fn sub_assign(&mut self, rhs: Duration) {
        *self = *self - rhs;
    }
}

impl ops::Mul<usize> for Duration {
    type Output = Duration;

    fn mul(self, rhs: usize) -> Duration {
        self.checked_mul(rhs).expect("overflow when multiplying duration by scalar")
    }
}

impl ops::Mul<Duration> for usize {
    type Output = Duration;

    fn mul(self, rhs: Duration) -> Duration {
        rhs * self
    }
}

impl ops::MulAssign<usize> for Duration {
    fn mul_assign(&mut self, rhs: usize) {
        *self = *self * rhs;
    }
}

impl ops::Div<usize> for Duration {
    type Output = Duration;

    fn div(self, rhs: usize) -> Duration {
        self.checked_div(rhs).expect("divide by zero error when dividing duration by scalar")
    }
}

impl ops::DivAssign<usize> for Duration {
    fn div_assign(&mut self, rhs: usize) {
        *self = *self / rhs;
    }
}
