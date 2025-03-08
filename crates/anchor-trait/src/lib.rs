pub trait Discriminator {
    const DISCRIMINATOR: [u8; 8];
    fn discriminator() -> [u8; 8] { Self::DISCRIMINATOR }
}

pub trait Space {
    const INIT_SPACE: usize;
}
