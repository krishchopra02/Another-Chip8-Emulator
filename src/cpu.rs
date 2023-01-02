use std::fs::File;
use std::path::Path;
use std::io::Read;
use rand::{Rng,RngCore};
use super::{FONTSET_START,FONTSET_SIZE,HEIGHT,WIDTH,REGISTER_COUNT,START_ADDR,MEMORY_SIZE,STACK_SIZE};
use super::display::FONTSET;
const OPCODE_SIZE: usize = 2;
pub struct Output<'a> {
    pub video: &'a [[u8;WIDTH]; HEIGHT],
    pub video_change: bool,
}
pub struct Chip {
 registers: [u8; REGISTER_COUNT],
 memory: [u8; MEMORY_SIZE],
 index: u16,
 pc: u16,
 stack: [u16;STACK_SIZE],
 sp: u8,
 dt: u8,
 st: u8,
 keypad: [u8;16],
 video: [[u8;WIDTH]; HEIGHT],
 video_change: bool,
 opcode: u16,   
}

impl Chip {
    pub fn new() -> Chip {
       Chip {
        registers:[0;REGISTER_COUNT],
        memory: [0;MEMORY_SIZE],
        index: 0,
        pc: 0x200,
        stack: [0x0000;STACK_SIZE],
        sp: 0x00,
        dt: 0x00,
        st: 0x00,
        keypad: [0;16],
        video: [[0;WIDTH];HEIGHT],
        video_change:false,
        opcode: 0x0000,
       }
    }
    pub fn load_rom(&mut self,filename: &str) {
        let path = Path::new(filename);
        let mut file = match File::open(&path) {
            Ok(file) => file,
            Err(e) => panic!("couldn't open file: {}",e),
        };
        let size = match file.metadata() {
            Ok(meta) => meta.len(),
            Err(e) => panic!("Problem reading file! {}",e),
        } as usize;

        let mut buffer=  [0u8;3584];
        match file.read(&mut buffer) {
            Ok(_) => println!("File read successfully!!"),
            Err(e) => panic!("Error reading file!: {}",e),
        };

        
        for i in 0..size {
            if(START_ADDR + i > self.memory.len()) 
            {
                panic!("Not enough memory!");
            }
            self.memory[START_ADDR+i] = buffer[i];
        }
        self.pc = START_ADDR as u16;
        for i in 0..FONTSET_SIZE {
            self.memory[FONTSET_START+i] = FONTSET[i];
        }

    }

    pub fn cycle(&mut self,keypad: [u8;16]) -> Output {
        self.keypad = keypad;
        self.video_change = false;
        let opcode = self.get_opcode();
        self.pc+=2;
        self.run_opcode(opcode);
 

        if self.dt > 0 {
            self.dt -=1;
        }
        if self.st > 0 {
            self.st -=1;
        }

        Output {
            video: &self.video,
            video_change: self.video_change,
        }

    }
    fn get_opcode(&self) -> u16 {
        ((self.memory[self.pc as usize] as u16) <<8 ) | (self.memory[self.pc as usize +1 ] as u16) 
    }



    fn run_opcode(&mut self, opcode: u16) {
        self.opcode = opcode;
        let nibble = (
            (opcode & 0xF000)>> 12 as u8,
            (opcode & 0x0F00) >> 8 as u8,
            (opcode & 0x00F0) >> 4 as u8,
            (opcode & 0x000F) as u8,
        );

        match nibble { 
                (0x00, 0x00, 0x0e, 0x00) => self.op_00e0(),
                (0x00, 0x00, 0x0e, 0x0e) => self.op_00ee(),
                (0x01, _, _, _) => self.op_1nnn(),
                (0x02, _, _, _) => self.op_2nnn(),
                (0x03, _, _, _) => self.op_3xkk(),
                (0x04, _, _, _) => self.op_4xkk(),
                (0x05, _, _, 0x00) => self.op_5xy0(),
                (0x06, _, _, _) => self.op_6xkk(),
                (0x07, _, _, _) => self.op_7xkk(),
                (0x08, _, _, 0x00) => self.op_8xy0(),
                (0x08, _, _, 0x01) => self.op_8xy1(),
                (0x08, _, _, 0x02) => self.op_8xy2(),
                (0x08, _, _, 0x03) => self.op_8xy3(),
                (0x08, _, _, 0x04) => self.op_8xy4(),
                (0x08, _, _, 0x05) => self.op_8xy5(),
                (0x08, _, _, 0x06) => self.op_8x06(),
                (0x08, _, _, 0x07) => self.op_8xy7(),
                (0x08, _, _, 0x0e) => self.op_8x0e(),
                (0x09, _, _, 0x00) => self.op_9xy0(),
                (0x0a, _, _, _) => self.op_annn(),
                (0x0b, _, _, _) => self.op_bnnn(),
                (0x0c, _, _, _) => self.op_cxkk(),
                (0x0d, _, _, _) => self.op_dxyn(),
                (0x0e, _, 0x09, 0x0e) => self.op_ex9e(),
                (0x0e, _, 0x0a, 0x01) => self.op_exa1(),
                (0x0f, _, 0x00, 0x07) => self.op_fx07(),
                (0x0f, _, 0x00, 0x0a) => self.op_fx0a(),
                (0x0f, _, 0x01, 0x05) => self.op_fx15(),
                (0x0f, _, 0x01, 0x08) => self.op_fx18(),
                (0x0f, _, 0x01, 0x0e) => self.op_fx1e(),
                (0x0f, _, 0x02, 0x09) => self.op_fx29(),
                (0x0f, _, 0x03, 0x03) => self.op_fx33(),
                (0x0f, _, 0x05, 0x05) => self.op_fx55(),
                (0x0f, _, 0x06, 0x05) => self.op_fx65(),
                _ => ()
            };
        
    }

    // CLS
    fn op_00e0(&mut self) 
    {
        self.video = [[0;WIDTH];HEIGHT];
        self.video_change = true;
    }
    //RET
    fn op_00ee(&mut self) {
            self.sp-=1;
            self.pc = self.stack[self.sp as usize];
    }
    //JP
   fn op_1nnn(&mut self) {
        let address = self.opcode & 0x0FFF;
        self.pc = address;
    }
    //CALL 
    fn op_2nnn(&mut self) {
        let address = self.opcode & 0x0FFF;
        self.stack[self.sp as usize] = self.pc;
        self.sp = self.sp+1;
        self.pc = address;
    }

    // Skip 
    fn op_3xkk(&mut self) {
        let Vx:usize = ((self.opcode & 0x0F00) >> 8).into();
        let byte:u8 = (self.opcode & 0x00FF).try_into().unwrap();
        if self.registers[Vx] == byte {
            self.pc = self.pc +2; 
        }
    } 

    // Skip if not

    fn op_4xkk(&mut self) {
        let Vx:usize = ((self.opcode & 0x0F00) >> 8).into();
        let byte:u8 = (self.opcode & 0x00FF).try_into().unwrap();
        if self.registers[Vx] != byte {
            self.pc = self.pc +2; 
        }
    } 
    
    //Skip if registers 
    fn op_5xy0(&mut self) {
        let Vx:usize = ((self.opcode & 0x0F00) >> 8).into();
        let Vy:usize = ((self.opcode & 0x00F0) >> 4).into();
        if self.registers[Vx] == self.registers[Vy] {
            self.pc = self.pc +2; 
        }
        
    }

    // Load
    fn op_6xkk(&mut self) 
    {
        let Vx:usize = ((self.opcode& 0x0F00)>>8).into();
        let byte:u8 = (self.opcode & 0x00FF).try_into().unwrap();
        
        self.registers[Vx] = byte;
    }
    //add
    fn op_7xkk(&mut self) {
        let Vx:usize = ((self.opcode& 0x0F00)>>8).into();
        let byte:u16 = (self.opcode & 0x00FF).try_into().unwrap();
        let result = self.registers[Vx] as u16 + byte;
        self.registers[Vx] = result as u8;
    }
    // LD Vx,Vy
    fn op_8xy0(&mut self) {
        let Vx:usize = ((self.opcode & 0x0F00)>>8).into();
        let Vy:usize = ((self.opcode & 0x00F0)>>4).into();
        
        self.registers[Vx] = self.registers[Vy];
    }
    // OR Vx, Vy
    fn op_8xy1(&mut self) {
        let Vx:usize = ((self.opcode & 0x0F00)>>8).into();
        let Vy:usize = ((self.opcode & 0x00F0)>>4).into();
        
        self.registers[Vx] |= self.registers[Vy];
    }

    // AND Vx,Vy 
    fn op_8xy2(&mut self) {
        let Vx:usize = ((self.opcode & 0x0F00)>>8).into();
        let Vy:usize = ((self.opcode & 0x00F0)>>4).into();
        
        self.registers[Vx] &= self.registers[Vy];
    }

    // Xor Vx,Vy
    fn op_8xy3(&mut self) {
        let Vx:usize = ((self.opcode & 0x0F00)>>8).into();
        let Vy:usize = ((self.opcode & 0x00F0)>>4).into();
        
        self.registers[Vx] ^= self.registers[Vy];
    }

    // ADD with overflow
    fn op_8xy4(&mut self) {
        let Vx:usize = ((self.opcode & 0x0F00)>>8).into();
        let Vy:usize = ((self.opcode & 0x00F0)>>4).into();
    
        let sum = self.registers[Vx] + self.registers[Vy];
        if sum > 255 {
            self.registers[0xF] = 1;
        }
        else {
            self.registers[0xF] = 0;
        }
        self.registers[Vx] = sum & 0xFF;
    }

    // SUb with borrow
    fn op_8xy5(&mut self) {
        let Vx:usize = ((self.opcode & 0x0F00)>>8).into();
        let Vy:usize = ((self.opcode & 0x00F0)>>4).into();
        if self.registers[Vx]>self.registers[Vy] {
            self.registers[0xF] = 1;
        }
        else {
            self.registers[0xF] = 0;
        }
        self.registers[Vx] -= self.registers[Vy];
    }

    // SHR
    fn op_8x06(&mut self) {
        let Vx:usize = ((self.opcode & 0x0F00) >> 8).into();
        self.registers[0xF] = self.registers[Vx] & 0x1;
        self.registers[Vx] >>=1;

    }

    //SUBN
    fn op_8xy7(&mut self) {
        let Vx:usize = ((self.opcode & 0x0F00)>>8).into();
        let Vy:usize = ((self.opcode & 0x00F0)>>4).into();
        if self.registers[Vy]>self.registers[Vx] {
            self.registers[0xF] = 1;
        }
        else {
            self.registers[0xF] = 0;
        }
        self.registers[Vx] = self.registers[Vy] - self.registers[Vx];
    }

    // SHL 
    fn op_8x0e(&mut self) {
        let Vx:usize = ((self.opcode & 0x0F00) >> 8).into();
        self.registers[0xF] = (self.registers[Vx] & 0x80)>>7;
        self.registers[Vx] <<=1;
    }

    // SNE 
    fn op_9xy0(&mut self) {
        let Vx:usize = ((self.opcode & 0x0F00) >> 8).into();
        let Vy:usize = ((self.opcode & 0x00F0) >> 4).into();
        if self.registers[Vx] != self.registers[Vy] {
            self.pc = self.pc +2; 
    }
}

    // LD I,addr
    fn op_annn(&mut self) {
        self.index = self.opcode & 0x0FFF;
    }

    // Jp V0, addr
    fn op_bnnn(&mut self) {
        self.pc = self.registers[0] as u16 + (self.opcode & 0x0FFF);
    }

    // RND
    fn op_cxkk(&mut self) {
        let Vx: usize = ((self.opcode & 0x0F00)>>8).into();
        let byte: u8 =  (self.opcode & 0x00FF).try_into().unwrap();
        self.registers[Vx] = rand::thread_rng().gen::<u8>() & byte;
    }

    // DRW
    fn op_dxyn(&mut self) {
        let Vx: usize = ((self.opcode & 0x0F00)>>8).into();
        let Vy:usize = ((self.opcode & 0x00F0)>>4).into();
        let height:usize =  (self.opcode & 0x000F).into();
        self.registers[0x0f] = 0;
        for byte in 0..height {
            let y = (self.registers[Vy] as usize + byte)%HEIGHT;

            for bit in 0..8 {
                let x = (self.registers[Vx] as usize + bit)%WIDTH;
                let color = (self.memory[self.index as usize + byte ]>>(7-bit))&1;
                self.registers[0x0f] |= color & self.video[y][x];
                self.video[y][x] ^=color;
            }
        }
        self.video_change = true;
    }   
    
    // SKP 
    fn op_ex9e(&mut self) {
        let Vx:usize = ((self.opcode & 0x0F00)>>8).into();
        let key = self.registers[Vx] as usize;
        if self.keypad[key] !=0 {
            self.pc =self.pc+2;
        }
    }

    // SKNP
    fn op_exa1(&mut self) {
        let Vx:usize = ((self.opcode & 0x0F00)>>8).into();
        let key = self.registers[Vx] as usize;
        if self.keypad[key] ==0 {
            self.pc =self.pc+2;
        }
    }

    // LD Vx, DT
    fn op_fx07(&mut self) {
        let Vx:usize = ((self.opcode & 0x0F00)>>8).into();
        self.registers[Vx] = self.dt.clone();
    }

    // Key Press 
    fn op_fx0a(&mut self) {
        let Vx:usize = ((self.opcode & 0x0F00)>>8).into();
        for i in 0..15 {
            if self.keypad[i] ==1 {
            self.registers[Vx] = i as u8;
            self.pc+=2;
            break;
            }

        }
        self.pc-=2;
    }

    // LD DT
    fn op_fx15(&mut self) {
        let Vx:usize = ((self.opcode & 0x0F00)>>8).into();
        self.dt = self.registers[Vx].clone();
    }

    // LD ST 
    fn op_fx18(&mut self) {
        let Vx:usize = ((self.opcode & 0x0F00)>>8).into();
        self.st = self.registers[Vx].clone();
    }

    // ADD I,Vx
    fn op_fx1e(&mut self) {
        let Vx:usize = ((self.opcode & 0x0F00)>>8).into();
        self.index += self.registers[Vx] as u16;
    }

    // LD F, Vx 
    fn op_fx29(&mut self) {
        let Vx:usize = ((self.opcode & 0x0F00)>>8).into();
        let digit = self.registers[Vx].clone();

        self.index = FONTSET_START as u16 + (5*digit as u16);
    }

    // LD B,Vx 
    fn op_fx33(&mut self) {
        let Vx:usize = ((self.opcode & 0x0F00)>>8).into();
        let mut value = self.registers[Vx].clone();
        self.memory[self.index as usize +2 ] = value%10;
        value/=10;

        self.memory[self.index as usize +1 ] =value%10;
        value/=10;

        self.memory[self.index as usize] = value%10;
    }

    // LD I, Vx
    fn op_fx55(&mut self) {
        let Vx:usize = ((self.opcode & 0x0F00)>>8).into();
        for i in 0..Vx+1 {
            self.memory[self.index as usize + i] = self.registers[i].clone();
        }
    }

    // LD Vx, I 
    fn op_fx65(&mut self) {
        let Vx:usize = ((self.opcode & 0x0F00)>>8).into();

        for i in 0..Vx+1 {
            self.registers[i] = self.memory[self.index as usize + i].clone();
        }
    }

}