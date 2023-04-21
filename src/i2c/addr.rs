#[doc = "Register `ADDR` reader"]
pub struct R(crate::R<ADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADDR` writer"]
pub struct W(crate::W<ADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADDR_SPEC>;
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
impl From<crate::W<ADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GC` reader - General Call Address Acknowledge."]
pub type GC_R = crate::BitReader<bool>;
#[doc = "Field `GC` writer - General Call Address Acknowledge."]
pub type GC_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADDR_SPEC, bool, O>;
#[doc = "Field `SLA` reader - Own I2C slave address (7 bit) ."]
pub type SLA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SLA` writer - Own I2C slave address (7 bit) ."]
pub type SLA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADDR_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bit 0 - General Call Address Acknowledge."]
    #[inline(always)]
    pub fn gc(&self) -> GC_R {
        GC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:7 - Own I2C slave address (7 bit) ."]
    #[inline(always)]
    pub fn sla(&self) -> SLA_R {
        SLA_R::new(((self.bits >> 1) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - General Call Address Acknowledge."]
    #[inline(always)]
    #[must_use]
    pub fn gc(&mut self) -> GC_W<0> {
        GC_W::new(self)
    }
    #[doc = "Bits 1:7 - Own I2C slave address (7 bit) ."]
    #[inline(always)]
    #[must_use]
    pub fn sla(&mut self) -> SLA_W<1> {
        SLA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2C address register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [addr](index.html) module"]
pub struct ADDR_SPEC;
impl crate::RegisterSpec for ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [addr::R](R) reader structure"]
impl crate::Readable for ADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [addr::W](W) writer structure"]
impl crate::Writable for ADDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADDR to value 0"]
impl crate::Resettable for ADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
