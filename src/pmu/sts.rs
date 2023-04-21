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
#[doc = "Field `INT_32K` reader - This bit represents the 32K crystal absent interrupt status."]
pub type INT_32K_R = crate::BitReader<bool>;
#[doc = "Field `INT_32K` writer - This bit represents the 32K crystal absent interrupt status."]
pub type INT_32K_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, STS_SPEC, bool, O>;
#[doc = "Field `INT_6M` reader - This bit represents the 6.5536M crystal absent interrupt status."]
pub type INT_6M_R = crate::BitReader<bool>;
#[doc = "Field `INT_6M` writer - This bit represents the 6.5536M crystal absent interrupt status."]
pub type INT_6M_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, STS_SPEC, bool, O>;
#[doc = "Field `EXIST_32K` reader - 32K XTAL exist status register."]
pub type EXIST_32K_R = crate::BitReader<bool>;
#[doc = "Field `EXIST_6M` reader - 6.5536M XTAL exist status register."]
pub type EXIST_6M_R = crate::BitReader<bool>;
#[doc = "Field `EXTRST` reader - This bit indicated if the last interrupt is cause by external reset signal."]
pub type EXTRST_R = crate::BitReader<bool>;
#[doc = "Field `EXTRST` writer - This bit indicated if the last interrupt is cause by external reset signal."]
pub type EXTRST_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, STS_SPEC, bool, O>;
#[doc = "Field `PORST` reader - This bit indicated if the last reset is cause by internal power-on reset signal."]
pub type PORST_R = crate::BitReader<bool>;
#[doc = "Field `PORST` writer - This bit indicated if the last reset is cause by internal power-on reset signal."]
pub type PORST_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, STS_SPEC, bool, O>;
#[doc = "Field `DPORST` reader - This bit indicated if the last reset is cause by internal digital power-on reset signal."]
pub type DPORST_R = crate::BitReader<bool>;
#[doc = "Field `DPORST` writer - This bit indicated if the last reset is cause by internal digital power-on reset signal."]
pub type DPORST_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, STS_SPEC, bool, O>;
#[doc = "Field `WDTRST` reader - This bit indicated if the last reset is caused by WDT."]
pub type WDTRST_R = crate::BitReader<bool>;
#[doc = "Field `WDTRST` writer - This bit indicated if the last reset is caused by WDT."]
pub type WDTRST_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, STS_SPEC, bool, O>;
#[doc = "Field `SFTRST` reader - This register indicates that a soft reset has happened."]
pub type SFTRST_R = crate::BitReader<bool>;
#[doc = "Field `SFTRST` writer - This register indicates that a soft reset has happened."]
pub type SFTRST_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, STS_SPEC, bool, O>;
#[doc = "Field `MODERST` reader - This register indicates that a MODE wakeup reset from sleep or deep sleep mode has happened."]
pub type MODERST_R = crate::BitReader<bool>;
#[doc = "Field `MODERST` writer - This register indicates that a MODE wakeup reset from sleep or deep sleep mode has happened."]
pub type MODERST_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, STS_SPEC, bool, O>;
#[doc = "Field `WKUIOA` reader - This register is used to indicate if the previous wake-up source from deep-sleep mode is from IOA wake-up."]
pub type WKUIOA_R = crate::BitReader<bool>;
#[doc = "Field `WKURTC` reader - This register is used to indicate if the previous wake-up source from deep-sleep mode is from RTC module."]
pub type WKURTC_R = crate::BitReader<bool>;
#[doc = "Field `WKUANA` reader - This register is used to indicate if the previous wake-up source from deep-sleep mode is from Analog module."]
pub type WKUANA_R = crate::BitReader<bool>;
#[doc = "Field `WKUU32K` reader - This register is used to indicate if the previous wake-up source from deep-sleep mode is from UART 32K module."]
pub type WKUU32K_R = crate::BitReader<bool>;
#[doc = "Field `WKUXTAL` reader - This register is used to indicate if the previous wake-up source from deep-sleep mode is from crystal absent detection module."]
pub type WKUXTAL_R = crate::BitReader<bool>;
#[doc = "Field `WKUMODE` reader - This register is used to indicate if the previous wake-up source from deep-sleep mode is from MODE pin change from 1 to 0."]
pub type WKUMODE_R = crate::BitReader<bool>;
#[doc = "Field `MODE` reader - This register shows the current status of MODE pin."]
pub type MODE_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - This bit represents the 32K crystal absent interrupt status."]
    #[inline(always)]
    pub fn int_32k(&self) -> INT_32K_R {
        INT_32K_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - This bit represents the 6.5536M crystal absent interrupt status."]
    #[inline(always)]
    pub fn int_6m(&self) -> INT_6M_R {
        INT_6M_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 32K XTAL exist status register."]
    #[inline(always)]
    pub fn exist_32k(&self) -> EXIST_32K_R {
        EXIST_32K_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 6.5536M XTAL exist status register."]
    #[inline(always)]
    pub fn exist_6m(&self) -> EXIST_6M_R {
        EXIST_6M_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - This bit indicated if the last interrupt is cause by external reset signal."]
    #[inline(always)]
    pub fn extrst(&self) -> EXTRST_R {
        EXTRST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - This bit indicated if the last reset is cause by internal power-on reset signal."]
    #[inline(always)]
    pub fn porst(&self) -> PORST_R {
        PORST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - This bit indicated if the last reset is cause by internal digital power-on reset signal."]
    #[inline(always)]
    pub fn dporst(&self) -> DPORST_R {
        DPORST_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - This bit indicated if the last reset is caused by WDT."]
    #[inline(always)]
    pub fn wdtrst(&self) -> WDTRST_R {
        WDTRST_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - This register indicates that a soft reset has happened."]
    #[inline(always)]
    pub fn sftrst(&self) -> SFTRST_R {
        SFTRST_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - This register indicates that a MODE wakeup reset from sleep or deep sleep mode has happened."]
    #[inline(always)]
    pub fn moderst(&self) -> MODERST_R {
        MODERST_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 16 - This register is used to indicate if the previous wake-up source from deep-sleep mode is from IOA wake-up."]
    #[inline(always)]
    pub fn wkuioa(&self) -> WKUIOA_R {
        WKUIOA_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - This register is used to indicate if the previous wake-up source from deep-sleep mode is from RTC module."]
    #[inline(always)]
    pub fn wkurtc(&self) -> WKURTC_R {
        WKURTC_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - This register is used to indicate if the previous wake-up source from deep-sleep mode is from Analog module."]
    #[inline(always)]
    pub fn wkuana(&self) -> WKUANA_R {
        WKUANA_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - This register is used to indicate if the previous wake-up source from deep-sleep mode is from UART 32K module."]
    #[inline(always)]
    pub fn wkuu32k(&self) -> WKUU32K_R {
        WKUU32K_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - This register is used to indicate if the previous wake-up source from deep-sleep mode is from crystal absent detection module."]
    #[inline(always)]
    pub fn wkuxtal(&self) -> WKUXTAL_R {
        WKUXTAL_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 22 - This register is used to indicate if the previous wake-up source from deep-sleep mode is from MODE pin change from 1 to 0."]
    #[inline(always)]
    pub fn wkumode(&self) -> WKUMODE_R {
        WKUMODE_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 24 - This register shows the current status of MODE pin."]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This bit represents the 32K crystal absent interrupt status."]
    #[inline(always)]
    #[must_use]
    pub fn int_32k(&mut self) -> INT_32K_W<0> {
        INT_32K_W::new(self)
    }
    #[doc = "Bit 1 - This bit represents the 6.5536M crystal absent interrupt status."]
    #[inline(always)]
    #[must_use]
    pub fn int_6m(&mut self) -> INT_6M_W<1> {
        INT_6M_W::new(self)
    }
    #[doc = "Bit 4 - This bit indicated if the last interrupt is cause by external reset signal."]
    #[inline(always)]
    #[must_use]
    pub fn extrst(&mut self) -> EXTRST_W<4> {
        EXTRST_W::new(self)
    }
    #[doc = "Bit 5 - This bit indicated if the last reset is cause by internal power-on reset signal."]
    #[inline(always)]
    #[must_use]
    pub fn porst(&mut self) -> PORST_W<5> {
        PORST_W::new(self)
    }
    #[doc = "Bit 6 - This bit indicated if the last reset is cause by internal digital power-on reset signal."]
    #[inline(always)]
    #[must_use]
    pub fn dporst(&mut self) -> DPORST_W<6> {
        DPORST_W::new(self)
    }
    #[doc = "Bit 7 - This bit indicated if the last reset is caused by WDT."]
    #[inline(always)]
    #[must_use]
    pub fn wdtrst(&mut self) -> WDTRST_W<7> {
        WDTRST_W::new(self)
    }
    #[doc = "Bit 8 - This register indicates that a soft reset has happened."]
    #[inline(always)]
    #[must_use]
    pub fn sftrst(&mut self) -> SFTRST_W<8> {
        SFTRST_W::new(self)
    }
    #[doc = "Bit 10 - This register indicates that a MODE wakeup reset from sleep or deep sleep mode has happened."]
    #[inline(always)]
    #[must_use]
    pub fn moderst(&mut self) -> MODERST_W<10> {
        MODERST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PMU Status register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sts](index.html) module"]
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
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x05f3;
}
#[doc = "`reset()` method sets STS to value 0x7c"]
impl crate::Resettable for STS_SPEC {
    const RESET_VALUE: Self::Ux = 0x7c;
}
