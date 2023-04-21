#[doc = "Register `SERASE` reader"]
pub struct R(crate::R<SERASE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SERASE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SERASE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SERASE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SERASE` writer"]
pub struct W(crate::W<SERASE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SERASE_SPEC>;
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
impl From<crate::W<SERASE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SERASE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SERASE` reader - This bit is used to indicate if the sector erase is ongoing or not."]
pub type SERASE_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - This bit is used to indicate if the sector erase is ongoing or not."]
    #[inline(always)]
    pub fn serase(&self) -> SERASE_R {
        SERASE_R::new((self.bits & 1) != 0)
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
#[doc = "FLASH sector erase control register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [serase](index.html) module"]
pub struct SERASE_SPEC;
impl crate::RegisterSpec for SERASE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [serase::R](R) reader structure"]
impl crate::Readable for SERASE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [serase::W](W) writer structure"]
impl crate::Writable for SERASE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SERASE to value 0"]
impl crate::Resettable for SERASE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
