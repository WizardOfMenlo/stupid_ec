mod generic;
mod x86;

/// Add x and y, storing the result in z and returning the carry
pub(super) fn add<const L: usize>(x: &[u64; L], y: &[u64; L], z: &mut [u64; L]) -> bool {
    generic::add(x, y, z)
}

/// Add x and y, storing the result in x
pub(super) fn add_self<const L: usize>(x: &mut [u64; L], y: &[u64; L]) -> bool {
    generic::add_self(x, y)
}

/// Subtract x - y, storing the result in z and returning the borrow
pub(super) fn sub<const L: usize>(x: &[u64; L], y: &[u64; L], z: &mut [u64; L]) -> bool {
    generic::sub(x, y, z)
}

/// Subtract x - y, storing the result in x and returning the borrow
pub(super) fn sub_self<const L: usize>(x: &mut [u64; L], y: &[u64; L]) -> bool {
    generic::sub_self(x, y)
}
