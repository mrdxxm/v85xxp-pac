#[doc = "Register `DSLEEPPASS` reader"]
pub struct R(crate::R<DSLEEPPASS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DSLEEPPASS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DSLEEPPASS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DSLEEPPASS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DSLEEPPASS` writer"]
pub struct W(crate::W<DSLEEPPASS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DSLEEPPASS_SPEC>;
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
impl From<crate::W<DSLEEPPASS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DSLEEPPASS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UNLOCK` reader - This bit indicates the entry of deep-sleep mode has been unlocked and is ready to entry deep-sleep mode."]
pub type UNLOCK_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - This bit indicates the entry of deep-sleep mode has been unlocked and is ready to entry deep-sleep mode."]
    #[inline(always)]
    pub fn unlock(&self) -> UNLOCK_R {
        UNLOCK_R::new((self.bits & 1) != 0)
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
#[doc = "PMU deep sleep password register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dsleeppass](index.html) module"]
pub struct DSLEEPPASS_SPEC;
impl crate::RegisterSpec for DSLEEPPASS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dsleeppass::R](R) reader structure"]
impl crate::Readable for DSLEEPPASS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dsleeppass::W](W) writer structure"]
impl crate::Writable for DSLEEPPASS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DSLEEPPASS to value 0"]
impl crate::Resettable for DSLEEPPASS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
