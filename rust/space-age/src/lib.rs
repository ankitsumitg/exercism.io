// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.
const EARTH_SECS: f64 = 31_557_600 as f64;
#[derive(Debug)]
pub struct Duration(u64);
impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Self(s)
    }
}

pub trait Planet {
    const MULTIPLIER: f64;
    fn years_during(d: &Duration) -> f64 {
        d.0 as f64/EARTH_SECS/Self::MULTIPLIER
    }
}

macro_rules! make_planet {
    ($name:ident,$multiplier:expr) => {
        pub struct $name;
        impl Planet for $name{
            const MULTIPLIER: f64 = $multiplier;
        }
    };
}
make_planet!(Mercury,0.2408467);
make_planet!(Venus,0.61519726);
make_planet!(Earth,1.0);
make_planet!(Mars,1.8808158);
make_planet!(Jupiter,11.862615);
make_planet!(Saturn,29.447498);
make_planet!(Uranus,84.016846);
make_planet!(Neptune,164.79132);