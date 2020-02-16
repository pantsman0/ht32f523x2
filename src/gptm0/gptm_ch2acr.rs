#[doc = "Reader of register GPTM_CH2ACR"]
pub type R = crate::R<u32, super::GPTM_CH2ACR>;
#[doc = "Writer for register GPTM_CH2ACR"]
pub type W = crate::W<u32, super::GPTM_CH2ACR>;
#[doc = "Register GPTM_CH2ACR `reset()`'s with value 0"]
impl crate::ResetValue for super::GPTM_CH2ACR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CH2ACV`"]
pub type CH2ACV_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CH2ACV`"]
pub struct CH2ACV_W<'a> {
    w: &'a mut W,
}
impl<'a> CH2ACV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - CH2ACV"]
    #[inline(always)]
    pub fn ch2acv(&self) -> CH2ACV_R {
        CH2ACV_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - CH2ACV"]
    #[inline(always)]
    pub fn ch2acv(&mut self) -> CH2ACV_W {
        CH2ACV_W { w: self }
    }
}
