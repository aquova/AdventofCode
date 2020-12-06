use std::io;

pub struct Intcode {
    pc: usize,
    running: bool,
    ram: Vec<i32>,
}

impl Intcode {
    pub fn new() -> Self {
        Self {
            pc: 0,
            running: true,
            ram: Vec::new(),
        }
    }

    pub fn reset(&mut self, data: Vec<i32>) {
        self.pc = 0;
        self.running = true;
        self.ram = data;
    }

    pub fn run(&mut self) -> i32 {
        while self.running {
            let op = self.ram[self.pc];
            let params = op / 100;
            match op % 100 {
                1 => self.add_1(params),
                2 => self.mul_2(params),
                3 => self.in_3(),
                4 => self.out_4(),
                5 => self.jmpt_5(params),
                6 => self.jmpf_6(params),
                7 => self.lt_7(params),
                8 => self.eq_8(params),
                99 => self.halt_99(),
                _ => panic!("Unsupported opcode: {}", (op % 100)),
            }
        }

        self.ram[0]
    }

    pub fn write_ram(&mut self, idx: usize, val: i32) {
        self.ram[idx] = val;
    }

    fn read_ram(&self, addr: usize, mode: i32) -> i32 {
        let val = self.ram[addr];
        match mode {
            0 => self.ram[val as usize],
            1 => val,
            _ => unimplemented!()
        }
    }

    fn add_1(&mut self, params: i32) {
        let (param1, param2, _) = parse_params(params);
        let add1 = self.read_ram(self.pc + 1, param1);
        let add2 = self.read_ram(self.pc + 2, param2);
        let sum_addr = self.ram[self.pc + 3] as usize;
        self.ram[sum_addr] = add1 + add2;
        self.pc += 4;
    }

    fn mul_2(&mut self, params: i32) {
        let (param1, param2, _) = parse_params(params);
        let mul1 = self.read_ram(self.pc + 1, param1);
        let mul2 = self.read_ram(self.pc + 2, param2);
        let mul_addr = self.ram[self.pc + 3] as usize;
        self.ram[mul_addr] = mul1 * mul2;
        self.pc += 4;
    }

    fn in_3(&mut self) {
        loop {
            let mut input = String::new();
            println!("Provide a valid input: ");
            io::stdin().read_line(&mut input).unwrap();

            let input: i32 = match input.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };

            let addr = self.ram[self.pc + 1];
            self.ram[addr as usize] = input;
            self.pc += 2;
            break;
        }
    }

    fn out_4(&mut self) {
        let addr = self.ram[self.pc + 1];
        println!("{}", self.ram[addr as usize]);
        self.pc += 2;
    }

    fn jmpt_5(&mut self, params: i32) {
        let (param1, param2, _) = parse_params(params);
        let val = self.read_ram(self.pc + 1, param1);
        if val != 0 {
            let addr = self.read_ram(self.pc + 2, param2);
            self.pc = addr as usize;
        } else {
            self.pc += 3;
        }
    }

    fn jmpf_6(&mut self, params: i32) {
        let (param1, param2, _) = parse_params(params);
        let val = self.read_ram(self.pc + 1, param1);
        if val == 0 {
            let addr = self.read_ram(self.pc + 2, param2);
            self.pc = addr as usize;
        } else {
            self.pc += 3;
        }
    }

    fn lt_7(&mut self, params: i32) {
        let (param1, param2, _) = parse_params(params);
        let val1 = self.read_ram(self.pc + 1, param1);
        let val2 = self.read_ram(self.pc + 2, param2);
        let val3 = if val1 < val2 { 1 } else { 0 };
        let addr = self.ram[self.pc + 3];
        self.ram[addr as usize] = val3;
        self.pc += 4;
    }

    fn eq_8(&mut self, params: i32) {
        let (param1, param2, _) = parse_params(params);
        let val1 = self.read_ram(self.pc + 1, param1);
        let val2 = self.read_ram(self.pc + 2, param2);
        let val3 = if val1 == val2 { 1 } else { 0 };
        let addr = self.ram[self.pc + 3];
        self.ram[addr as usize] = val3;
        self.pc += 4;
    }

    fn halt_99(&mut self) {
        self.running = false;
    }
}

fn parse_params(params: i32) -> (i32, i32, i32) {
    let c = params % 10;
    let b = (params / 10) % 10;
    let a = params / 100;
    (c, b, a)
}
