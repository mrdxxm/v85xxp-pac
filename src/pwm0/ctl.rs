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
#[doc = "Field `IFG` reader - Interrupt status flag of PWM timer."]
pub type IFG_R = crate::BitReader<bool>;
#[doc = "Field `IFG` writer - Interrupt status flag of PWM timer."]
pub type IFG_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, CTL_SPEC, bool, O>;
#[doc = "Field `IE` reader - Interrupt enable register of PWM timer x."]
pub type IE_R = crate::BitReader<bool>;
#[doc = "Field `IE` writer - Interrupt enable register of PWM timer x."]
pub type IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL_SPEC, bool, O>;
#[doc = "Field `CLR` reader - TAR clear register"]
pub type CLR_R = crate::BitReader<bool>;
#[doc = "Field `CLR` writer - TAR clear register"]
pub type CLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL_SPEC, bool, O>;
#[doc = "Field `TSEL` reader - Clock source selection."]
pub type TSEL_R = crate::BitReader<bool>;
#[doc = "Field `TSEL` writer - Clock source selection."]
pub type TSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL_SPEC, bool, O>;
#[doc = "Field `MC` reader - PWM Timer mode control."]
pub type MC_R = crate::FieldReader<u8, MC_A>;
#[doc = "PWM Timer mode control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MC_A {
    #[doc = "0: `0`"]
    TIMER_STOP = 0,
    #[doc = "1: `1`"]
    UP_COUNT_MODE = 1,
    #[doc = "2: `10`"]
    CONTINUOUS_MODE = 2,
    #[doc = "3: `11`"]
    UP_DOWN_MODE = 3,
}
impl From<MC_A> for u8 {
    #[inline(always)]
    fn from(variant: MC_A) -> Self {
        variant as _
    }
}
impl MC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MC_A {
        match self.bits {
            0 => MC_A::TIMER_STOP,
            1 => MC_A::UP_COUNT_MODE,
            2 => MC_A::CONTINUOUS_MODE,
            3 => MC_A::UP_DOWN_MODE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TIMER_STOP`"]
    #[inline(always)]
    pub fn is_timer_stop(&self) -> bool {
        *self == MC_A::TIMER_STOP
    }
    #[doc = "Checks if the value of the field is `UP_COUNT_MODE`"]
    #[inline(always)]
    pub fn is_up_count_mode(&self) -> bool {
        *self == MC_A::UP_COUNT_MODE
    }
    #[doc = "Checks if the value of the field is `CONTINUOUS_MODE`"]
    #[inline(always)]
    pub fn is_continuous_mode(&self) -> bool {
        *self == MC_A::CONTINUOUS_MODE
    }
    #[doc = "Checks if the value of the field is `UP_DOWN_MODE`"]
    #[inline(always)]
    pub fn is_up_down_mode(&self) -> bool {
        *self == MC_A::UP_DOWN_MODE
    }
}
#[doc = "Field `MC` writer - PWM Timer mode control."]
pub type MC_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CTL_SPEC, u8, MC_A, 2, O>;
impl<'a, const O: u8> MC_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn timer_stop(self) -> &'a mut W {
        self.variant(MC_A::TIMER_STOP)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn up_count_mode(self) -> &'a mut W {
        self.variant(MC_A::UP_COUNT_MODE)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn continuous_mode(self) -> &'a mut W {
        self.variant(MC_A::CONTINUOUS_MODE)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn up_down_mode(self) -> &'a mut W {
        self.variant(MC_A::UP_DOWN_MODE)
    }
}
#[doc = "Field `ID` reader - Input clock divider control for PWM timer x."]
pub type ID_R = crate::FieldReader<u8, ID_A>;
#[doc = "Input clock divider control for PWM timer x.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ID_A {
    #[doc = "0: `0`"]
    DIV2 = 0,
    #[doc = "1: `1`"]
    DIV4 = 1,
    #[doc = "2: `10`"]
    DIV8 = 2,
    #[doc = "3: `11`"]
    DIV16 = 3,
}
impl From<ID_A> for u8 {
    #[inline(always)]
    fn from(variant: ID_A) -> Self {
        variant as _
    }
}
impl ID_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ID_A {
        match self.bits {
            0 => ID_A::DIV2,
            1 => ID_A::DIV4,
            2 => ID_A::DIV8,
            3 => ID_A::DIV16,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == ID_A::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == ID_A::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == ID_A::DIV8
    }
    #[doc = "Checks if the value of the field is `DIV16`"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == ID_A::DIV16
    }
}
#[doc = "Field `ID` writer - Input clock divider control for PWM timer x."]
pub type ID_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CTL_SPEC, u8, ID_A, 2, O>;
impl<'a, const O: u8> ID_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(ID_A::DIV2)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(ID_A::DIV4)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(ID_A::DIV8)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(ID_A::DIV16)
    }
}
impl R {
    #[doc = "Bit 0 - Interrupt status flag of PWM timer."]
    #[inline(always)]
    pub fn ifg(&self) -> IFG_R {
        IFG_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Interrupt enable register of PWM timer x."]
    #[inline(always)]
    pub fn ie(&self) -> IE_R {
        IE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TAR clear register"]
    #[inline(always)]
    pub fn clr(&self) -> CLR_R {
        CLR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Clock source selection."]
    #[inline(always)]
    pub fn tsel(&self) -> TSEL_R {
        TSEL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - PWM Timer mode control."]
    #[inline(always)]
    pub fn mc(&self) -> MC_R {
        MC_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Input clock divider control for PWM timer x."]
    #[inline(always)]
    pub fn id(&self) -> ID_R {
        ID_R::new(((self.bits >> 6) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt status flag of PWM timer."]
    #[inline(always)]
    #[must_use]
    pub fn ifg(&mut self) -> IFG_W<0> {
        IFG_W::new(self)
    }
    #[doc = "Bit 1 - Interrupt enable register of PWM timer x."]
    #[inline(always)]
    #[must_use]
    pub fn ie(&mut self) -> IE_W<1> {
        IE_W::new(self)
    }
    #[doc = "Bit 2 - TAR clear register"]
    #[inline(always)]
    #[must_use]
    pub fn clr(&mut self) -> CLR_W<2> {
        CLR_W::new(self)
    }
    #[doc = "Bit 3 - Clock source selection."]
    #[inline(always)]
    #[must_use]
    pub fn tsel(&mut self) -> TSEL_W<3> {
        TSEL_W::new(self)
    }
    #[doc = "Bits 4:5 - PWM Timer mode control."]
    #[inline(always)]
    #[must_use]
    pub fn mc(&mut self) -> MC_W<4> {
        MC_W::new(self)
    }
    #[doc = "Bits 6:7 - Input clock divider control for PWM timer x."]
    #[inline(always)]
    #[must_use]
    pub fn id(&mut self) -> ID_W<6> {
        ID_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control register of PWM Timer 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctl](index.html) module"]
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
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x01;
}
#[doc = "`reset()` method sets CTL to value 0"]
impl crate::Resettable for CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
