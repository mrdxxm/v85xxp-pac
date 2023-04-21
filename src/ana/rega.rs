#[doc = "Register `REGA` reader"]
pub struct R(crate::R<REGA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<REGA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<REGA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<REGA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `REGA` writer"]
pub struct W(crate::W<REGA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<REGA_SPEC>;
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
impl From<crate::W<REGA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<REGA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VDCINDETPD` reader - PD VDCIN detector."]
pub type VDCINDETPD_R = crate::BitReader<bool>;
#[doc = "Field `VDCINDETPD` writer - PD VDCIN detector."]
pub type VDCINDETPD_W<'a, const O: u8> = crate::BitWriter<'a, u32, REGA_SPEC, bool, O>;
impl R {
    #[doc = "Bit 7 - PD VDCIN detector."]
    #[inline(always)]
    pub fn vdcindetpd(&self) -> VDCINDETPD_R {
        VDCINDETPD_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 7 - PD VDCIN detector."]
    #[inline(always)]
    #[must_use]
    pub fn vdcindetpd(&mut self) -> VDCINDETPD_W<7> {
        VDCINDETPD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Analog register 10.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rega](index.html) module"]
pub struct REGA_SPEC;
impl crate::RegisterSpec for REGA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rega::R](R) reader structure"]
impl crate::Readable for REGA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rega::W](W) writer structure"]
impl crate::Writable for REGA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets REGA to value 0"]
impl crate::Resettable for REGA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
