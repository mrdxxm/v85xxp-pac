#[doc = "Register `MISC` reader"]
pub struct R(crate::R<MISC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MISC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MISC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MISC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MISC` writer"]
pub struct W(crate::W<MISC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MISC_SPEC>;
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
impl From<crate::W<MISC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MISC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TFE` reader - Transmit FIFO Empty Flag."]
pub type TFE_R = crate::BitReader<bool>;
#[doc = "Field `TNF` reader - Transmit FIFO Not Full Flag."]
pub type TNF_R = crate::BitReader<bool>;
#[doc = "Field `RNE` reader - Receive FIFO Not Empty Flag."]
pub type RNE_R = crate::BitReader<bool>;
#[doc = "Field `RFF` reader - Receive FIFO Full Flag."]
pub type RFF_R = crate::BitReader<bool>;
#[doc = "Field `BSY` reader - SPI Controller Busy Flag."]
pub type BSY_R = crate::BitReader<bool>;
#[doc = "Field `SMART` reader - SPI FIFO SMART Mode Register."]
pub type SMART_R = crate::BitReader<bool>;
#[doc = "Field `SMART` writer - SPI FIFO SMART Mode Register."]
pub type SMART_W<'a, const O: u8> = crate::BitWriter<'a, u32, MISC_SPEC, bool, O>;
#[doc = "Field `OVER` reader - SPI FIFO Over Write Mode."]
pub type OVER_R = crate::BitReader<bool>;
#[doc = "Field `OVER` writer - SPI FIFO Over Write Mode."]
pub type OVER_W<'a, const O: u8> = crate::BitWriter<'a, u32, MISC_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Transmit FIFO Empty Flag."]
    #[inline(always)]
    pub fn tfe(&self) -> TFE_R {
        TFE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit FIFO Not Full Flag."]
    #[inline(always)]
    pub fn tnf(&self) -> TNF_R {
        TNF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Receive FIFO Not Empty Flag."]
    #[inline(always)]
    pub fn rne(&self) -> RNE_R {
        RNE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Receive FIFO Full Flag."]
    #[inline(always)]
    pub fn rff(&self) -> RFF_R {
        RFF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SPI Controller Busy Flag."]
    #[inline(always)]
    pub fn bsy(&self) -> BSY_R {
        BSY_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - SPI FIFO SMART Mode Register."]
    #[inline(always)]
    pub fn smart(&self) -> SMART_R {
        SMART_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SPI FIFO Over Write Mode."]
    #[inline(always)]
    pub fn over(&self) -> OVER_R {
        OVER_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - SPI FIFO SMART Mode Register."]
    #[inline(always)]
    #[must_use]
    pub fn smart(&mut self) -> SMART_W<8> {
        SMART_W::new(self)
    }
    #[doc = "Bit 9 - SPI FIFO Over Write Mode."]
    #[inline(always)]
    #[must_use]
    pub fn over(&mut self) -> OVER_W<9> {
        OVER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI Misc. Control Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [misc](index.html) module"]
pub struct MISC_SPEC;
impl crate::RegisterSpec for MISC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [misc::R](R) reader structure"]
impl crate::Readable for MISC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [misc::W](W) writer structure"]
impl crate::Writable for MISC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MISC to value 0x03"]
impl crate::Resettable for MISC_SPEC {
    const RESET_VALUE: Self::Ux = 0x03;
}
