use core::fmt;
use core::arch::global_asm;

extern "C" {
    fn hyp_console_log(x: u8) -> ();
}


global_asm!(
    include_str!("uarthyp.s"),
    //FFA_CONSOLE_LOG_64 = const 0xC400008A,
);

pub struct HypUart {
}

impl HypUart {
    pub fn new() -> Self {
         HypUart {}
    }

    fn write_byte(&self, data: u8) {
        unsafe { hyp_console_log(data) };
    }
}

impl fmt::Write for HypUart {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.chars() {
            self.write_byte(c as u8);
        }

        Ok(())
    }
}
