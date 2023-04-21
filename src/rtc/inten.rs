#[doc = "Register `INTEN` reader"]
pub struct R(crate::R<INTEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTEN` writer"]
pub struct W(crate::W<INTEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTEN_SPEC>;
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
impl From<crate::W<INTEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INTEN0` reader - Interrupt enable 0."]
pub type INTEN0_R = crate::BitReader<bool>;
#[doc = "Field `INTEN0` writer - Interrupt enable 0."]
pub type INTEN0_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN_SPEC, bool, O>;
#[doc = "Field `INTEN1` reader - Interrupt enable 1."]
pub type INTEN1_R = crate::BitReader<bool>;
#[doc = "Field `INTEN1` writer - Interrupt enable 1."]
pub type INTEN1_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN_SPEC, bool, O>;
#[doc = "Field `INTEN2` reader - Interrupt enable 2."]
pub type INTEN2_R = crate::BitReader<bool>;
#[doc = "Field `INTEN2` writer - Interrupt enable 2."]
pub type INTEN2_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN_SPEC, bool, O>;
#[doc = "Field `INTRN3` reader - Interrupt enable 3."]
pub type INTRN3_R = crate::BitReader<bool>;
#[doc = "Field `INTRN3` writer - Interrupt enable 3."]
pub type INTRN3_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN_SPEC, bool, O>;
#[doc = "Field `INTEN4` reader - Interrupt enable 4."]
pub type INTEN4_R = crate::BitReader<bool>;
#[doc = "Field `INTEN4` writer - Interrupt enable 4."]
pub type INTEN4_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN_SPEC, bool, O>;
#[doc = "Field `INTEN5` reader - Interrupt enable 5."]
pub type INTEN5_R = crate::BitReader<bool>;
#[doc = "Field `INTEN5` writer - Interrupt enable 5."]
pub type INTEN5_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN_SPEC, bool, O>;
#[doc = "Field `INTEN6` reader - Interrupt enable 6."]
pub type INTEN6_R = crate::BitReader<bool>;
#[doc = "Field `INTEN6` writer - Interrupt enable 6."]
pub type INTEN6_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN_SPEC, bool, O>;
#[doc = "Field `INTEN8` reader - Interrupt enable 8."]
pub type INTEN8_R = crate::BitReader<bool>;
#[doc = "Field `INTEN8` writer - Interrupt enable 8."]
pub type INTEN8_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN_SPEC, bool, O>;
#[doc = "Field `INTEN10` reader - Interrupt enable alarm."]
pub type INTEN10_R = crate::BitReader<bool>;
#[doc = "Field `INTEN10` writer - Interrupt enable alarm."]
pub type INTEN10_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Interrupt enable 0."]
    #[inline(always)]
    pub fn inten0(&self) -> INTEN0_R {
        INTEN0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Interrupt enable 1."]
    #[inline(always)]
    pub fn inten1(&self) -> INTEN1_R {
        INTEN1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Interrupt enable 2."]
    #[inline(always)]
    pub fn inten2(&self) -> INTEN2_R {
        INTEN2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt enable 3."]
    #[inline(always)]
    pub fn intrn3(&self) -> INTRN3_R {
        INTRN3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Interrupt enable 4."]
    #[inline(always)]
    pub fn inten4(&self) -> INTEN4_R {
        INTEN4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Interrupt enable 5."]
    #[inline(always)]
    pub fn inten5(&self) -> INTEN5_R {
        INTEN5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Interrupt enable 6."]
    #[inline(always)]
    pub fn inten6(&self) -> INTEN6_R {
        INTEN6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Interrupt enable 8."]
    #[inline(always)]
    pub fn inten8(&self) -> INTEN8_R {
        INTEN8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - Interrupt enable alarm."]
    #[inline(always)]
    pub fn inten10(&self) -> INTEN10_R {
        INTEN10_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt enable 0."]
    #[inline(always)]
    #[must_use]
    pub fn inten0(&mut self) -> INTEN0_W<0> {
        INTEN0_W::new(self)
    }
    #[doc = "Bit 1 - Interrupt enable 1."]
    #[inline(always)]
    #[must_use]
    pub fn inten1(&mut self) -> INTEN1_W<1> {
        INTEN1_W::new(self)
    }
    #[doc = "Bit 2 - Interrupt enable 2."]
    #[inline(always)]
    #[must_use]
    pub fn inten2(&mut self) -> INTEN2_W<2> {
        INTEN2_W::new(self)
    }
    #[doc = "Bit 3 - Interrupt enable 3."]
    #[inline(always)]
    #[must_use]
    pub fn intrn3(&mut self) -> INTRN3_W<3> {
        INTRN3_W::new(self)
    }
    #[doc = "Bit 4 - Interrupt enable 4."]
    #[inline(always)]
    #[must_use]
    pub fn inten4(&mut self) -> INTEN4_W<4> {
        INTEN4_W::new(self)
    }
    #[doc = "Bit 5 - Interrupt enable 5."]
    #[inline(always)]
    #[must_use]
    pub fn inten5(&mut self) -> INTEN5_W<5> {
        INTEN5_W::new(self)
    }
    #[doc = "Bit 6 - Interrupt enable 6."]
    #[inline(always)]
    #[must_use]
    pub fn inten6(&mut self) -> INTEN6_W<6> {
        INTEN6_W::new(self)
    }
    #[doc = "Bit 8 - Interrupt enable 8."]
    #[inline(always)]
    #[must_use]
    pub fn inten8(&mut self) -> INTEN8_W<8> {
        INTEN8_W::new(self)
    }
    #[doc = "Bit 10 - Interrupt enable alarm."]
    #[inline(always)]
    #[must_use]
    pub fn inten10(&mut self) -> INTEN10_W<10> {
        INTEN10_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC interrupt enable control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [inten](index.html) module"]
pub struct INTEN_SPEC;
impl crate::RegisterSpec for INTEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [inten::R](R) reader structure"]
impl crate::Readable for INTEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [inten::W](W) writer structure"]
impl crate::Writable for INTEN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTEN to value 0"]
impl crate::Resettable for INTEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
