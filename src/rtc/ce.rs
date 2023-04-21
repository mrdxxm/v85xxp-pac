#[doc = "Register `CE` reader"]
pub struct R(crate::R<CE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CE` writer"]
pub struct W(crate::W<CE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<CE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CE` reader - This register is used to unlock the access to RTC register."]
pub type CE_R = crate::BitReader<bool>;
#[doc = "Field `BSY` reader - This flag is used to indicated the RTC update procedure or RTC read procedure is ongoing."]
pub type BSY_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - This register is used to unlock the access to RTC register."]
    #[inline(always)]
    pub fn ce(&self) -> CE_R {
        CE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - This flag is used to indicated the RTC update procedure or RTC read procedure is ongoing."]
    #[inline(always)]
    pub fn bsy(&self) -> BSY_R {
        BSY_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC write enable control register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ce](index.html) module"]
pub struct CE_SPEC;
impl crate::RegisterSpec for CE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ce::R](R) reader structure"]
impl crate::Readable for CE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ce::W](W) writer structure"]
impl crate::Writable for CE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CE to value 0"]
impl crate::Resettable for CE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
