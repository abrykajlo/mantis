use std::ops::{
    Add, AddAssign,
    Sub, SubAssign,
    Mul, MulAssign,
    Div, DivAssign,
    Index, IndexMut,
    Neg
};

#[derive(Copy, Clone)]
pub struct Vec3 {
    e: [f32; 3],
}

impl Vec3 {
    #[inline(always)]
    pub fn new(e0: f32, e1: f32, e2: f32) -> Vec3 {
        Vec3 {
            e: [e0, e1, e2],
        }
    }

    #[inline(always)]
    pub fn x(&self) -> f32 {
        self.e[0]
    }

    #[inline(always)]
    pub fn y(&self) -> f32 {
        self.e[1]
    }

    #[inline(always)]
    pub fn z(&self) -> f32 {
        self.e[2]
    }

    #[inline(always)]
    pub fn r(&self) -> f32 {
        self.e[0]
    }

    #[inline(always)]
    pub fn g(&self) -> f32 {
        self.e[1]
    }

    #[inline(always)]
    pub fn b(&self) -> f32 {
        self.e[2]
    }

    #[inline(always)]
    pub fn length(&self) -> f32 {
        self.squared_length().sqrt()
    }

    #[inline(always)]
    pub fn squared_length(&self) -> f32 {
        self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
    }

    #[inline(always)]
    pub fn make_unit_vector(&mut self) {
        *self /= self.length();
    }

    #[inline(always)]
    pub fn unit_vector(&self) -> Vec3 {
        self / self.length()
    }
}

impl<'a> Neg for &'a Vec3 {
    type Output = Vec3;
    #[inline(always)]
    fn neg(self) -> Self::Output {
        Vec3 {
            e: [
                -self.e[0],
                -self.e[1],
                -self.e[2],
            ]
        }
    }
}

impl Index<usize> for Vec3 {
    type Output = f32;
    #[inline(always)]
    fn index(&self, idx: usize) -> &f32 {
        &self.e[idx]
    }
}

impl IndexMut<usize> for Vec3 {
    #[inline(always)]
    fn index_mut(&mut self, index: usize) -> &mut f32 {
        &mut self.e[index]
    }
}

impl MulAssign<f32> for Vec3 {
    #[inline(always)]
    fn mul_assign(&mut self, rhs: f32) {
        self.e[0] *= rhs;
        self.e[1] *= rhs;
        self.e[2] *= rhs;
    }
}

impl DivAssign<f32> for Vec3 {
    #[inline(always)]
    fn div_assign(&mut self, rhs: f32) {
        self.e[0] /= rhs;
        self.e[1] /= rhs;
        self.e[2] /= rhs;
    }
}


impl<'a> Mul<f32> for &'a Vec3 {
    type Output = Vec3;
    #[inline(always)]
    fn mul(self, rhs: f32) -> Vec3 {
        Vec3 {
            e: [
                self.e[0] * rhs,
                self.e[1] * rhs,
                self.e[2] * rhs
            ],
        }
    }
}

impl<'a> Div<f32> for &'a Vec3 {
    type Output = Vec3;
    #[inline(always)]
    fn div(self, rhs: f32) -> Vec3 {
        Vec3 {
            e: [
                self.e[0] / rhs,
                self.e[1] / rhs,
                self.e[2] / rhs
            ],
        }
    }
}

macro_rules! impl_op_assign_vec {
($trait:ident, $func:ident, $op:tt) => {
impl<'a> $trait<&'a Vec3> for Vec3 {
    #[inline(always)]
    fn $func(&mut self, rhs: &'a Vec3) {
        self.e[0] $op rhs.e[0];
        self.e[1] $op rhs.e[1];
        self.e[2] $op rhs.e[3];
    }
}

impl $trait for Vec3 {
    #[inline(always)]
    fn $func(&mut self, rhs: Vec3) {
        self.e[0] $op rhs.e[0];
        self.e[1] $op rhs.e[1];
        self.e[2] $op rhs.e[3];
    }
}
};
}

macro_rules! impl_op_vec {
($trait:ident, $func:ident, $op:tt) => {
impl<'a> $trait for &'a Vec3 {
    type Output = Vec3;
    #[inline(always)]
    fn $func(self, rhs: &'a Vec3) -> Vec3 {
        Vec3 {
            e: [
                self.e[0] $op rhs.e[0],
                self.e[1] $op rhs.e[1],
                self.e[2] $op rhs.e[2],
            ],
        }
    }
}

impl<'a> $trait<Vec3> for &'a Vec3 {
    type Output = Vec3;
    #[inline(always)]
    fn $func(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            e: [
                self.e[0] $op rhs.e[0],
                self.e[1] $op rhs.e[1],
                self.e[2] $op rhs.e[2],
            ],
        }
    }
}

impl $trait for Vec3 {
    type Output = Vec3;
    #[inline(always)]
    fn $func(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            e: [
                self.e[0] $op rhs.e[0],
                self.e[1] $op rhs.e[1],
                self.e[2] $op rhs.e[2],
            ],
        }
    }
}

impl<'a> $trait<&'a Vec3> for Vec3 {
    type Output = Vec3;
    #[inline(always)]
    fn $func(self, rhs: &'a Vec3) -> Vec3 {
        Vec3 {
            e: [
                self.e[0] $op rhs.e[0],
                self.e[1] $op rhs.e[1],
                self.e[2] $op rhs.e[2],
            ],
        }
    }
}
};
}

impl_op_assign_vec!(AddAssign, add_assign, +=);
impl_op_assign_vec!(SubAssign, sub_assign, -=);
impl_op_assign_vec!(MulAssign, mul_assign, *=);
impl_op_assign_vec!(DivAssign, div_assign, /=);

impl_op_vec!(Add, add, +);
impl_op_vec!(Sub, sub, -);
impl_op_vec!(Mul, mul, *);
impl_op_vec!(Div, div, /);
