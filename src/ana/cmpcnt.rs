#[doc = "Register `CMPCNT%s` reader"]
pub struct R(crate::R<CMPCNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMPCNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMPCNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMPCNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CMPCNT%s` writer"]
pub struct W(crate::W<CMPCNT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMPCNT_SPEC>;
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
impl From<crate::W<CMPCNT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMPCNT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CNT` reader - This register store the happen times of comparator x according to the setting in COMPx_SEL."]
pub type CNT_R = crate::FieldReader<u32, u32>;
#[doc = "Field `CNT` writer - This register store the happen times of comparator x according to the setting in COMPx_SEL."]
pub type CNT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CMPCNT_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - This register store the happen times of comparator x according to the setting in COMPx_SEL."]
    #[inline(always)]
    pub fn cnt(&self) -> CNT_R {
        CNT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - This register store the happen times of comparator x according to the setting in COMPx_SEL."]
    #[inline(always)]
    #[must_use]
    pub fn cnt(&mut self) -> CNT_W<0> {
        CNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Comparator x counter.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmpcnt](index.html) module"]
pub struct CMPCNT_SPEC;
impl crate::RegisterSpec for CMPCNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cmpcnt::R](R) reader structure"]
impl crate::Readable for CMPCNT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cmpcnt::W](W) writer structure"]
impl crate::Writable for CMPCNT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CMPCNT%s to value 0"]
impl crate::Resettable for CMPCNT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
