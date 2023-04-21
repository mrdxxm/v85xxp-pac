#[doc = "Register `STS` reader"]
pub struct R(crate::R<STS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `IOASTS` reader - Each bit represents the current IOA input data value."]
pub type IOASTS_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Each bit represents the current IOA input data value."]
    #[inline(always)]
    pub fn ioasts(&self) -> IOASTS_R {
        IOASTS_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "IOA input status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sts](index.html) module"]
pub struct STS_SPEC;
impl crate::RegisterSpec for STS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sts::R](R) reader structure"]
impl crate::Readable for STS_SPEC {
    type Reader = R;
}
