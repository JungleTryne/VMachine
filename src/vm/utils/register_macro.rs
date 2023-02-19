/// # Machine registers
/// The following macro generates an enum with registers,
/// as well as [as_addr] and [from_addr] functions.
/// When creating a register, one specifies the address of it
///
/// For example:
/// ```
/// make_registers! {
///     REG0 => 0,
///     REG1 => 4,
///     REG2 => 8,
/// }
/// ```
///
/// On the right side of the arrow one can specify an expression
/// for the address
///
macro_rules! make_registers {
    {
        $(
            $register: ident => $address: expr
        ),+
    } => {
        #[derive(Copy, Clone, Debug)]
        pub enum Register {
            $($register,)+
        }

        impl Register {
            pub fn as_addr(&self) -> u32 {
                match self {
                    $(Register::$register => $address),+
                }
            }

            pub fn from_addr(addr: u32) -> Self {
                match addr {
                    $(addr if addr == $address => Register::$register),+,
                    _ => panic!("Invalid register address"),
                }
            }
        }
    }
}

pub(crate) use make_registers;
