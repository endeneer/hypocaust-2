use riscv::register::{sstatus::{self, SPP, Sstatus}, hstatus::{Hstatus, self}};

#[repr(C)]
#[derive(Debug)]
/// trap context structure containing sstatus, sepc and registers
pub struct TrapContext {
    /// general regs[0..31]
    pub x: [usize; 32],
    /// CSR sstatus      
    pub sstatus: Sstatus,
    /// CSR sepc
    pub sepc: usize,
    /// Addr of Page Table
    pub hgatp: usize,
    /// kernel stack
    pub kernel_sp: usize,
    /// Addr of trap_handler function
    pub trap_handler: usize,
    /// CSR hstatus
    pub hstatus: Hstatus
}

impl TrapContext {
    /// set stack pointer to x_2 reg (sp)
    pub fn set_sp(&mut self, sp: usize) {
        self.x[2] = sp;
    }

    /// init guest trap context
    pub fn initialize_context(
        entry: usize,
        sp: usize,
        hgatp: usize,
        kernel_sp: usize,
        trap_handler: usize,
    ) -> Self {
        let mut sstatus = sstatus::read();
        // 这里需要注意，进入 VS 态的时候需要将 sstatus 的 SPP 设置为 Supervisor
        sstatus.set_spp(SPP::Supervisor); 
        let mut hstatus = hstatus::read();
        hstatus.set_spv(true);
        let mut cx = Self {
            x: [0; 32],
            sstatus,
            sepc: entry,  // entry point of app
            hgatp,  // addr of page table
            kernel_sp,    // kernel stack
            trap_handler, // addr of trap_handler function
            hstatus
        };
        cx.set_sp(sp); // app's user stack pointer
        cx // return initial Trap Context of app
    }
}
