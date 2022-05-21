pub(super) fn add<const L: usize>(x: &[u64; L], y: &[u64; L], z: &mut [u64; L]) -> bool {
    let mut carry = false;
    for i in 0..L {
        (z[i], carry) = x[i].carrying_add(y[i], carry);
    }
    carry
}

pub(super) fn add_self<const L: usize>(x: &mut [u64; L], y: &[u64; L]) -> bool {
    let mut carry = false;
    for i in 0..L {
        (x[i], carry) = x[i].carrying_add(y[i], carry);
    }
    carry
}

pub(super) fn sub<const L: usize>(x: &[u64; L], y: &[u64; L], z: &mut [u64; L]) -> bool {
    let mut borrow = false;
    for i in 0..L {
        (z[i], borrow) = x[i].borrowing_sub(y[i], borrow);
    }
    borrow
}

pub(super) fn sub_self<const L: usize>(x: &mut [u64; L], y: &[u64; L]) -> bool {
    let mut borrow = false;
    for i in 0..L {
        (x[i], borrow) = x[i].borrowing_sub(y[i], borrow);
    }
    borrow
}


pub(super) fn mul<const L: usize>(x: &[u64; L], y: &[u64; L], z: &mut [u64; L + 1]) -> bool {
    false
}
