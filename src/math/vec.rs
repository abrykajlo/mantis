use std::ops;

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
}

impl<'a> ops::Neg for &'a Vec3 {
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

impl ops::Index<usize> for Vec3 {
    type Output = f32;
    #[inline(always)]
    fn index(&self, idx: usize) -> &f32 {
        &self.e[idx]
    }
}

impl ops::IndexMut<usize> for Vec3 {
    #[inline(always)]
    fn index_mut(&mut self, index: usize) -> &mut f32 {
        &mut self.e[index]
    }
}

impl<'a> ops::AddAssign<&'a Vec3> for Vec3 {
    #[inline(always)]
    fn add_assign(&mut self, rhs: &'a Vec3) {
        self.e[0] += rhs.e[0];
        self.e[1] += rhs.e[1];
        self.e[2] += rhs.e[3];
    }
}

impl<'a> ops::SubAssign<&'a Vec3> for Vec3 {
    #[inline(always)]
    fn sub_assign(&mut self, rhs: &'a Vec3) {
        self.e[0] -= rhs.e[0];
        self.e[1] -= rhs.e[1];
        self.e[2] -= rhs.e[3];
    }
}

impl<'a> ops::MulAssign<&'a Vec3> for Vec3 {
    #[inline(always)]
    fn mul_assign(&mut self, rhs: &'a Vec3) {
        self.e[0] *= rhs.e[0];
        self.e[1] *= rhs.e[1];
        self.e[2] *= rhs.e[2];
    }
}

impl<'a> ops::DivAssign<&'a Vec3> for Vec3 {
    #[inline(always)]
    fn div_assign(&mut self, rhs: &'a Vec3) {
        self.e[0] /= rhs.e[0];
        self.e[1] /= rhs.e[1];
        self.e[2] /= rhs.e[3];
    }
}

impl ops::MulAssign<f32> for Vec3 {
    #[inline(always)]
    fn mul_assign(&mut self, rhs: f32) {
        self.e[0] *= rhs;
        self.e[1] *= rhs;
        self.e[2] *= rhs;
    }
}

impl ops::DivAssign<f32> for Vec3 {
    #[inline(always)]
    fn div_assign(&mut self, rhs: f32) {
        self.e[0] /= rhs;
        self.e[1] /= rhs;
        self.e[2] /= rhs;
    }
}