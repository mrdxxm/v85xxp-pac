#[doc = "Register `ADCDATATHD1_0` reader"]
pub struct R(crate::R<ADCDATATHD1_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADCDATATHD1_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADCDATATHD1_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADCDATATHD1_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADCDATATHD1_0` writer"]
pub struct W(crate::W<ADCDATATHD1_0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADCDATATHD1_0_SPEC>;
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
impl From<crate::W<ADCDATATHD1_0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADCDATATHD1_0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LOWER_THD0` reader - ADC data below this setting would trigger the interrupt or status update 00: Disable"]
pub type LOWER_THD0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LOWER_THD0` writer - ADC data below this setting would trigger the interrupt or status update 00: Disable"]
pub type LOWER_THD0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ADCDATATHD1_0_SPEC, u8, u8, 8, O>;
#[doc = "Field `UPPER_THD0` reader - ADC data over this setting would trigger the interrupt or status update FF: Disable"]
pub type UPPER_THD0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `UPPER_THD0` writer - ADC data over this setting would trigger the interrupt or status update FF: Disable"]
pub type UPPER_THD0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ADCDATATHD1_0_SPEC, u8, u8, 8, O>;
#[doc = "Field `LOWER_THD1` reader - ADC data below this setting would trigger the interrupt or status update 00: Disable"]
pub type LOWER_THD1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LOWER_THD1` writer - ADC data below this setting would trigger the interrupt or status update 00: Disable"]
pub type LOWER_THD1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ADCDATATHD1_0_SPEC, u8, u8, 8, O>;
#[doc = "Field `UPPER_THD1` reader - ADC data over this setting would trigger the interrupt or status update FF: Disable"]
pub type UPPER_THD1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `UPPER_THD1` writer - ADC data over this setting would trigger the interrupt or status update FF: Disable"]
pub type UPPER_THD1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ADCDATATHD1_0_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - ADC data below this setting would trigger the interrupt or status update 00: Disable"]
    #[inline(always)]
    pub fn lower_thd0(&self) -> LOWER_THD0_R {
        LOWER_THD0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - ADC data over this setting would trigger the interrupt or status update FF: Disable"]
    #[inline(always)]
    pub fn upper_thd0(&self) -> UPPER_THD0_R {
        UPPER_THD0_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - ADC data below this setting would trigger the interrupt or status update 00: Disable"]
    #[inline(always)]
    pub fn lower_thd1(&self) -> LOWER_THD1_R {
        LOWER_THD1_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - ADC data over this setting would trigger the interrupt or status update FF: Disable"]
    #[inline(always)]
    pub fn upper_thd1(&self) -> UPPER_THD1_R {
        UPPER_THD1_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - ADC data below this setting would trigger the interrupt or status update 00: Disable"]
    #[inline(always)]
    #[must_use]
    pub fn lower_thd0(&mut self) -> LOWER_THD0_W<0> {
        LOWER_THD0_W::new(self)
    }
    #[doc = "Bits 8:15 - ADC data over this setting would trigger the interrupt or status update FF: Disable"]
    #[inline(always)]
    #[must_use]
    pub fn upper_thd0(&mut self) -> UPPER_THD0_W<8> {
        UPPER_THD0_W::new(self)
    }
    #[doc = "Bits 16:23 - ADC data below this setting would trigger the interrupt or status update 00: Disable"]
    #[inline(always)]
    #[must_use]
    pub fn lower_thd1(&mut self) -> LOWER_THD1_W<16> {
        LOWER_THD1_W::new(self)
    }
    #[doc = "Bits 24:31 - ADC data over this setting would trigger the interrupt or status update FF: Disable"]
    #[inline(always)]
    #[must_use]
    pub fn upper_thd1(&mut self) -> UPPER_THD1_W<24> {
        UPPER_THD1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ANA_ADCDATATHD1_0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adcdatathd1_0](index.html) module"]
pub struct ADCDATATHD1_0_SPEC;
impl crate::RegisterSpec for ADCDATATHD1_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adcdatathd1_0::R](R) reader structure"]
impl crate::Readable for ADCDATATHD1_0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adcdatathd1_0::W](W) writer structure"]
impl crate::Writable for ADCDATATHD1_0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADCDATATHD1_0 to value 0"]
impl crate::Resettable for ADCDATATHD1_0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
