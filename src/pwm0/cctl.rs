#[doc = "Register `CCTL[%s]` reader"]
pub struct R(crate::R<CCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CCTL[%s]` writer"]
pub struct W(crate::W<CCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCTL_SPEC>;
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
impl From<crate::W<CCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CCIFG` reader - Under capture mode, this bit will be set when a capture event is coming and the TAR will be latched into CCRx. Under compare mode, this bit will be set when TAR=CCRx."]
pub type CCIFG_R = crate::BitReader<bool>;
#[doc = "Field `CCIFG` writer - Under capture mode, this bit will be set when a capture event is coming and the TAR will be latched into CCRx. Under compare mode, this bit will be set when TAR=CCRx."]
pub type CCIFG_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, CCTL_SPEC, bool, O>;
#[doc = "Field `COV` reader - Capture overflow flag."]
pub type COV_R = crate::BitReader<bool>;
#[doc = "Field `COV` writer - Capture overflow flag."]
pub type COV_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, CCTL_SPEC, bool, O>;
#[doc = "Field `OUT` reader - This bit is used to control the output value of OUTx when OUTMOD is set to 0."]
pub type OUT_R = crate::BitReader<bool>;
#[doc = "Field `OUT` writer - This bit is used to control the output value of OUTx when OUTMOD is set to 0."]
pub type OUT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCTL_SPEC, bool, O>;
#[doc = "Field `CCIE` reader - Capture/Compare interrupt enable register."]
pub type CCIE_R = crate::BitReader<bool>;
#[doc = "Field `CCIE` writer - Capture/Compare interrupt enable register."]
pub type CCIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCTL_SPEC, bool, O>;
#[doc = "Field `OUTMOD` reader - Output mode selection."]
pub type OUTMOD_R = crate::FieldReader<u8, OUTMOD_A>;
#[doc = "Output mode selection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OUTMOD_A {
    #[doc = "0: `0`"]
    CONSTANT_MODE = 0,
    #[doc = "1: `1`"]
    SET_MODE = 1,
    #[doc = "2: `10`"]
    TOGGLE_RESET_MODE = 2,
    #[doc = "3: `11`"]
    SET_RESET_MODE = 3,
    #[doc = "4: `100`"]
    TOGGLE_MODE = 4,
    #[doc = "5: `101`"]
    RESET_MODE = 5,
    #[doc = "6: `110`"]
    TOGGLE_SET_MODE = 6,
    #[doc = "7: `111`"]
    RESET_SET_MODE = 7,
}
impl From<OUTMOD_A> for u8 {
    #[inline(always)]
    fn from(variant: OUTMOD_A) -> Self {
        variant as _
    }
}
impl OUTMOD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUTMOD_A {
        match self.bits {
            0 => OUTMOD_A::CONSTANT_MODE,
            1 => OUTMOD_A::SET_MODE,
            2 => OUTMOD_A::TOGGLE_RESET_MODE,
            3 => OUTMOD_A::SET_RESET_MODE,
            4 => OUTMOD_A::TOGGLE_MODE,
            5 => OUTMOD_A::RESET_MODE,
            6 => OUTMOD_A::TOGGLE_SET_MODE,
            7 => OUTMOD_A::RESET_SET_MODE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CONSTANT_MODE`"]
    #[inline(always)]
    pub fn is_constant_mode(&self) -> bool {
        *self == OUTMOD_A::CONSTANT_MODE
    }
    #[doc = "Checks if the value of the field is `SET_MODE`"]
    #[inline(always)]
    pub fn is_set_mode(&self) -> bool {
        *self == OUTMOD_A::SET_MODE
    }
    #[doc = "Checks if the value of the field is `TOGGLE_RESET_MODE`"]
    #[inline(always)]
    pub fn is_toggle_reset_mode(&self) -> bool {
        *self == OUTMOD_A::TOGGLE_RESET_MODE
    }
    #[doc = "Checks if the value of the field is `SET_RESET_MODE`"]
    #[inline(always)]
    pub fn is_set_reset_mode(&self) -> bool {
        *self == OUTMOD_A::SET_RESET_MODE
    }
    #[doc = "Checks if the value of the field is `TOGGLE_MODE`"]
    #[inline(always)]
    pub fn is_toggle_mode(&self) -> bool {
        *self == OUTMOD_A::TOGGLE_MODE
    }
    #[doc = "Checks if the value of the field is `RESET_MODE`"]
    #[inline(always)]
    pub fn is_reset_mode(&self) -> bool {
        *self == OUTMOD_A::RESET_MODE
    }
    #[doc = "Checks if the value of the field is `TOGGLE_SET_MODE`"]
    #[inline(always)]
    pub fn is_toggle_set_mode(&self) -> bool {
        *self == OUTMOD_A::TOGGLE_SET_MODE
    }
    #[doc = "Checks if the value of the field is `RESET_SET_MODE`"]
    #[inline(always)]
    pub fn is_reset_set_mode(&self) -> bool {
        *self == OUTMOD_A::RESET_SET_MODE
    }
}
#[doc = "Field `OUTMOD` writer - Output mode selection."]
pub type OUTMOD_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CCTL_SPEC, u8, OUTMOD_A, 3, O>;
impl<'a, const O: u8> OUTMOD_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn constant_mode(self) -> &'a mut W {
        self.variant(OUTMOD_A::CONSTANT_MODE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn set_mode(self) -> &'a mut W {
        self.variant(OUTMOD_A::SET_MODE)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn toggle_reset_mode(self) -> &'a mut W {
        self.variant(OUTMOD_A::TOGGLE_RESET_MODE)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn set_reset_mode(self) -> &'a mut W {
        self.variant(OUTMOD_A::SET_RESET_MODE)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn toggle_mode(self) -> &'a mut W {
        self.variant(OUTMOD_A::TOGGLE_MODE)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn reset_mode(self) -> &'a mut W {
        self.variant(OUTMOD_A::RESET_MODE)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn toggle_set_mode(self) -> &'a mut W {
        self.variant(OUTMOD_A::TOGGLE_SET_MODE)
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn reset_set_mode(self) -> &'a mut W {
        self.variant(OUTMOD_A::RESET_SET_MODE)
    }
}
#[doc = "Field `CAP` reader - Capture/Compare mode selection."]
pub type CAP_R = crate::BitReader<bool>;
#[doc = "Field `CAP` writer - Capture/Compare mode selection."]
pub type CAP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCTL_SPEC, bool, O>;
#[doc = "Field `OUTEN` reader - OUTx output enable control register."]
pub type OUTEN_R = crate::BitReader<bool>;
#[doc = "Field `OUTEN` writer - OUTx output enable control register."]
pub type OUTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCTL_SPEC, bool, O>;
#[doc = "Field `SCCI` reader - The read only register shows the INx input value when the TAR is equal to CCRx."]
pub type SCCI_R = crate::BitReader<bool>;
#[doc = "Field `CM` reader - Capture edge selection."]
pub type CM_R = crate::FieldReader<u8, CM_A>;
#[doc = "Capture edge selection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CM_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    RISING = 1,
    #[doc = "2: `10`"]
    FALLING = 2,
    #[doc = "3: `11`"]
    BOTH = 3,
}
impl From<CM_A> for u8 {
    #[inline(always)]
    fn from(variant: CM_A) -> Self {
        variant as _
    }
}
impl CM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CM_A {
        match self.bits {
            0 => CM_A::DISABLE,
            1 => CM_A::RISING,
            2 => CM_A::FALLING,
            3 => CM_A::BOTH,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CM_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `RISING`"]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == CM_A::RISING
    }
    #[doc = "Checks if the value of the field is `FALLING`"]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == CM_A::FALLING
    }
    #[doc = "Checks if the value of the field is `BOTH`"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == CM_A::BOTH
    }
}
#[doc = "Field `CM` writer - Capture edge selection."]
pub type CM_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CCTL_SPEC, u8, CM_A, 2, O>;
impl<'a, const O: u8> CM_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CM_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn rising(self) -> &'a mut W {
        self.variant(CM_A::RISING)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn falling(self) -> &'a mut W {
        self.variant(CM_A::FALLING)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn both(self) -> &'a mut W {
        self.variant(CM_A::BOTH)
    }
}
impl R {
    #[doc = "Bit 0 - Under capture mode, this bit will be set when a capture event is coming and the TAR will be latched into CCRx. Under compare mode, this bit will be set when TAR=CCRx."]
    #[inline(always)]
    pub fn ccifg(&self) -> CCIFG_R {
        CCIFG_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Capture overflow flag."]
    #[inline(always)]
    pub fn cov(&self) -> COV_R {
        COV_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - This bit is used to control the output value of OUTx when OUTMOD is set to 0."]
    #[inline(always)]
    pub fn out(&self) -> OUT_R {
        OUT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Capture/Compare interrupt enable register."]
    #[inline(always)]
    pub fn ccie(&self) -> CCIE_R {
        CCIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:7 - Output mode selection."]
    #[inline(always)]
    pub fn outmod(&self) -> OUTMOD_R {
        OUTMOD_R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bit 8 - Capture/Compare mode selection."]
    #[inline(always)]
    pub fn cap(&self) -> CAP_R {
        CAP_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - OUTx output enable control register."]
    #[inline(always)]
    pub fn outen(&self) -> OUTEN_R {
        OUTEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - The read only register shows the INx input value when the TAR is equal to CCRx."]
    #[inline(always)]
    pub fn scci(&self) -> SCCI_R {
        SCCI_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 14:15 - Capture edge selection."]
    #[inline(always)]
    pub fn cm(&self) -> CM_R {
        CM_R::new(((self.bits >> 14) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Under capture mode, this bit will be set when a capture event is coming and the TAR will be latched into CCRx. Under compare mode, this bit will be set when TAR=CCRx."]
    #[inline(always)]
    #[must_use]
    pub fn ccifg(&mut self) -> CCIFG_W<0> {
        CCIFG_W::new(self)
    }
    #[doc = "Bit 1 - Capture overflow flag."]
    #[inline(always)]
    #[must_use]
    pub fn cov(&mut self) -> COV_W<1> {
        COV_W::new(self)
    }
    #[doc = "Bit 2 - This bit is used to control the output value of OUTx when OUTMOD is set to 0."]
    #[inline(always)]
    #[must_use]
    pub fn out(&mut self) -> OUT_W<2> {
        OUT_W::new(self)
    }
    #[doc = "Bit 4 - Capture/Compare interrupt enable register."]
    #[inline(always)]
    #[must_use]
    pub fn ccie(&mut self) -> CCIE_W<4> {
        CCIE_W::new(self)
    }
    #[doc = "Bits 5:7 - Output mode selection."]
    #[inline(always)]
    #[must_use]
    pub fn outmod(&mut self) -> OUTMOD_W<5> {
        OUTMOD_W::new(self)
    }
    #[doc = "Bit 8 - Capture/Compare mode selection."]
    #[inline(always)]
    #[must_use]
    pub fn cap(&mut self) -> CAP_W<8> {
        CAP_W::new(self)
    }
    #[doc = "Bit 9 - OUTx output enable control register."]
    #[inline(always)]
    #[must_use]
    pub fn outen(&mut self) -> OUTEN_W<9> {
        OUTEN_W::new(self)
    }
    #[doc = "Bits 14:15 - Capture edge selection."]
    #[inline(always)]
    #[must_use]
    pub fn cm(&mut self) -> CM_W<14> {
        CM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Compare/capture control register x(x=0~3) for PWM timer x.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cctl](index.html) module"]
pub struct CCTL_SPEC;
impl crate::RegisterSpec for CCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cctl::R](R) reader structure"]
impl crate::Readable for CCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cctl::W](W) writer structure"]
impl crate::Writable for CCTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x03;
}
#[doc = "`reset()` method sets CCTL[%s]
to value 0"]
impl crate::Resettable for CCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
