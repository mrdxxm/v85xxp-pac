#[doc = "Register `ADCSTATE` reader"]
pub struct R(crate::R<ADCSTATE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADCSTATE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADCSTATE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADCSTATE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ADCSTATE` reader - These bits show the state of ADC controller state machine."]
pub type ADCSTATE_R = crate::FieldReader<u8, ADCSTATE_A>;
#[doc = "These bits show the state of ADC controller state machine.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ADCSTATE_A {
    #[doc = "0: `0`"]
    IDLE = 0,
    #[doc = "2: `10`"]
    AUTO_TRIGGER = 2,
    #[doc = "4: `100`"]
    MANUALLY_TRIGGER = 4,
}
impl From<ADCSTATE_A> for u8 {
    #[inline(always)]
    fn from(variant: ADCSTATE_A) -> Self {
        variant as _
    }
}
impl ADCSTATE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ADCSTATE_A> {
        match self.bits {
            0 => Some(ADCSTATE_A::IDLE),
            2 => Some(ADCSTATE_A::AUTO_TRIGGER),
            4 => Some(ADCSTATE_A::MANUALLY_TRIGGER),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `IDLE`"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == ADCSTATE_A::IDLE
    }
    #[doc = "Checks if the value of the field is `AUTO_TRIGGER`"]
    #[inline(always)]
    pub fn is_auto_trigger(&self) -> bool {
        *self == ADCSTATE_A::AUTO_TRIGGER
    }
    #[doc = "Checks if the value of the field is `MANUALLY_TRIGGER`"]
    #[inline(always)]
    pub fn is_manually_trigger(&self) -> bool {
        *self == ADCSTATE_A::MANUALLY_TRIGGER
    }
}
#[doc = "Field `ADC_EN` reader - This bit shows the status of ADC_EN in the ADC interface."]
pub type ADC_EN_R = crate::BitReader<bool>;
#[doc = "Field `RESET` reader - This bit shows the status of RESET in the ADC interface."]
pub type RESET_R = crate::BitReader<bool>;
#[doc = "Field `CAL_EN` reader - This bit shows the status of CAL_EN in the ADC interface."]
pub type CAL_EN_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:2 - These bits show the state of ADC controller state machine."]
    #[inline(always)]
    pub fn adcstate(&self) -> ADCSTATE_R {
        ADCSTATE_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - This bit shows the status of ADC_EN in the ADC interface."]
    #[inline(always)]
    pub fn adc_en(&self) -> ADC_EN_R {
        ADC_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - This bit shows the status of RESET in the ADC interface."]
    #[inline(always)]
    pub fn reset(&self) -> RESET_R {
        RESET_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - This bit shows the status of CAL_EN in the ADC interface."]
    #[inline(always)]
    pub fn cal_en(&self) -> CAL_EN_R {
        CAL_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
}
#[doc = "ADC State register.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adcstate](index.html) module"]
pub struct ADCSTATE_SPEC;
impl crate::RegisterSpec for ADCSTATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adcstate::R](R) reader structure"]
impl crate::Readable for ADCSTATE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ADCSTATE to value 0"]
impl crate::Resettable for ADCSTATE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
