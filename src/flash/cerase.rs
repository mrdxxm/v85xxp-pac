#[doc = "Register `CERASE` reader"]
pub struct R(crate::R<CERASE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CERASE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CERASE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CERASE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CERASE` writer"]
pub struct W(crate::W<CERASE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CERASE_SPEC>;
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
impl From<crate::W<CERASE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CERASE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CERASE` reader - This bit is used to indicate if the chip erase is ongoing or not."]
pub type CERASE_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - This bit is used to indicate if the chip erase is ongoing or not."]
    #[inline(always)]
    pub fn cerase(&self) -> CERASE_R {
        CERASE_R::new((self.bits & 1) != 0)
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
#[doc = "FLASH chip erase control register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cerase](index.html) module"]
pub struct CERASE_SPEC;
impl crate::RegisterSpec for CERASE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cerase::R](R) reader structure"]
impl crate::Readable for CERASE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cerase::W](W) writer structure"]
impl crate::Writable for CERASE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CERASE to value 0"]
impl crate::Resettable for CERASE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
