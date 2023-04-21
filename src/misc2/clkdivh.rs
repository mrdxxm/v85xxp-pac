#[doc = "Register `CLKDIVH` reader"]
pub struct R(crate::R<CLKDIVH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLKDIVH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLKDIVH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLKDIVH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLKDIVH` writer"]
pub struct W(crate::W<CLKDIVH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLKDIVH_SPEC>;
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
impl From<crate::W<CLKDIVH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLKDIVH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLKDIVH` reader - This register is used to control AHB clock divider."]
pub type CLKDIVH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CLKDIVH` writer - This register is used to control AHB clock divider."]
pub type CLKDIVH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CLKDIVH_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - This register is used to control AHB clock divider."]
    #[inline(always)]
    pub fn clkdivh(&self) -> CLKDIVH_R {
        CLKDIVH_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - This register is used to control AHB clock divider."]
    #[inline(always)]
    #[must_use]
    pub fn clkdivh(&mut self) -> CLKDIVH_W<0> {
        CLKDIVH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AHB clock divider control register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clkdivh](index.html) module"]
pub struct CLKDIVH_SPEC;
impl crate::RegisterSpec for CLKDIVH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clkdivh::R](R) reader structure"]
impl crate::Readable for CLKDIVH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clkdivh::W](W) writer structure"]
impl crate::Writable for CLKDIVH_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLKDIVH to value 0"]
impl crate::Resettable for CLKDIVH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
