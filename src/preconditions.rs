use num::Integer;

#[derive(Debug, Clone)]
pub struct Checked<T, Check> {
    inner: T,
    check: Check,
}

impl<T, Check> Checked<T, Check>
where
    Check: Checkable<T>,
{
    pub fn new(inner: T) -> Self {
        Self::try_new(inner).unwrap()
    }

    pub fn try_new(inner: T) -> Option<Self> {
        let check = Check::default();
        if check.check(&inner) {
            Some(Checked { inner, check })
        } else {
            None
        }
    }

    pub fn inner(self) -> T {
        self.inner
    }
}

pub trait Checkable<T>: Default {
    fn check(&self, t: &T) -> bool;
}

#[derive(Debug, Default, Clone, Copy)]
pub struct OddCheck;

impl<T: Integer> Checkable<T> for OddCheck {
    fn check(&self, t: &T) -> bool {
        t.is_odd()
    }
}
