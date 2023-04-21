#[doc = "Register `ADCDCPN` reader"]
pub struct R(crate::R<ADCDCPN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADCDCPN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADCDCPN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADCDCPN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DCPN1` reader - ADC DCPN1."]
pub type DCPN1_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DCPN2` reader - ADC DCPN2."]
pub type DCPN2_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DCPN3` reader - ADC DCPN3."]
pub type DCPN3_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DCPN4` reader - ADC DCPN4."]
pub type DCPN4_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:8 - ADC DCPN1."]
    #[inline(always)]
    pub fn dcpn1(&self) -> DCPN1_R {
        DCPN1_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 9:17 - ADC DCPN2."]
    #[inline(always)]
    pub fn dcpn2(&self) -> DCPN2_R {
        DCPN2_R::new(((self.bits >> 9) & 0x01ff) as u16)
    }
    #[doc = "Bits 18:26 - ADC DCPN3."]
    #[inline(always)]
    pub fn dcpn3(&self) -> DCPN3_R {
        DCPN3_R::new(((self.bits >> 18) & 0x01ff) as u16)
    }
    #[doc = "Bits 27:31 - ADC DCPN4."]
    #[inline(always)]
    pub fn dcpn4(&self) -> DCPN4_R {
        DCPN4_R::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
#[doc = "ANA_ADCDCPN.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adcdcpn](index.html) module"]
pub struct ADCDCPN_SPEC;
impl crate::RegisterSpec for ADCDCPN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adcdcpn::R](R) reader structure"]
impl crate::Readable for ADCDCPN_SPEC {
    type Reader = R;
}
