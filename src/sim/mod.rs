use std::collections::VecDeque;

mod port;
use port::*;

#[derive(Default)]
struct ArchState {
    pc: u64,
}

pub struct Sim {
    cycle: u64,
    counter: u64,
    state: ArchState,
    pub imem_req: OutputPort,
    pub imem_resp: InputPort,
}

impl Sim {
    pub fn new() -> Sim {
        Sim {
            cycle: 0,
            counter: 0,
            state: ArchState::default(),
            imem_req: OutputPort::new(),
            imem_resp: InputPort::new(),
        }
    }

    pub fn time(&self) -> u64 {
        self.cycle
    }

    fn fetch(&mut self) {
        if !self.imem_req.put(self.state.pc) {
            assert!(false, "imem_req port blocked!");
        }
        self.state.pc += 4; // FIXME: 32-bit hardcoded
    }

    fn decode(&mut self) {
        match self.imem_resp.get() {
            Some(data) => {
                println!("decode: got data={}", data);
            }
            None => {}
        }
    }

    pub fn tick(&mut self) {
        println!("tick! cycle={}", self.cycle);

        self.decode();
        self.fetch();

        self.cycle += 1;
    }
}
