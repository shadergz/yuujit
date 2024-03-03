use yuujit::{config::Config, guest_memory::GuestMemory, jit::Jit};

struct CodeMemory {
    code: Vec<u32>,
}

impl GuestMemory for CodeMemory {
    fn read_byte(&self, addr: u32) -> u8 {
        todo!()
    }

    fn read_half(&self, addr: u32) -> u16 {
        todo!()
    }

    fn read_word(&self, addr: u32) -> u32 {
        self.code[addr as usize / 4]
    }
}

#[test]
fn test_mov() {
    let mut code: Vec<u32> = Vec::new();
    code.push(0xe3a00001); // mov r0, #1
    code.push(0xe2801002); // add r1, r0, #2

    let code_memory = CodeMemory {
        code,
    };

    let mut jit = Jit::new(Config::default(), code_memory);
    jit.run(2);

    let state = jit.state();
    assert_eq!(state.gpr[0], 1);
    assert_eq!(state.gpr[1], 3);
    assert_eq!(state.gpr[15], 8);
}