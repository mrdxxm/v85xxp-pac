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
#[doc = "Field `INTSTS0` reader - Interrupt status 0."]
pub type INTSTS0_R = crate::BitReader<bool>;
#[doc = "Field `INTSTS0` writer - Interrupt status 0."]
pub type INTSTS0_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, INTSTS_SPEC, bool, O>;
#[doc = "Field `INTSTS1` reader - Interrupt status 1."]
pub type INTSTS1_R = crate::BitReader<bool>;
#[doc = "Field `INTSTS1` writer - Interrupt status 1."]
pub type INTSTS1_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, INTSTS_SPEC, bool, O>;
#[doc = "Field `INTSTS2` reader - Interrupt status 2."]
pub type INTSTS2_R = crate::BitReader<bool>;
#[doc = "Field `INTSTS2` writer - Interrupt status 2."]
pub type INTSTS2_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, INTSTS_SPEC, bool, O>;
#[doc = "Field `INTSTS3` reader - Interrupt status 3."]
pub type INTSTS3_R = crate::BitReader<bool>;
#[doc = "Field `INTSTS3` writer - Interrupt status 3."]
pub type INTSTS3_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, INTSTS_SPEC, bool, O>;
#[doc = "Field `INTSTS4` reader - Interrupt status 4."]
pub type INTSTS4_R = crate::BitReader<bool>;
#[doc = "Field `INTSTS4` writer - Interrupt status 4."]
pub type INTSTS4_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, INTSTS_SPEC, bool, O>;
#[doc = "Field `INTSTS5` reader - Interrupt status 5."]
pub type INTSTS5_R = crate::BitReader<bool>;
#[doc = "Field `INTSTS5` writer - Interrupt status 5."]
pub type INTSTS5_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, INTSTS_SPEC, bool, O>;
#[doc = "Field `INTSTS6` reader - Interrupt status 6."]
pub type INTSTS6_R = crate::BitReader<bool>;
#[doc = "Field `INTSTS6` writer - Interrupt status 6."]
pub type INTSTS6_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, INTSTS_SPEC, bool, O>;
#[doc = "Field `INTSTS8` reader - Interrupt status 8."]
pub type INTSTS8_R = crate::BitReader<bool>;
#[doc = "Field `INTSTS8` writer - Interrupt status 8."]
pub type INTSTS8_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, INTSTS_SPEC, bool, O>;
#[doc = "Field `INTSTS10` reader - Interrupt status alarm."]
pub type INTSTS10_R = crate::BitReader<bool>;
#[doc = "Field `INTSTS10` writer - Interrupt status alarm."]
pub type INTSTS10_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, INTSTS_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Interrupt status 0."]
    #[inline(always)]
    pub fn intsts0(&self) -> INTSTS0_R {
        INTSTS0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Interrupt status 1."]
    #[inline(always)]
    pub fn intsts1(&self) -> INTSTS1_R {
        INTSTS1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Interrupt status 2."]
    #[inline(always)]
    pub fn intsts2(&self) -> INTSTS2_R {
        INTSTS2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt status 3."]
    #[inline(always)]
    pub fn intsts3(&self) -> INTSTS3_R {
        INTSTS3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Interrupt status 4."]
    #[inline(always)]
    pub fn intsts4(&self) -> INTSTS4_R {
        INTSTS4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Interrupt status 5."]
    #[inline(always)]
    pub fn intsts5(&self) -> INTSTS5_R {
        INTSTS5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Interrupt status 6."]
    #[inline(always)]
    pub fn intsts6(&self) -> INTSTS6_R {
        INTSTS6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Interrupt status 8."]
    #[inline(always)]
    pub fn intsts8(&self) -> INTSTS8_R {
        INTSTS8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - Interrupt status alarm."]
    #[inline(always)]
    pub fn intsts10(&self) -> INTSTS10_R {
        INTSTS10_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt status 0."]
    #[inline(always)]
    #[must_use]
    pub fn intsts0(&mut self) -> INTSTS0_W<0> {
        INTSTS0_W::new(self)
    }
    #[doc = "Bit 1 - Interrupt status 1."]
    #[inline(always)]
    #[must_use]
    pub fn intsts1(&mut self) -> INTSTS1_W<1> {
        INTSTS1_W::new(self)
    }
    #[doc = "Bit 2 - Interrupt status 2."]
    #[inline(always)]
    #[must_use]
    pub fn intsts2(&mut self) -> INTSTS2_W<2> {
        INTSTS2_W::new(self)
    }
    #[doc = "Bit 3 - Interrupt status 3."]
    #[inline(always)]
    #[must_use]
    pub fn intsts3(&mut self) -> INTSTS3_W<3> {
        INTSTS3_W::new(self)
    }
    #[doc = "Bit 4 - Interrupt status 4."]
    #[inline(always)]
    #[must_use]
    pub fn intsts4(&mut self) -> INTSTS4_W<4> {
        INTSTS4_W::new(self)
    }
    #[doc = "Bit 5 - Interrupt status 5."]
    #[inline(always)]
    #[must_use]
    pub fn intsts5(&mut self) -> INTSTS5_W<5> {
        INTSTS5_W::new(self)
    }
    #[doc = "Bit 6 - Interrupt status 6."]
    #[inline(always)]
    #[must_use]
    pub fn intsts6(&mut self) -> INTSTS6_W<6> {
        INTSTS6_W::new(self)
    }
    #[doc = "Bit 8 - Interrupt status 8."]
    #[inline(always)]
    #[must_use]
    pub fn intsts8(&mut self) -> INTSTS8_W<8> {
        INTSTS8_W::new(self)
    }
    #[doc = "Bit 10 - Interrupt status alarm."]
    #[inline(always)]
    #[must_use]
    pub fn intsts10(&mut self) -> INTSTS10_W<10> {
        INTSTS10_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC interrupt status control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intsts](index.html) module"]
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
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x057f;
}
#[doc = "`reset()` method sets INTSTS to value 0"]
impl crate::Resettable for INTSTS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
