#[doc = "Register `CARRY` reader"]
pub struct R(crate::R<CARRY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CARRY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CARRY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CARRY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CARRY` reader - This bit represent the carry bit after add operation is done. The bit represent borrow bit after sub operation is done. Programmer can read this register immediately after the ACT bit is set to 1."]
pub type CARRY_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - This bit represent the carry bit after add operation is done. The bit represent borrow bit after sub operation is done. Programmer can read this register immediately after the ACT bit is set to 1."]
    #[inline(always)]
    pub fn carry(&self) -> CARRY_R {
        CARRY_R::new((self.bits & 1) != 0)
    }
}
#[doc = "CRYPT carry/borrow bit register.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [carry](index.html) module"]
pub struct CARRY_SPEC;
impl crate::RegisterSpec for CARRY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [carry::R](R) reader structure"]
impl crate::Readable for CARRY_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CARRY to value 0"]
impl crate::Resettable for CARRY_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
