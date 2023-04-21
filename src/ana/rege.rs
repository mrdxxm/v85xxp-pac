#[doc = "Register `REGE` reader"]
pub struct R(crate::R<REGE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<REGE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<REGE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<REGE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `REGE` writer"]
pub struct W(crate::W<REGE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<REGE_SPEC>;
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
impl From<crate::W<REGE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<REGE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BKPWREN` reader - BK.power enable."]
pub type BKPWREN_R = crate::BitReader<bool>;
#[doc = "Field `BKPWREN` writer - BK.power enable."]
pub type BKPWREN_W<'a, const O: u8> = crate::BitWriter<'a, u32, REGE_SPEC, bool, O>;
impl R {
    #[doc = "Bit 7 - BK.power enable."]
    #[inline(always)]
    pub fn bkpwren(&self) -> BKPWREN_R {
        BKPWREN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 7 - BK.power enable."]
    #[inline(always)]
    #[must_use]
    pub fn bkpwren(&mut self) -> BKPWREN_W<7> {
        BKPWREN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Analog register 14.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rege](index.html) module"]
pub struct REGE_SPEC;
impl crate::RegisterSpec for REGE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rege::R](R) reader structure"]
impl crate::Readable for REGE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rege::W](W) writer structure"]
impl crate::Writable for REGE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets REGE to value 0"]
impl crate::Resettable for REGE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
