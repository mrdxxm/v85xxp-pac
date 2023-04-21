#[doc = "Register `FLASHWC` reader"]
pub struct R(crate::R<FLASHWC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLASHWC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLASHWC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLASHWC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FLASHWC` writer"]
pub struct W(crate::W<FLASHWC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLASHWC_SPEC>;
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
impl From<crate::W<FLASHWC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLASHWC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CYCLE_1US` reader - This register is used for Flash controller to calculate 1us tick from AHB clock 1us tick = (AHB clock period)*(CYCLE_1US+1)"]
pub type CYCLE_1US_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CYCLE_1US` writer - This register is used for Flash controller to calculate 1us tick from AHB clock 1us tick = (AHB clock period)*(CYCLE_1US+1)"]
pub type CYCLE_1US_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FLASHWC_SPEC, u8, u8, 6, O>;
impl R {
    #[doc = "Bits 8:13 - This register is used for Flash controller to calculate 1us tick from AHB clock 1us tick = (AHB clock period)*(CYCLE_1US+1)"]
    #[inline(always)]
    pub fn cycle_1us(&self) -> CYCLE_1US_R {
        CYCLE_1US_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 8:13 - This register is used for Flash controller to calculate 1us tick from AHB clock 1us tick = (AHB clock period)*(CYCLE_1US+1)"]
    #[inline(always)]
    #[must_use]
    pub fn cycle_1us(&mut self) -> CYCLE_1US_W<8> {
        CYCLE_1US_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flash wait cycle register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flashwc](index.html) module"]
pub struct FLASHWC_SPEC;
impl crate::RegisterSpec for FLASHWC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [flashwc::R](R) reader structure"]
impl crate::Readable for FLASHWC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [flashwc::W](W) writer structure"]
impl crate::Writable for FLASHWC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FLASHWC to value 0"]
impl crate::Resettable for FLASHWC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
