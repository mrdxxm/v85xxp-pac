#[doc = "Register `ADCDATATHD3_2` reader"]
pub struct R(crate::R<ADCDATATHD3_2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADCDATATHD3_2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADCDATATHD3_2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADCDATATHD3_2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADCDATATHD3_2` writer"]
pub struct W(crate::W<ADCDATATHD3_2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADCDATATHD3_2_SPEC>;
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
impl From<crate::W<ADCDATATHD3_2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADCDATATHD3_2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LOWER_THD2` reader - ADC data below this setting would trigger the interrupt or status update 00: Disable"]
pub type LOWER_THD2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LOWER_THD2` writer - ADC data below this setting would trigger the interrupt or status update 00: Disable"]
pub type LOWER_THD2_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ADCDATATHD3_2_SPEC, u8, u8, 8, O>;
#[doc = "Field `UPPER_THD2` reader - ADC data over this setting would trigger the interrupt or status update FF: Disable"]
pub type UPPER_THD2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `UPPER_THD2` writer - ADC data over this setting would trigger the interrupt or status update FF: Disable"]
pub type UPPER_THD2_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ADCDATATHD3_2_SPEC, u8, u8, 8, O>;
#[doc = "Field `LOWER_THD3` reader - ADC data below this setting would trigger the interrupt or status update 00: Disable"]
pub type LOWER_THD3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LOWER_THD3` writer - ADC data below this setting would trigger the interrupt or status update 00: Disable"]
pub type LOWER_THD3_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ADCDATATHD3_2_SPEC, u8, u8, 8, O>;
#[doc = "Field `UPPER_THD3` reader - ADC data over this setting would trigger the interrupt or status update FF: Disable"]
pub type UPPER_THD3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `UPPER_THD3` writer - ADC data over this setting would trigger the interrupt or status update FF: Disable"]
pub type UPPER_THD3_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ADCDATATHD3_2_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - ADC data below this setting would trigger the interrupt or status update 00: Disable"]
    #[inline(always)]
    pub fn lower_thd2(&self) -> LOWER_THD2_R {
        LOWER_THD2_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - ADC data over this setting would trigger the interrupt or status update FF: Disable"]
    #[inline(always)]
    pub fn upper_thd2(&self) -> UPPER_THD2_R {
        UPPER_THD2_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - ADC data below this setting would trigger the interrupt or status update 00: Disable"]
    #[inline(always)]
    pub fn lower_thd3(&self) -> LOWER_THD3_R {
        LOWER_THD3_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - ADC data over this setting would trigger the interrupt or status update FF: Disable"]
    #[inline(always)]
    pub fn upper_thd3(&self) -> UPPER_THD3_R {
        UPPER_THD3_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - ADC data below this setting would trigger the interrupt or status update 00: Disable"]
    #[inline(always)]
    #[must_use]
    pub fn lower_thd2(&mut self) -> LOWER_THD2_W<0> {
        LOWER_THD2_W::new(self)
    }
    #[doc = "Bits 8:15 - ADC data over this setting would trigger the interrupt or status update FF: Disable"]
    #[inline(always)]
    #[must_use]
    pub fn upper_thd2(&mut self) -> UPPER_THD2_W<8> {
        UPPER_THD2_W::new(self)
    }
    #[doc = "Bits 16:23 - ADC data below this setting would trigger the interrupt or status update 00: Disable"]
    #[inline(always)]
    #[must_use]
    pub fn lower_thd3(&mut self) -> LOWER_THD3_W<16> {
        LOWER_THD3_W::new(self)
    }
    #[doc = "Bits 24:31 - ADC data over this setting would trigger the interrupt or status update FF: Disable"]
    #[inline(always)]
    #[must_use]
    pub fn upper_thd3(&mut self) -> UPPER_THD3_W<24> {
        UPPER_THD3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ANA_ADCDATATHD3_2.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adcdatathd3_2](index.html) module"]
pub struct ADCDATATHD3_2_SPEC;
impl crate::RegisterSpec for ADCDATATHD3_2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adcdatathd3_2::R](R) reader structure"]
impl crate::Readable for ADCDATATHD3_2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adcdatathd3_2::W](W) writer structure"]
impl crate::Writable for ADCDATATHD3_2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADCDATATHD3_2 to value 0"]
impl crate::Resettable for ADCDATATHD3_2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
