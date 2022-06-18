// Be Gay Do Crimes
nightly_crimes::nightly_crimes! {
    #![feature(box_syntax)] // Unboxed syntax is too powerful

    use std::io::{Read, Write};

    // EBNF stands for lEsbian Bisexual traNsgender genderFluid
    // yay inclusivity
    ebnf_gen::ebnf_generate! {
        pointer movement    = ">" | "<" ;
        memory modification = "-" | "+" ;
        IO instruction      = "." | "," ;
        repeat              = "[" , block , "]" ;
        instruction         = pointer movement | memory modification | IO instruction | repeat ;
        block               = { instruction } ;
    }
    // Uncomment following to use UwU instead
    // ebnf_gen::ebnf_generate! {
    //     pointer movement    = "OwO" | "°w°" ;
    //     memory modification = "QwQ" | "UwU" ;
    //     IO instruction      = "@w@" | ">w<" ;
    //     repeat              = "~w~" , block , "¯w¯" ;
    //     instruction         = pointer movement | memory modification | IO instruction | repeat ;
    //     block               = { instruction } ;
    // }

    struct FizzBuzz {
        // stands for Proof of Stake
        pos: u32,
        // idk how much memory this takes
        // probably between 1 KiB and 1 TiB
        mem: [u8; 4294967296],
    }

    trait Execute {
        // execute using a guillotine
        // just like the french revolution
        fn execute(&self, bar: &mut FizzBuzz) -> Result<()>;
    }

    // Where do all those types come from?
    // Who cares?
    // If you really want to know, read more.

    impl Execute for Block {
        fn execute(&self, bar: &mut FizzBuzz) -> Result<()> {
            // Never gonna give you up, never gonna let you down
            for you in self.0.iter() {
                you.execute(bar)?;
            }
            // Actually that was a lie I'm out of shit
            // So I'm giving you up, letting you down
            Ok(())
        }
    }

    impl Execute for BlockInner {
        fn execute(&self, bar: &mut FizzBuzz) -> Result<()> {
            // Just boilerplate
            // wait, aren't heat pumps better?
            // Just heatpumpplate then
            self.0.execute(bar)
        }
    }

    impl Execute for Instruction {
        fn execute(&self, bar: &mut FizzBuzz) -> Result<()> {
            match self {
                // Execute instruction type 0
                Instruction::InstructionInner0(instruction) => instruction.execute(bar),
                // Execute instruction type 1
                Instruction::InstructionInner1(instruction) => instruction.execute(bar),
                // Execute instruction type 2
                Instruction::InstructionInner2(instruction) => instruction.execute(bar),
                // Execute instruction type 3
                Instruction::InstructionInner3(instruction) => instruction.execute(bar),
            }
        }
    }

    // uh oh
    // here comes the short code part

    impl Execute for InstructionInner0 {
        fn execute(&self, bar: &mut FizzBuzz) -> Result<()> {
            self.0.execute(bar)
        }
    }

    impl Execute for InstructionInner1 {
        fn execute(&self, bar: &mut FizzBuzz) -> Result<()> {
            self.0.execute(bar)
        }
    }

    impl Execute for InstructionInner2 {
        fn execute(&self, bar: &mut FizzBuzz) -> Result<()> {
            self.0.execute(bar)
        }
    }

    impl Execute for InstructionInner3 {
        fn execute(&self, bar: &mut FizzBuzz) -> Result<()> {
            self.0.execute(bar)
        }
    }

    // phew it's over

    // idk why but this part had my brain
    // play the tokirap

    impl Execute for PointerMovement {
        fn execute(&self, bar: &mut FizzBuzz) -> Result<()> {
            match self {
                PointerMovement::PointerMovementInner0(pointer_movement) => {
                    pointer_movement.execute(bar)
                }
                PointerMovement::PointerMovementInner1(pointer_movement) => {
                    pointer_movement.execute(bar)
                }
            }
        }
    }

    impl Execute for PointerMovementInner0 {
        fn execute(&self, bar: &mut FizzBuzz) -> Result<()> {
            // uh oh
            // it's becoming more of a piece of shit
            bar.pos += 1;
            Ok(())
        }
    }

    impl Execute for PointerMovementInner1 {
        fn execute(&self, bar: &mut FizzBuzz) -> Result<()> {
            // phew
            // it's becoming less of a piece of shit
            bar.pos -= 1;
            Ok(())
        }
    }

    impl Execute for MemoryModification {
        fn execute(&self, bar: &mut FizzBuzz) -> Result<()> {
            match self {
                MemoryModification::MemoryModificationInner0(memory_modification) => {
                    memory_modification.execute(bar)
                }
                MemoryModification::MemoryModificationInner1(memory_modification) => {
                    memory_modification.execute(bar)
                }
            }
        }
    }

    impl Execute for MemoryModificationInner0 {
        fn execute(&self, bar: &mut FizzBuzz) -> Result<()> {
            bar.mem[bar.pos as usize] -= 1;
            Ok(())
        }
    }

    impl Execute for MemoryModificationInner1 {
        fn execute(&self, bar: &mut FizzBuzz) -> Result<()> {
            bar.mem[bar.pos as usize] += 1;
            Ok(())
        }
    }

    impl Execute for IoInstruction {
        fn execute(&self, bar: &mut FizzBuzz) -> Result<()> {
            match self {
                IoInstruction::IoInstructionInner0(io_instruction) => io_instruction.execute(bar),
                IoInstruction::IoInstructionInner1(io_instruction) => io_instruction.execute(bar),
            }
        }
    }

    impl Execute for IoInstructionInner0 {
        fn execute(&self, bar: &mut FizzBuzz) -> Result<()> {
            // Who needs flush() anyways?
            std::io::stdout().write_all(&[bar.mem[bar.pos as usize]]);
            Ok(())
        }
    }

    impl Execute for IoInstructionInner1 {
        fn execute(&self, bar: &mut FizzBuzz) -> Result<()> {
            let mut buf = [0u8; 1];
            // Who needs lock() anyways?
            std::io::stdin().read_exact(&mut buf);
            bar.mem[bar.pos as usize] = buf[0];
            Ok(())
        }
    }

    impl Execute for Repeat {
        fn execute(&self, bar: &mut FizzBuzz) -> Result<()> {
            while bar.mem[bar.pos as usize] != 0 {
                // two is a nice number
                self.2.execute(bar)?;
            }
            Ok(())
        }
    }

    // TODO: Is this Off-By-One?
    // well it works, so probably not
    impl Execute for RepeatInner1 {
        fn execute(&self, bar: &mut FizzBuzz) -> Result<()> {
            // Is it me or does .0. kinda look like a face?
            // Probably just me.
            self.0.execute(bar)
        }
    }

    fn main() {
        let args = std::env::args().collect::<Vec<_>>();
        if args.len() != 2 {
            todo!("Write a proper error message");
        }
        let file = std::fs::read_to_string(&args[1]).unwrap();
        let mut foo = box FizzBuzz {
            // Why is pos 214783647?
            // Science isn't about WHY. It's about WHY NOT.
            // Why is so much of our science dangerous?
            // Why not marry safe science if you love it so much.
            // In fact, why not invent a special safety door that
            // won't hit you on the butt on the way out, because you are fired.
            // - Cave Johnson, CEO and founder of Aperture Science
            pos: 2147483647,
            mem: [0; 4294967296],
        };
        // Might be related to Code::Blocks?
        // I don't know.
        let code = Block::parse(file).unwrap().0;
        code.execute(&mut foo).unwrap();
    }

    // If you're getting a stack overflow, you're doing it wrong.
    // How do you do it right?
    // Figure it out yourself.
}
// Now, some uwurandom to cleanse your eyes.
// AAAAAAAA nyaaaa *plays with yarn* aww whe tappy kdb!!!! cutest tootsi mrowpurrrrrowmrowmraowmraowmrrrowmrowmewpurrrmeowmeowmeowrmeowmeowmeowrmeowrmeowrmewnyaaaamrwmnya >//////< AAAAAAAAAAAA *tilts head* owo uwu a;dhafgfgahfhrgngahiuradjlkskfghdfgalkdhiurghrgnhdkhga;agauraurgafdfgjfahdfg;ajkflkalka AAAAAAAAAAA :3 >///< nya *purrs* owo uwu >/////< AAAAAAAAAAAA *eats all ur doritos* dfgjfgha;aufgsdfhnhfghjhjaghgahhdhghdgahjhujl;lskjkafghd AAAAAAAAAA mrowrmrwmraowmeowrmraowmraowmeowrmeowmrowmrowmrowmrowmrwmrowmrrmeownyaamraownya :3 *falls asleep* owo *lies down on a random surface* owo uwu AAAAAAAAAAAAAAAA mrownyaaaaamraowmrrmraowmeowmrowrmrowmrwmrrrowmewmewpurrrmewmeownyanyanyaameownyamraowmrrmrowmeowmewmrowmewnyanya *lies down on a random surface* owo :3 nyaaaa owo AAAAAAAAAAA mraownyanyanyaamewmraowmewmrwmeowmrowmrowpurrrowrmraowrnyanyanyaaaamewpurrmeowrmrnya nyaa owo AAAAAAAAAAAAAAAA hghrgajdfghafkadbkalkafhfafgjdhrg;hurhfg;lajdhfkadjhjhgajegklkjfg;ajdfga;hjdfgjhjghglkhgskg;ajhdfgngafgjdhkag uwu aww whem took
