#[doc = "Register `IE` reader"]
pub struct R(crate::R<IE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IE` writer"]
pub struct W(crate::W<IE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IE_SPEC>;
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
impl From<crate::W<IE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `C0PEIE` reader - Channel 0 package end interrupt enable."]
pub type C0PEIE_R = crate::BitReader<bool>;
#[doc = "Field `C0PEIE` writer - Channel 0 package end interrupt enable."]
pub type C0PEIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IE_SPEC, bool, O>;
#[doc = "Field `C1PEIE` reader - Channel 1 package end interrupt enable."]
pub type C1PEIE_R = crate::BitReader<bool>;
#[doc = "Field `C1PEIE` writer - Channel 1 package end interrupt enable."]
pub type C1PEIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IE_SPEC, bool, O>;
#[doc = "Field `C2PEIE` reader - Channel 2 package end interrupt enable."]
pub type C2PEIE_R = crate::BitReader<bool>;
#[doc = "Field `C2PEIE` writer - Channel 2 package end interrupt enable."]
pub type C2PEIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IE_SPEC, bool, O>;
#[doc = "Field `C3PEIE` reader - Channel 3 package end interrupt enable."]
pub type C3PEIE_R = crate::BitReader<bool>;
#[doc = "Field `C3PEIE` writer - Channel 3 package end interrupt enable."]
pub type C3PEIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IE_SPEC, bool, O>;
#[doc = "Field `C0FEIE` reader - Channel 0 frame end interrupt enable."]
pub type C0FEIE_R = crate::BitReader<bool>;
#[doc = "Field `C0FEIE` writer - Channel 0 frame end interrupt enable."]
pub type C0FEIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IE_SPEC, bool, O>;
#[doc = "Field `C1FEIE` reader - Channel 1 frame end interrupt enable."]
pub type C1FEIE_R = crate::BitReader<bool>;
#[doc = "Field `C1FEIE` writer - Channel 1 frame end interrupt enable."]
pub type C1FEIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IE_SPEC, bool, O>;
#[doc = "Field `C2FEIE` reader - Channel 2 frame end interrupt enable."]
pub type C2FEIE_R = crate::BitReader<bool>;
#[doc = "Field `C2FEIE` writer - Channel 2 frame end interrupt enable."]
pub type C2FEIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IE_SPEC, bool, O>;
#[doc = "Field `C3FEIE` reader - Channel 3 frame end interrupt enable."]
pub type C3FEIE_R = crate::BitReader<bool>;
#[doc = "Field `C3FEIE` writer - Channel 3 frame end interrupt enable."]
pub type C3FEIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IE_SPEC, bool, O>;
#[doc = "Field `C0DAIE` reader - Channel 0 data abort interrupt enable"]
pub type C0DAIE_R = crate::BitReader<bool>;
#[doc = "Field `C0DAIE` writer - Channel 0 data abort interrupt enable"]
pub type C0DAIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IE_SPEC, bool, O>;
#[doc = "Field `C1DAIE` reader - Channel 1 data abort interrupt enable"]
pub type C1DAIE_R = crate::BitReader<bool>;
#[doc = "Field `C1DAIE` writer - Channel 1 data abort interrupt enable"]
pub type C1DAIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IE_SPEC, bool, O>;
#[doc = "Field `C2DAIE` reader - Channel 2 data abort interrupt enable"]
pub type C2DAIE_R = crate::BitReader<bool>;
#[doc = "Field `C2DAIE` writer - Channel 2 data abort interrupt enable"]
pub type C2DAIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IE_SPEC, bool, O>;
#[doc = "Field `C3DAIE` reader - Channel 3 data abort interrupt enable"]
pub type C3DAIE_R = crate::BitReader<bool>;
#[doc = "Field `C3DAIE` writer - Channel 3 data abort interrupt enable"]
pub type C3DAIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IE_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Channel 0 package end interrupt enable."]
    #[inline(always)]
    pub fn c0peie(&self) -> C0PEIE_R {
        C0PEIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel 1 package end interrupt enable."]
    #[inline(always)]
    pub fn c1peie(&self) -> C1PEIE_R {
        C1PEIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel 2 package end interrupt enable."]
    #[inline(always)]
    pub fn c2peie(&self) -> C2PEIE_R {
        C2PEIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel 3 package end interrupt enable."]
    #[inline(always)]
    pub fn c3peie(&self) -> C3PEIE_R {
        C3PEIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Channel 0 frame end interrupt enable."]
    #[inline(always)]
    pub fn c0feie(&self) -> C0FEIE_R {
        C0FEIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Channel 1 frame end interrupt enable."]
    #[inline(always)]
    pub fn c1feie(&self) -> C1FEIE_R {
        C1FEIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Channel 2 frame end interrupt enable."]
    #[inline(always)]
    pub fn c2feie(&self) -> C2FEIE_R {
        C2FEIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Channel 3 frame end interrupt enable."]
    #[inline(always)]
    pub fn c3feie(&self) -> C3FEIE_R {
        C3FEIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Channel 0 data abort interrupt enable"]
    #[inline(always)]
    pub fn c0daie(&self) -> C0DAIE_R {
        C0DAIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Channel 1 data abort interrupt enable"]
    #[inline(always)]
    pub fn c1daie(&self) -> C1DAIE_R {
        C1DAIE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Channel 2 data abort interrupt enable"]
    #[inline(always)]
    pub fn c2daie(&self) -> C2DAIE_R {
        C2DAIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Channel 3 data abort interrupt enable"]
    #[inline(always)]
    pub fn c3daie(&self) -> C3DAIE_R {
        C3DAIE_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channel 0 package end interrupt enable."]
    #[inline(always)]
    #[must_use]
    pub fn c0peie(&mut self) -> C0PEIE_W<0> {
        C0PEIE_W::new(self)
    }
    #[doc = "Bit 1 - Channel 1 package end interrupt enable."]
    #[inline(always)]
    #[must_use]
    pub fn c1peie(&mut self) -> C1PEIE_W<1> {
        C1PEIE_W::new(self)
    }
    #[doc = "Bit 2 - Channel 2 package end interrupt enable."]
    #[inline(always)]
    #[must_use]
    pub fn c2peie(&mut self) -> C2PEIE_W<2> {
        C2PEIE_W::new(self)
    }
    #[doc = "Bit 3 - Channel 3 package end interrupt enable."]
    #[inline(always)]
    #[must_use]
    pub fn c3peie(&mut self) -> C3PEIE_W<3> {
        C3PEIE_W::new(self)
    }
    #[doc = "Bit 4 - Channel 0 frame end interrupt enable."]
    #[inline(always)]
    #[must_use]
    pub fn c0feie(&mut self) -> C0FEIE_W<4> {
        C0FEIE_W::new(self)
    }
    #[doc = "Bit 5 - Channel 1 frame end interrupt enable."]
    #[inline(always)]
    #[must_use]
    pub fn c1feie(&mut self) -> C1FEIE_W<5> {
        C1FEIE_W::new(self)
    }
    #[doc = "Bit 6 - Channel 2 frame end interrupt enable."]
    #[inline(always)]
    #[must_use]
    pub fn c2feie(&mut self) -> C2FEIE_W<6> {
        C2FEIE_W::new(self)
    }
    #[doc = "Bit 7 - Channel 3 frame end interrupt enable."]
    #[inline(always)]
    #[must_use]
    pub fn c3feie(&mut self) -> C3FEIE_W<7> {
        C3FEIE_W::new(self)
    }
    #[doc = "Bit 8 - Channel 0 data abort interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn c0daie(&mut self) -> C0DAIE_W<8> {
        C0DAIE_W::new(self)
    }
    #[doc = "Bit 9 - Channel 1 data abort interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn c1daie(&mut self) -> C1DAIE_W<9> {
        C1DAIE_W::new(self)
    }
    #[doc = "Bit 10 - Channel 2 data abort interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn c2daie(&mut self) -> C2DAIE_W<10> {
        C2DAIE_W::new(self)
    }
    #[doc = "Bit 11 - Channel 3 data abort interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn c3daie(&mut self) -> C3DAIE_W<11> {
        C3DAIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA interrupt enable register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ie](index.html) module"]
pub struct IE_SPEC;
impl crate::RegisterSpec for IE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ie::R](R) reader structure"]
impl crate::Readable for IE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ie::W](W) writer structure"]
impl crate::Writable for IE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IE to value 0"]
impl crate::Resettable for IE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
