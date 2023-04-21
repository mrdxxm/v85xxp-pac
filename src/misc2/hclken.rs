#[doc = "Register `HCLKEN` reader"]
pub struct R(crate::R<HCLKEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HCLKEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HCLKEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HCLKEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HCLKEN` writer"]
pub struct W(crate::W<HCLKEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HCLKEN_SPEC>;
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
impl From<crate::W<HCLKEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HCLKEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DMA` reader - DMA Controller."]
pub type DMA_R = crate::BitReader<bool>;
#[doc = "Field `DMA` writer - DMA Controller."]
pub type DMA_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCLKEN_SPEC, bool, O>;
#[doc = "Field `GPIO` reader - GPIO Controller."]
pub type GPIO_R = crate::BitReader<bool>;
#[doc = "Field `GPIO` writer - GPIO Controller."]
pub type GPIO_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCLKEN_SPEC, bool, O>;
#[doc = "Field `LCD` reader - LCD Controller."]
pub type LCD_R = crate::BitReader<bool>;
#[doc = "Field `LCD` writer - LCD Controller."]
pub type LCD_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCLKEN_SPEC, bool, O>;
#[doc = "Field `CRYPT` reader - CRYPT Controller."]
pub type CRYPT_R = crate::BitReader<bool>;
#[doc = "Field `CRYPT` writer - CRYPT Controller."]
pub type CRYPT_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCLKEN_SPEC, bool, O>;
impl R {
    #[doc = "Bit 4 - DMA Controller."]
    #[inline(always)]
    pub fn dma(&self) -> DMA_R {
        DMA_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - GPIO Controller."]
    #[inline(always)]
    pub fn gpio(&self) -> GPIO_R {
        GPIO_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - LCD Controller."]
    #[inline(always)]
    pub fn lcd(&self) -> LCD_R {
        LCD_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - CRYPT Controller."]
    #[inline(always)]
    pub fn crypt(&self) -> CRYPT_R {
        CRYPT_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - DMA Controller."]
    #[inline(always)]
    #[must_use]
    pub fn dma(&mut self) -> DMA_W<4> {
        DMA_W::new(self)
    }
    #[doc = "Bit 5 - GPIO Controller."]
    #[inline(always)]
    #[must_use]
    pub fn gpio(&mut self) -> GPIO_W<5> {
        GPIO_W::new(self)
    }
    #[doc = "Bit 6 - LCD Controller."]
    #[inline(always)]
    #[must_use]
    pub fn lcd(&mut self) -> LCD_W<6> {
        LCD_W::new(self)
    }
    #[doc = "Bit 8 - CRYPT Controller."]
    #[inline(always)]
    #[must_use]
    pub fn crypt(&mut self) -> CRYPT_W<8> {
        CRYPT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AHB clock enable control register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hclken](index.html) module"]
pub struct HCLKEN_SPEC;
impl crate::RegisterSpec for HCLKEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hclken::R](R) reader structure"]
impl crate::Readable for HCLKEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hclken::W](W) writer structure"]
impl crate::Writable for HCLKEN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HCLKEN to value 0x01ff"]
impl crate::Resettable for HCLKEN_SPEC {
    const RESET_VALUE: Self::Ux = 0x01ff;
}
