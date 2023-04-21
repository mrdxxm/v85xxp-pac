#[doc = "Register `REG3` reader"]
pub struct R(crate::R<REG3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<REG3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<REG3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<REG3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `REG3` writer"]
pub struct W(crate::W<REG3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<REG3_SPEC>;
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
impl From<crate::W<REG3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<REG3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMP1PDN` reader - CMP1 power up control signal."]
pub type CMP1PDN_R = crate::BitReader<bool>;
#[doc = "Field `CMP1PDN` writer - CMP1 power up control signal."]
pub type CMP1PDN_W<'a, const O: u8> = crate::BitWriter<'a, u32, REG3_SPEC, bool, O>;
#[doc = "Field `CMP2PDN` reader - CMP2 power up control signal."]
pub type CMP2PDN_R = crate::BitReader<bool>;
#[doc = "Field `CMP2PDN` writer - CMP2 power up control signal."]
pub type CMP2PDN_W<'a, const O: u8> = crate::BitWriter<'a, u32, REG3_SPEC, bool, O>;
#[doc = "Field `BGPPD` reader - BGP power down control signal."]
pub type BGPPD_R = crate::BitReader<bool>;
#[doc = "Field `BGPPD` writer - BGP power down control signal."]
pub type BGPPD_W<'a, const O: u8> = crate::BitWriter<'a, u32, REG3_SPEC, bool, O>;
#[doc = "Field `RCHPD` reader - RCH (6.5536M RC) power down control signal."]
pub type RCHPD_R = crate::BitReader<bool>;
#[doc = "Field `RCHPD` writer - RCH (6.5536M RC) power down control signal."]
pub type RCHPD_W<'a, const O: u8> = crate::BitWriter<'a, u32, REG3_SPEC, bool, O>;
#[doc = "Field `PLLLPDN` reader - PLLL (32768Hz input PLL) power up control signal."]
pub type PLLLPDN_R = crate::BitReader<bool>;
#[doc = "Field `PLLLPDN` writer - PLLL (32768Hz input PLL) power up control signal."]
pub type PLLLPDN_W<'a, const O: u8> = crate::BitWriter<'a, u32, REG3_SPEC, bool, O>;
#[doc = "Field `PLLHPDN` reader - PLLH (6.5536MHz input PLL) power up control signal."]
pub type PLLHPDN_R = crate::BitReader<bool>;
#[doc = "Field `PLLHPDN` writer - PLLH (6.5536MHz input PLL) power up control signal."]
pub type PLLHPDN_W<'a, const O: u8> = crate::BitWriter<'a, u32, REG3_SPEC, bool, O>;
#[doc = "Field `XOHPDN` reader - Turn on signal of 6.5536M crystal"]
pub type XOHPDN_R = crate::BitReader<bool>;
#[doc = "Field `XOHPDN` writer - Turn on signal of 6.5536M crystal"]
pub type XOHPDN_W<'a, const O: u8> = crate::BitWriter<'a, u32, REG3_SPEC, bool, O>;
impl R {
    #[doc = "Bit 1 - CMP1 power up control signal."]
    #[inline(always)]
    pub fn cmp1pdn(&self) -> CMP1PDN_R {
        CMP1PDN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CMP2 power up control signal."]
    #[inline(always)]
    pub fn cmp2pdn(&self) -> CMP2PDN_R {
        CMP2PDN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - BGP power down control signal."]
    #[inline(always)]
    pub fn bgppd(&self) -> BGPPD_R {
        BGPPD_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - RCH (6.5536M RC) power down control signal."]
    #[inline(always)]
    pub fn rchpd(&self) -> RCHPD_R {
        RCHPD_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PLLL (32768Hz input PLL) power up control signal."]
    #[inline(always)]
    pub fn plllpdn(&self) -> PLLLPDN_R {
        PLLLPDN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PLLH (6.5536MHz input PLL) power up control signal."]
    #[inline(always)]
    pub fn pllhpdn(&self) -> PLLHPDN_R {
        PLLHPDN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Turn on signal of 6.5536M crystal"]
    #[inline(always)]
    pub fn xohpdn(&self) -> XOHPDN_R {
        XOHPDN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - CMP1 power up control signal."]
    #[inline(always)]
    #[must_use]
    pub fn cmp1pdn(&mut self) -> CMP1PDN_W<1> {
        CMP1PDN_W::new(self)
    }
    #[doc = "Bit 2 - CMP2 power up control signal."]
    #[inline(always)]
    #[must_use]
    pub fn cmp2pdn(&mut self) -> CMP2PDN_W<2> {
        CMP2PDN_W::new(self)
    }
    #[doc = "Bit 3 - BGP power down control signal."]
    #[inline(always)]
    #[must_use]
    pub fn bgppd(&mut self) -> BGPPD_W<3> {
        BGPPD_W::new(self)
    }
    #[doc = "Bit 4 - RCH (6.5536M RC) power down control signal."]
    #[inline(always)]
    #[must_use]
    pub fn rchpd(&mut self) -> RCHPD_W<4> {
        RCHPD_W::new(self)
    }
    #[doc = "Bit 5 - PLLL (32768Hz input PLL) power up control signal."]
    #[inline(always)]
    #[must_use]
    pub fn plllpdn(&mut self) -> PLLLPDN_W<5> {
        PLLLPDN_W::new(self)
    }
    #[doc = "Bit 6 - PLLH (6.5536MHz input PLL) power up control signal."]
    #[inline(always)]
    #[must_use]
    pub fn pllhpdn(&mut self) -> PLLHPDN_W<6> {
        PLLHPDN_W::new(self)
    }
    #[doc = "Bit 7 - Turn on signal of 6.5536M crystal"]
    #[inline(always)]
    #[must_use]
    pub fn xohpdn(&mut self) -> XOHPDN_W<7> {
        XOHPDN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Analog register 3.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reg3](index.html) module"]
pub struct REG3_SPEC;
impl crate::RegisterSpec for REG3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [reg3::R](R) reader structure"]
impl crate::Readable for REG3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [reg3::W](W) writer structure"]
impl crate::Writable for REG3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets REG3 to value 0"]
impl crate::Resettable for REG3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
