use num_traits::NumOps;
/**
 * Structs to make it easier to define positions/directions (no more evil messing around with tuples)
 * They are implemented as if positive X is right, negative X is left, positive Y is down, negative Y is up.
 */
use std::ops::{Add, AddAssign, Div, Mul, Neg, Sub, SubAssign};

/**
 * An integer vector
 */
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Vec2<I>
where
    I: NumOps + Copy,
{
    pub x: I,
    pub y: I,
}

impl<I> Vec2<I>
where
    I: NumOps + Copy,
{
    pub fn new(x: I, y: I) -> Self {
        Self { x, y }
    }

    pub fn dot(self, rhs: Self) -> I {
        self.x * rhs.x + self.y * rhs.y
    }

    pub fn components_try_into<T: NumOps + Copy + TryFrom<I>>(
        self,
    ) -> Result<Vec2<T>, <T as TryFrom<I>>::Error> {
        match (self.x.try_into(), self.y.try_into()) {
            (Ok(x), Ok(y)) => Ok(Vec2 { x, y }),
            (Err(e), _) => Err(e),
            (_, Err(e)) => Err(e),
        }
    }
}

impl<I, N> Vec2<I>
where
    I: NumOps + Copy + Neg<Output = N> + Into<N>,
    N: NumOps + Copy,
{
    /**
     * Return a new vector rotated left 90 degrees.
     */
    pub fn rotate_left(&self) -> Vec2<N> {
        Vec2 {
            x: self.y.into(),
            y: -self.x,
        }
    }

    /**
     * Return a new vector rotated right 90 degrees.
     */
    pub fn rotate_right(&self) -> Vec2<N> {
        Vec2 {
            x: -self.y,
            y: self.x.into(),
        }
    }
}

impl<I> Vec2<I>
where
    I: NumOps + Copy + num_traits::identities::Zero + num_traits::identities::One + Neg<Output = I>,
{
    pub fn up() -> Self {
        Self::new(I::zero(), -I::one())
    }

    pub fn right() -> Self {
        Self::new(I::one(), I::zero())
    }

    pub fn down() -> Self {
        Self::new(I::zero(), I::one())
    }

    pub fn left() -> Self {
        Self::new(-I::one(), I::zero())
    }
}

impl<I> Vec2<I>
where
    I: NumOps + Copy + num_traits::identities::Zero,
{
    pub fn zero() -> Self {
        Self::new(I::zero(), I::zero())
    }
}

impl<I> Add<Vec2<I>> for Vec2<I>
where
    I: NumOps + Copy,
{
    type Output = Self;
    fn add(self, rhs: Vec2<I>) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<I> AddAssign<Vec2<I>> for Vec2<I>
where
    I: NumOps + Copy,
{
    fn add_assign(&mut self, rhs: Vec2<I>) {
        self.x = self.x + rhs.x;
        self.y = self.y + rhs.y;
    }
}

impl<I> Sub<Vec2<I>> for Vec2<I>
where
    I: NumOps + Copy,
{
    type Output = Self;
    fn sub(self, rhs: Vec2<I>) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<I> SubAssign<Vec2<I>> for Vec2<I>
where
    I: NumOps + Copy,
{
    fn sub_assign(&mut self, rhs: Vec2<I>) {
        self.x = self.x - rhs.x;
        self.y = self.y - rhs.y;
    }
}

impl<I, N> Neg for Vec2<I>
where
    I: NumOps + Neg<Output = N> + Copy,
    N: NumOps + Copy,
{
    type Output = Vec2<<I as Neg>::Output>;
    fn neg(self) -> Self::Output {
        Self::Output {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl<I> Mul<I> for Vec2<I>
where
    I: NumOps + Copy,
{
    type Output = Self;
    fn mul(self, rhs: I) -> Self::Output {
        Vec2 {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl<I> Div<I> for Vec2<I>
where
    I: NumOps + Copy,
{
    type Output = Self;
    fn div(self, rhs: I) -> Self::Output {
        Vec2 {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl<I> From<(I, I)> for Vec2<I>
where
    I: NumOps + Copy,
{
    fn from(tup: (I, I)) -> Self {
        Self::new(tup.0, tup.1)
    }
}

impl<I> From<Vec2<I>> for (I, I)
where
    I: NumOps + Copy,
{
    fn from(v: Vec2<I>) -> (I, I) {
        (v.x, v.y)
    }
}
