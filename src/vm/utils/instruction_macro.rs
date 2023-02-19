macro_rules! register_instructions {
    {
        $(
            $code_value:expr => $struct_name:ty
        ),*
    } => {
        pub fn decode(code: &[u8]) -> Box<dyn Instruction> {
            let instruction_code = code[0];
            match instruction_code {
                $($code_value => Box::new(<$struct_name>::new(code))),*,
                _ => panic!("Invalid instruction"),
            }
        }
    }
}

pub(crate) use register_instructions;
