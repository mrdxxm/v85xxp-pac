#[doc = "Register `REGB` reader"]
pub struct R(crate::R<REGB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<REGB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<REGB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<REGB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `REGB` writer"]
pub struct W(crate::W<REGB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<REGB_SPEC>;
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
impl From<crate::W<REGB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<REGB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RCLTRIM` reader - Trimming of 32kHz RC. 00000~01111: increased by 4% for each step; 10000~11111: decreased by 4% for each step;"]
pub type RCLTRIM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RCLTRIM` writer - Trimming of 32kHz RC. 00000~01111: increased by 4% for each step; 10000~11111: decreased by 4% for each step;"]
pub type RCLTRIM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, REGB_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 0:4 - Trimming of 32kHz RC. 00000~01111: increased by 4% for each step; 10000~11111: decreased by 4% for each step;"]
    #[inline(always)]
    pub fn rcltrim(&self) -> RCLTRIM_R {
        RCLTRIM_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Trimming of 32kHz RC. 00000~01111: increased by 4% for each step; 10000~11111: decreased by 4% for each step;"]
    #[inline(always)]
    #[must_use]
    pub fn rcltrim(&mut self) -> RCLTRIM_W<0> {
        RCLTRIM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Analog register 11.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [regb](index.html) module"]
pub struct REGB_SPEC;
impl crate::RegisterSpec for REGB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [regb::R](R) reader structure"]
impl crate::Readable for REGB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [regb::W](W) writer structure"]
impl crate::Writable for REGB_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets REGB to value 0"]
impl crate::Resettable for REGB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
