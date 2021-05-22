pub trait Norm {
    type Length;
    fn length(self: &Self) -> Self::Length;
}