use riscv::register::sstatus::{self, Sstatus, SPP};

/// Trap Context
#[repr(C)]
pub struct TrapContext {
    /// general registers[0..31]
    pub x: [usize; 32],

    /// CSR sstatus
    pub sstatus: Sstatus,

    /// CSR sepc
    pub sepc: usize,
}

impl TrapContext {
    /// set stack pointer to x[2] register(sp)
    pub fn set_sp(&mut self, sp: usize) {
        self.x[2] = sp;
    }

    /// init the app context
    pub fn app_init_context(entry: usize, user_sp: usize) -> Self {
        let mut sstatus = sstatus::read();
        // set previous privilege mode: user mode, used to jump from supervisor mode
        sstatus.set_spp(SPP::User);

        let mut cx = Self {
            x: [0; 32],
            sstatus,
            sepc: entry, // entry point of app
        };
        cx.set_sp(user_sp); // app's user stack pointer

        cx
    }
}
