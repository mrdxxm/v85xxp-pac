#[doc = "Register `FB[%s]` reader"]
pub struct R(crate::R<FB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FB[%s]` writer"]
pub struct W(crate::W<FB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FB_SPEC>;
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
impl From<crate::W<FB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATA` reader - Each bit represents a data in the display array."]
pub type DATA_R = crate::FieldReader<u32, u32>;
#[doc = "Field `DATA` writer - Each bit represents a data in the display array."]
pub type DATA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FB_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Each bit represents a data in the display array."]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Each bit represents a data in the display array."]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DATA_W<0> {
        DATA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LCD Frame buffer x register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fb](index.html) module"]
pub struct FB_SPEC;
impl crate::RegisterSpec for FB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fb::R](R) reader structure"]
impl crate::Readable for FB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fb::W](W) writer structure"]
impl crate::Writable for FB_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
