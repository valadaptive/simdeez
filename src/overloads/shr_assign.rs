use super::*;
impl ShrAssign<i32> for I16x1 {
    #[inline(always)]
    fn shr_assign(&mut self, rhs: i32) {
        *self = I16x1(self.0 >> rhs);
    }
}

impl ShrAssign<i32> for I32x1 {
    #[inline(always)]
    fn shr_assign(&mut self, rhs: i32) {
        *self = I32x1(self.0 >> rhs);
    }
}

impl ShrAssign<i32> for I64x1 {
    #[inline(always)]
    fn shr_assign(&mut self, rhs: i32) {
        *self = I64x1(self.0 >> rhs);
    }
}

impl ShrAssign<i32> for F32x1 {
    #[inline(always)]
    fn shr_assign(&mut self, rhs: i32) {
        let bits = self.0.to_bits() >> rhs;
        *self = F32x1(f32::from_bits(bits));
    }
}

impl ShrAssign<i32> for F64x1 {
    #[inline(always)]
    fn shr_assign(&mut self, rhs: i32) {
        let bits = self.0.to_bits() >> rhs;
        *self = F64x1(f64::from_bits(bits));
    }
}

impl ShrAssign<i32> for I16x16 {
    #[inline(always)]
    fn shr_assign(&mut self, rhs: i32) {
        macro_rules! call {
            ($rhs:expr) => {
                *self = unsafe { I16x16(_mm256_srai_epi16(self.0, $rhs)) }
            };
        }
        constify_imm8!(rhs, call)
    }
}


