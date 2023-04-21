#[doc = "Register `INTSTS` reader"]
pub struct R(crate::R<INTSTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTSTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTSTS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTSTS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTSTS` writer"]
pub struct W(crate::W<INTSTS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTSTS_SPEC>;
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
impl From<crate::W<INTSTS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTSTS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CSERR` reader - Checksum error status bit."]
pub type CSERR_R = crate::BitReader<bool>;
#[doc = "Field `CSERR` writer - Checksum error status bit."]
pub type CSERR_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, INTSTS_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Checksum error status bit."]
    #[inline(always)]
    pub fn cserr(&self) -> CSERR_R {
        CSERR_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Checksum error status bit."]
    #[inline(always)]
    #[must_use]
    pub fn cserr(&mut self) -> CSERR_W<0> {
        CSERR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FLASH Checksum interrupt status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intsts](index.html) module"]
pub struct INTSTS_SPEC;
impl crate::RegisterSpec for INTSTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intsts::R](R) reader structure"]
impl crate::Readable for INTSTS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intsts::W](W) writer structure"]
impl crate::Writable for INTSTS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x01;
}
#[doc = "`reset()` method sets INTSTS to value 0"]
impl crate::Resettable for INTSTS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
