use borsh::BorshSerialize;

pub trait Discriminator {
    const DISCRIMINATOR: [u8; 8];
    fn discriminator() -> [u8; 8] { Self::DISCRIMINATOR }
}

pub trait Space {
    const INIT_SPACE: usize;
}

/// Calculates the data for an instruction invocation, where the data is
/// `Sha256(<namespace>:<method_name>)[..8] || BorshSerialize(args)`.
/// `args` is a borsh serialized struct of named fields for each argument given
/// to an instruction.
pub trait InstructionData: Discriminator + BorshSerialize {
    fn data(&self) -> Vec<u8> {
        let mut data = Vec::with_capacity(256);
        data.extend_from_slice(&Self::discriminator());
        self.serialize(&mut data).unwrap();
        data
    }

    /// Clears `data` and writes instruction data to it.
    ///
    /// We use a `Vec<u8>`` here because of the additional flexibility of
    /// re-allocation (only if necessary), and because the data field in
    /// `Instruction` expects a `Vec<u8>`.
    fn write_to(&self, mut data: &mut Vec<u8>) {
        data.clear();
        data.extend_from_slice(&Self::DISCRIMINATOR);
        self.serialize(&mut data).unwrap()
    }
}
