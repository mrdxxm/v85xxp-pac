#[doc = "Register `ALARMCTL` reader"]
pub struct R(crate::R<ALARMCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ALARMCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ALARMCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ALARMCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ALARMCTL` writer"]
pub struct W(crate::W<ALARMCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ALARMCTL_SPEC>;
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
impl From<crate::W<ALARMCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ALARMCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ALARM_EN` reader - This register is used to control the alarm function enable."]
pub type ALARM_EN_R = crate::BitReader<bool>;
#[doc = "Field `ALARM_EN` writer - This register is used to control the alarm function enable."]
pub type ALARM_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ALARMCTL_SPEC, bool, O>;
#[doc = "Field `ALARM_INACCURATE` reader - This register is used to control alarm function accurate or not."]
pub type ALARM_INACCURATE_R = crate::BitReader<bool>;
#[doc = "Field `ALARM_INACCURATE` writer - This register is used to control alarm function accurate or not."]
pub type ALARM_INACCURATE_W<'a, const O: u8> = crate::BitWriter<'a, u32, ALARMCTL_SPEC, bool, O>;
#[doc = "Field `TIME_CNT_EN` reader - This register is used to control alarm time counter enable."]
pub type TIME_CNT_EN_R = crate::BitReader<bool>;
#[doc = "Field `TIME_CNT_EN` writer - This register is used to control alarm time counter enable."]
pub type TIME_CNT_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ALARMCTL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - This register is used to control the alarm function enable."]
    #[inline(always)]
    pub fn alarm_en(&self) -> ALARM_EN_R {
        ALARM_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - This register is used to control alarm function accurate or not."]
    #[inline(always)]
    pub fn alarm_inaccurate(&self) -> ALARM_INACCURATE_R {
        ALARM_INACCURATE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - This register is used to control alarm time counter enable."]
    #[inline(always)]
    pub fn time_cnt_en(&self) -> TIME_CNT_EN_R {
        TIME_CNT_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This register is used to control the alarm function enable."]
    #[inline(always)]
    #[must_use]
    pub fn alarm_en(&mut self) -> ALARM_EN_W<0> {
        ALARM_EN_W::new(self)
    }
    #[doc = "Bit 1 - This register is used to control alarm function accurate or not."]
    #[inline(always)]
    #[must_use]
    pub fn alarm_inaccurate(&mut self) -> ALARM_INACCURATE_W<1> {
        ALARM_INACCURATE_W::new(self)
    }
    #[doc = "Bit 2 - This register is used to control alarm time counter enable."]
    #[inline(always)]
    #[must_use]
    pub fn time_cnt_en(&mut self) -> TIME_CNT_EN_W<2> {
        TIME_CNT_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC alarm control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [alarmctl](index.html) module"]
pub struct ALARMCTL_SPEC;
impl crate::RegisterSpec for ALARMCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [alarmctl::R](R) reader structure"]
impl crate::Readable for ALARMCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [alarmctl::W](W) writer structure"]
impl crate::Writable for ALARMCTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ALARMCTL to value 0"]
impl crate::Resettable for ALARMCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
