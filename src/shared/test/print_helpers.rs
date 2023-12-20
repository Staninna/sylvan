pub struct OkPrint;
pub struct FailPrint;
pub struct NoTestsPrint;

impl core::fmt::Display for OkPrint {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        let str = "[ok]";
        write!(f, "\x1b[32m{}\x1b[0m", str)
    }
}

impl core::fmt::Display for FailPrint {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        let str = "[fail]";
        write!(f, "\x1b[31m{}\x1b[0m", str)
    }
}

impl core::fmt::Display for NoTestsPrint {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        let str = "No tests to run here :(";
        write!(f, "\x1b[38;5;214m{}\x1b[0m", str)
    }
}
