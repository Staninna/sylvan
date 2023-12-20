pub struct Ok;
pub struct Fail;

impl core::fmt::Display for Ok {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "\x1b[32m[ok]\x1b[0m")
    }
}

impl core::fmt::Display for Fail {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "\x1b[31m[fail]\x1b[0m")
    }
}
