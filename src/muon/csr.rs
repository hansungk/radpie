use std::collections::HashMap;
use std::sync::{RwLock, RwLockWriteGuard};
use crate::base::behavior::{Parameterizable, Resets, Stalls, Ticks};
use crate::base::component::*;
use crate::base::state::HasState;
use crate::muon::warp::Warp;

pub enum CSROp {
    CLEAR, // AND value
    SET,   // OR value
    WRITE  // replace value
}

#[derive(Default)]
pub struct CSRState {
    csr: HashMap<u32, u32>,
}

// this is instantiated per lane
#[derive(Default)]
pub struct CSRFile {
    base: ComponentBase<CSRState, (), Warp>,
    lock: RwLock<()>,
}

impl Ticks for CSRFile {
    fn tick_one(&mut self) {
        // TODO: count cycles and stuff
    }
}

impl Resets for CSRFile {
    fn reset(&mut self) {
        self.base.state.csr.clear();
    }
}

impl Stalls for CSRFile {}
impl HasState for CSRFile {}

impl IsComponent<CSRState, (), Warp> for CSRFile {
    base_boilerplate!(CSRState, (), Warp);
}

macro_rules! get_ref_rw_match {
    ($self:expr, $variable:expr, [$( $addr:expr, $init:expr );* $(;)?]) => {
        #[allow(unreachable_patterns)]
        match $variable { $(
            $addr => Some($self.base.state.csr.entry($addr).or_insert($init)),
            _ => None,
        )* }
    };
}

macro_rules! get_ro_match {
    ($self:expr, $variable:expr, [$( $addr:expr, $init:expr );* $(;)?]) => {
        #[allow(unreachable_patterns)]
        match $variable { $(
            $addr => Some($init),
            _ => None,
        )* }
    };
}

impl CSRFile {
    pub fn new() -> Self {
        let mut csr = CSRFile::default();
        csr.lock = RwLock::new(());
        csr
    }

    // these are constant values
    fn csr_ro_ref(&self, addr: u32) -> Option<u32> {
        get_ro_match!(self, addr, [
            0xf11, 0; // mvendorid
            0xf12, 0; // marchid
            0xf13, 0; // mimpid
            0x301, (1 << 30)
                | (0 <<  0) /* A - Atomic Instructions extension */
                | (0 <<  1) /* B - Tentatively reserved for Bit operations extension */
                | (0 <<  2) /* C - Compressed extension */
                | (0 <<  3) /* D - Double precsision floating-point extension */
                | (0 <<  4) /* E - RV32E base ISA */
                | (1 <<  5) /* F - Single precsision floating-point extension */
                | (0 <<  6) /* G - Additional standard extensions present */
                | (0 <<  7) /* H - Hypervisor mode implemented */
                | (1 <<  8) /* I - RV32I/64I/128I base ISA */
                | (0 <<  9) /* J - Reserved */
                | (0 << 10) /* K - Reserved */
                | (0 << 11) /* L - Tentatively reserved for Bit operations extension */
                | (1 << 12) /* M - Integer Multiply/Divide extension */
                | (0 << 13) /* N - User level interrupts supported */
                | (0 << 14) /* O - Reserved */
                | (0 << 15) /* P - Tentatively reserved for Packed-SIMD extension */
                | (0 << 16) /* Q - Quad-precision floating-point extension */
                | (0 << 17) /* R - Reserved */
                | (0 << 18) /* S - Supervisor mode implemented */
                | (0 << 19) /* T - Tentatively reserved for Transactional Memory extension */
                | (1 << 20) /* U - User mode implemented */
                | (0 << 21) /* V - Tentatively reserved for Vector extension */
                | (0 << 22) /* W - Reserved */
                | (1 << 23) /* X - Non-standard extensions present */
                | (0 << 24) /* Y - Reserved */
                | (0 << 25) /* Z - Reserved */
            ; // misa
            0x180, 0; // satp
            0x300, 0; // mstatus
            0x302, 0; // medeleg
            0x303, 0; // mideleg
            0x304, 0; // mie
            0x305, 0; // mtvec
            0x341, 0; // mepc
            0x3a0, 0; // pmpcf0
            0x3b0, 0; // pmpaddr0
            0xb01, 0; // mpm_reserved
            0xb81, 0; // mpm_reserved_h
        ])
    }

    // these can only be read by the user,
    // but the emulator can update them
    fn csr_rw_ref_emu(&mut self, addr: u32) -> Option<&mut u32> {
        get_ref_rw_match!(self, addr, [
            0xf14, 0; // mhartid
            0xcc0, 0; // thread_id
            0xcc1, 0; // warp_id
            0xcc2, 0; // core_id
            0xcc3, 0; // warp_mask
            0xcc4, 0; // thread_mask
            0xfc0, 0; // num_threads
            0xfc1, 0; // num_warps
            0xfc2, 0; // num_cores
            0xb00, 0; // mcycle
            0xb80, 0; // mcycle_h
            0xb02, 0; // minstret
            0xb82, 0; // minstret_h
        ])
    }

    // these are writable by user with an initial value
    fn csr_rw_ref_user(&mut self, addr: u32) -> Option<&mut u32> {
        get_ref_rw_match!(self, addr, [
            0xacc, 0; // cisc accelerator
            0x001, 0; // vx_fflags
            0x002, 0; // vx_frm
            0x003, 0; // vx_fcsr

        ])
    }

    pub fn user_access(&mut self, addr: u32, value: u32, op: CSROp) -> u32 {
        if let Some(&mut mut w) = self.csr_rw_ref_user(addr) { // writable
            let _ = self.lock.write().expect("lock poisoned");
            let old_value = w.clone();
            match op {
                CSROp::CLEAR => { w &= value }
                CSROp::SET => { w = value }
                CSROp::WRITE => { w |= value }
            }
            old_value
        } else {
            let _ = self.lock.read().expect("lock poisoned");
            self.csr_rw_ref_emu(addr).map(|x| *x).or(self.csr_ro_ref(addr))
                .expect(&format!("reading nonexistent csr {}", addr))
        }
    }

    pub fn emu_access(&mut self, addr :u32, value: u32) {
        let _ = self.lock.write().expect("lock poisoned");
        *self.csr_rw_ref_emu(addr).expect(&format!("setting nonexistent csr {}", addr)) = value
    }
}