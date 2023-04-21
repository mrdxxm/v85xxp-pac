#[doc = "Register `REG6` reader"]
pub struct R(crate::R<REG6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<REG6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<REG6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<REG6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `REG6` writer"]
pub struct W(crate::W<REG6_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<REG6_SPEC>;
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
impl From<crate::W<REG6_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<REG6_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LCDBMODE` reader - LCD BIAS mode selection."]
pub type LCDBMODE_R = crate::BitReader<bool>;
#[doc = "Field `LCDBMODE` writer - LCD BIAS mode selection."]
pub type LCDBMODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, REG6_SPEC, bool, O>;
#[doc = "Field `VLCD` reader - LCD driving voltage."]
pub type VLCD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VLCD` writer - LCD driving voltage."]
pub type VLCD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, REG6_SPEC, u8, u8, 4, O>;
#[doc = "Field `BAT1DISC` reader - Discharge the BAT1 battery."]
pub type BAT1DISC_R = crate::BitReader<bool>;
#[doc = "Field `BAT1DISC` writer - Discharge the BAT1 battery."]
pub type BAT1DISC_W<'a, const O: u8> = crate::BitWriter<'a, u32, REG6_SPEC, bool, O>;
#[doc = "Field `BATRTCDISC` reader - Discharge the BATRTC battery."]
pub type BATRTCDISC_R = crate::BitReader<bool>;
#[doc = "Field `BATRTCDISC` writer - Discharge the BATRTC battery."]
pub type BATRTCDISC_W<'a, const O: u8> = crate::BitWriter<'a, u32, REG6_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - LCD BIAS mode selection."]
    #[inline(always)]
    pub fn lcdbmode(&self) -> LCDBMODE_R {
        LCDBMODE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:4 - LCD driving voltage."]
    #[inline(always)]
    pub fn vlcd(&self) -> VLCD_R {
        VLCD_R::new(((self.bits >> 1) & 0x0f) as u8)
    }
    #[doc = "Bit 6 - Discharge the BAT1 battery."]
    #[inline(always)]
    pub fn bat1disc(&self) -> BAT1DISC_R {
        BAT1DISC_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Discharge the BATRTC battery."]
    #[inline(always)]
    pub fn batrtcdisc(&self) -> BATRTCDISC_R {
        BATRTCDISC_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LCD BIAS mode selection."]
    #[inline(always)]
    #[must_use]
    pub fn lcdbmode(&mut self) -> LCDBMODE_W<0> {
        LCDBMODE_W::new(self)
    }
    #[doc = "Bits 1:4 - LCD driving voltage."]
    #[inline(always)]
    #[must_use]
    pub fn vlcd(&mut self) -> VLCD_W<1> {
        VLCD_W::new(self)
    }
    #[doc = "Bit 6 - Discharge the BAT1 battery."]
    #[inline(always)]
    #[must_use]
    pub fn bat1disc(&mut self) -> BAT1DISC_W<6> {
        BAT1DISC_W::new(self)
    }
    #[doc = "Bit 7 - Discharge the BATRTC battery."]
    #[inline(always)]
    #[must_use]
    pub fn batrtcdisc(&mut self) -> BATRTCDISC_W<7> {
        BATRTCDISC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Analog register 6.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reg6](index.html) module"]
pub struct REG6_SPEC;
impl crate::RegisterSpec for REG6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [reg6::R](R) reader structure"]
impl crate::Readable for REG6_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [reg6::W](W) writer structure"]
impl crate::Writable for REG6_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets REG6 to value 0"]
impl crate::Resettable for REG6_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
