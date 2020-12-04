const OP_SIZE: usize = 4;

pub struct Intcode {
    pc: usize,
    running: bool,
    ram: Vec<u32>,
}

impl Intcode {
    pub fn new() -> Self {
        Self {
            pc: 0,
            running: true,
            ram: Vec::new(),
        }
    }

    pub fn reset(&mut self, data: Vec<u32>) {
        self.pc = 0;
        self.running = true;
        self.ram = data;
    }

    pub fn run(&mut self) -> u32 {
        while self.running {
            let op = self.ram[self.pc];
            match op {
                1 => { self.add_1() },
                2 => { self.mul_2() },
                99 => { self.halt_99() },
                _ => { panic!("Unsupported opcode"); }
            }
            self.pc += OP_SIZE;
        }

        self.ram[0]
    }

    pub fn write_ram(&mut self, idx: usize, val: u32) {
        self.ram[idx] = val;
    }

    fn add_1(&mut self) {
        let addr1 = self.ram[self.pc + 1] as usize;
        let addr2 = self.ram[self.pc + 2] as usize;
        let sum_addr = self.ram[self.pc + 3] as usize;
        self.ram[sum_addr] = self.ram[addr1] + self.ram[addr2];
    }

    fn mul_2(&mut self) {
        let addr1 = self.ram[self.pc + 1] as usize;
        let addr2 = self.ram[self.pc + 2] as usize;
        let mul_addr = self.ram[self.pc + 3] as usize;
        self.ram[mul_addr] = self.ram[addr1] * self.ram[addr2];

    }

    fn halt_99(&mut self) {
        self.running = false;
    }
}
