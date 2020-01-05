use super::*;
// -- Mul
impl Mul for I16x1 {
    type Output = I16x1;

    #[inline(always)]
    fn mul(self, rhs: I16x1) -> I16x1 {
        I16x1(self.0.wrapping_mul(rhs.0))
    }
}

impl Mul for I32x1 {
    type Output = I32x1;

    #[inline(always)]
    fn mul(self, rhs: I32x1) -> I32x1 {
        I32x1(self.0.wrapping_mul(rhs.0))
    }
}

impl Mul for I64x1 {
    type Output = I64x1;

    #[inline(always)]
    fn mul(self, rhs: I64x1) -> I64x1 {
        I64x1(self.0.wrapping_mul(rhs.0))
    }
}

impl Mul for F32x1 {
    type Output = F32x1;

    #[inline(always)]
    fn mul(self, rhs: F32x1) -> F32x1 {
        F32x1(self.0 * rhs.0)
    }
}

impl Mul for F64x1 {
    type Output = F64x1;

    #[inline(always)]
    fn mul(self, rhs: F64x1) -> F64x1 {
        F64x1(self.0 * rhs.0)
    }
}

impl Mul for I16x16 {
    type Output = I16x16;

    #[inline(always)]
    fn mul(self, rhs: I16x16) -> I16x16 {
        I16x16(unsafe { _mm256_mullo_epi16(self.0, rhs.0) })
    }
}
