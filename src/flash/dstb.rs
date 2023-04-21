#[doc = "Register `DSTB` reader"]
pub struct R(crate::R<DSTB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DSTB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DSTB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DSTB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DSTB` writer"]
pub struct W(crate::W<DSTB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DSTB_SPEC>;
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
impl From<crate::W<DSTB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DSTB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DSTB` reader - This bit is used to indicate if the flash IP is entering deep standby."]
pub type DSTB_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - This bit is used to indicate if the flash IP is entering deep standby."]
    #[inline(always)]
    pub fn dstb(&self) -> DSTB_R {
        DSTB_R::new((self.bits & 1) != 0)
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
#[doc = "FLASH deep standby control register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dstb](index.html) module"]
pub struct DSTB_SPEC;
impl crate::RegisterSpec for DSTB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dstb::R](R) reader structure"]
impl crate::Readable for DSTB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dstb::W](W) writer structure"]
impl crate::Writable for DSTB_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DSTB to value 0"]
impl crate::Resettable for DSTB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
