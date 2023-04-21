#[doc = "Register `CTL` reader"]
pub struct R(crate::R<CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTL` writer"]
pub struct W(crate::W<CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTL_SPEC>;
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
impl From<crate::W<CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RTCPLLOE` reader - RTCPLL Divider output enable."]
pub type RTCPLLOE_R = crate::BitReader<bool>;
#[doc = "Field `RTCPLLOE` writer - RTCPLL Divider output enable."]
pub type RTCPLLOE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL_SPEC, bool, O>;
#[doc = "Field `RTCPLLCLKSEL` reader - RTC_PLLDIV clock select."]
pub type RTCPLLCLKSEL_R = crate::BitReader<bool>;
#[doc = "Field `RTCPLLCLKSEL` writer - RTC_PLLDIV clock select."]
pub type RTCPLLCLKSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 2 - RTCPLL Divider output enable."]
    #[inline(always)]
    pub fn rtcplloe(&self) -> RTCPLLOE_R {
        RTCPLLOE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - RTC_PLLDIV clock select."]
    #[inline(always)]
    pub fn rtcpllclksel(&self) -> RTCPLLCLKSEL_R {
        RTCPLLCLKSEL_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - RTCPLL Divider output enable."]
    #[inline(always)]
    #[must_use]
    pub fn rtcplloe(&mut self) -> RTCPLLOE_W<2> {
        RTCPLLOE_W::new(self)
    }
    #[doc = "Bit 4 - RTC_PLLDIV clock select."]
    #[inline(always)]
    #[must_use]
    pub fn rtcpllclksel(&mut self) -> RTCPLLCLKSEL_W<4> {
        RTCPLLCLKSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC PLL divider control register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctl](index.html) module"]
pub struct CTL_SPEC;
impl crate::RegisterSpec for CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctl::R](R) reader structure"]
impl crate::Readable for CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctl::W](W) writer structure"]
impl crate::Writable for CTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTL to value 0"]
impl crate::Resettable for CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
