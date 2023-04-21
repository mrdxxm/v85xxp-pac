#[doc = "Register `STS` reader"]
pub struct R(crate::R<STS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STS` writer"]
pub struct W(crate::W<STS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STS_SPEC>;
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
impl From<crate::W<STS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `C0BUSY` reader - DMA channel 0 busy register."]
pub type C0BUSY_R = crate::BitReader<bool>;
#[doc = "Field `C1BUSY` reader - DMA channel 1 busy register."]
pub type C1BUSY_R = crate::BitReader<bool>;
#[doc = "Field `C2BUSY` reader - DMA channel 2 busy register."]
pub type C2BUSY_R = crate::BitReader<bool>;
#[doc = "Field `C3BUSY` reader - DMA channel 3 busy register."]
pub type C3BUSY_R = crate::BitReader<bool>;
#[doc = "Field `C0PE` reader - Channel 0 package end interrupt flag."]
pub type C0PE_R = crate::BitReader<bool>;
#[doc = "Field `C0PE` writer - Channel 0 package end interrupt flag."]
pub type C0PE_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, STS_SPEC, bool, O>;
#[doc = "Field `C1PE` reader - Channel 1 package end interrupt flag."]
pub type C1PE_R = crate::BitReader<bool>;
#[doc = "Field `C1PE` writer - Channel 1 package end interrupt flag."]
pub type C1PE_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, STS_SPEC, bool, O>;
#[doc = "Field `C2PE` reader - Channel 2 package end interrupt flag."]
pub type C2PE_R = crate::BitReader<bool>;
#[doc = "Field `C2PE` writer - Channel 2 package end interrupt flag."]
pub type C2PE_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, STS_SPEC, bool, O>;
#[doc = "Field `C3PE` reader - Channel 3 package end interrupt flag."]
pub type C3PE_R = crate::BitReader<bool>;
#[doc = "Field `C3PE` writer - Channel 3 package end interrupt flag."]
pub type C3PE_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, STS_SPEC, bool, O>;
#[doc = "Field `C0FE` reader - Channel 0 frame end interrupt flag."]
pub type C0FE_R = crate::BitReader<bool>;
#[doc = "Field `C0FE` writer - Channel 0 frame end interrupt flag."]
pub type C0FE_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, STS_SPEC, bool, O>;
#[doc = "Field `C1FE` reader - Channel 1 frame end interrupt flag."]
pub type C1FE_R = crate::BitReader<bool>;
#[doc = "Field `C1FE` writer - Channel 1 frame end interrupt flag."]
pub type C1FE_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, STS_SPEC, bool, O>;
#[doc = "Field `C2FE` reader - Channel 2 frame end interrupt flag."]
pub type C2FE_R = crate::BitReader<bool>;
#[doc = "Field `C2FE` writer - Channel 2 frame end interrupt flag."]
pub type C2FE_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, STS_SPEC, bool, O>;
#[doc = "Field `C3FE` reader - Channel 3 frame end interrupt flag."]
pub type C3FE_R = crate::BitReader<bool>;
#[doc = "Field `C3FE` writer - Channel 3 frame end interrupt flag."]
pub type C3FE_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, STS_SPEC, bool, O>;
#[doc = "Field `C0DA` reader - Channel 0 data abort interrupt flag."]
pub type C0DA_R = crate::BitReader<bool>;
#[doc = "Field `C0DA` writer - Channel 0 data abort interrupt flag."]
pub type C0DA_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, STS_SPEC, bool, O>;
#[doc = "Field `C1DA` reader - Channel 1 data abort interrupt flag."]
pub type C1DA_R = crate::BitReader<bool>;
#[doc = "Field `C1DA` writer - Channel 1 data abort interrupt flag."]
pub type C1DA_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, STS_SPEC, bool, O>;
#[doc = "Field `C2DA` reader - Channel 2 data abort interrupt flag."]
pub type C2DA_R = crate::BitReader<bool>;
#[doc = "Field `C2DA` writer - Channel 2 data abort interrupt flag."]
pub type C2DA_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, STS_SPEC, bool, O>;
#[doc = "Field `C3DA` reader - Channel 3 data abort interrupt flag."]
pub type C3DA_R = crate::BitReader<bool>;
#[doc = "Field `C3DA` writer - Channel 3 data abort interrupt flag."]
pub type C3DA_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, STS_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - DMA channel 0 busy register."]
    #[inline(always)]
    pub fn c0busy(&self) -> C0BUSY_R {
        C0BUSY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMA channel 1 busy register."]
    #[inline(always)]
    pub fn c1busy(&self) -> C1BUSY_R {
        C1BUSY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DMA channel 2 busy register."]
    #[inline(always)]
    pub fn c2busy(&self) -> C2BUSY_R {
        C2BUSY_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DMA channel 3 busy register."]
    #[inline(always)]
    pub fn c3busy(&self) -> C3BUSY_R {
        C3BUSY_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Channel 0 package end interrupt flag."]
    #[inline(always)]
    pub fn c0pe(&self) -> C0PE_R {
        C0PE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Channel 1 package end interrupt flag."]
    #[inline(always)]
    pub fn c1pe(&self) -> C1PE_R {
        C1PE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Channel 2 package end interrupt flag."]
    #[inline(always)]
    pub fn c2pe(&self) -> C2PE_R {
        C2PE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Channel 3 package end interrupt flag."]
    #[inline(always)]
    pub fn c3pe(&self) -> C3PE_R {
        C3PE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Channel 0 frame end interrupt flag."]
    #[inline(always)]
    pub fn c0fe(&self) -> C0FE_R {
        C0FE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Channel 1 frame end interrupt flag."]
    #[inline(always)]
    pub fn c1fe(&self) -> C1FE_R {
        C1FE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Channel 2 frame end interrupt flag."]
    #[inline(always)]
    pub fn c2fe(&self) -> C2FE_R {
        C2FE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Channel 3 frame end interrupt flag."]
    #[inline(always)]
    pub fn c3fe(&self) -> C3FE_R {
        C3FE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Channel 0 data abort interrupt flag."]
    #[inline(always)]
    pub fn c0da(&self) -> C0DA_R {
        C0DA_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Channel 1 data abort interrupt flag."]
    #[inline(always)]
    pub fn c1da(&self) -> C1DA_R {
        C1DA_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Channel 2 data abort interrupt flag."]
    #[inline(always)]
    pub fn c2da(&self) -> C2DA_R {
        C2DA_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Channel 3 data abort interrupt flag."]
    #[inline(always)]
    pub fn c3da(&self) -> C3DA_R {
        C3DA_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - Channel 0 package end interrupt flag."]
    #[inline(always)]
    #[must_use]
    pub fn c0pe(&mut self) -> C0PE_W<4> {
        C0PE_W::new(self)
    }
    #[doc = "Bit 5 - Channel 1 package end interrupt flag."]
    #[inline(always)]
    #[must_use]
    pub fn c1pe(&mut self) -> C1PE_W<5> {
        C1PE_W::new(self)
    }
    #[doc = "Bit 6 - Channel 2 package end interrupt flag."]
    #[inline(always)]
    #[must_use]
    pub fn c2pe(&mut self) -> C2PE_W<6> {
        C2PE_W::new(self)
    }
    #[doc = "Bit 7 - Channel 3 package end interrupt flag."]
    #[inline(always)]
    #[must_use]
    pub fn c3pe(&mut self) -> C3PE_W<7> {
        C3PE_W::new(self)
    }
    #[doc = "Bit 8 - Channel 0 frame end interrupt flag."]
    #[inline(always)]
    #[must_use]
    pub fn c0fe(&mut self) -> C0FE_W<8> {
        C0FE_W::new(self)
    }
    #[doc = "Bit 9 - Channel 1 frame end interrupt flag."]
    #[inline(always)]
    #[must_use]
    pub fn c1fe(&mut self) -> C1FE_W<9> {
        C1FE_W::new(self)
    }
    #[doc = "Bit 10 - Channel 2 frame end interrupt flag."]
    #[inline(always)]
    #[must_use]
    pub fn c2fe(&mut self) -> C2FE_W<10> {
        C2FE_W::new(self)
    }
    #[doc = "Bit 11 - Channel 3 frame end interrupt flag."]
    #[inline(always)]
    #[must_use]
    pub fn c3fe(&mut self) -> C3FE_W<11> {
        C3FE_W::new(self)
    }
    #[doc = "Bit 12 - Channel 0 data abort interrupt flag."]
    #[inline(always)]
    #[must_use]
    pub fn c0da(&mut self) -> C0DA_W<12> {
        C0DA_W::new(self)
    }
    #[doc = "Bit 13 - Channel 1 data abort interrupt flag."]
    #[inline(always)]
    #[must_use]
    pub fn c1da(&mut self) -> C1DA_W<13> {
        C1DA_W::new(self)
    }
    #[doc = "Bit 14 - Channel 2 data abort interrupt flag."]
    #[inline(always)]
    #[must_use]
    pub fn c2da(&mut self) -> C2DA_W<14> {
        C2DA_W::new(self)
    }
    #[doc = "Bit 15 - Channel 3 data abort interrupt flag."]
    #[inline(always)]
    #[must_use]
    pub fn c3da(&mut self) -> C3DA_W<15> {
        C3DA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA status register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sts](index.html) module"]
pub struct STS_SPEC;
impl crate::RegisterSpec for STS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sts::R](R) reader structure"]
impl crate::Readable for STS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sts::W](W) writer structure"]
impl crate::Writable for STS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0xfff0;
}
#[doc = "`reset()` method sets STS to value 0"]
impl crate::Resettable for STS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
