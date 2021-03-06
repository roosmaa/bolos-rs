use bolos::ui;

#[inline(always)]
pub fn badge_rust<'a>() -> ui::Icon<'a> {
    ui::CustomIcon{
        width: 14, height: 14, bits_per_pixel: 1,
        colors: &[0x00000000, 0x00ffffff],
        bitmap: &[0xe0, 0x01, 0xfe, 0xc1, 0xff, 0xf8, 0x7f, 0x0e, 0xdf, 0x93, 0xff, 0xe4, 0x3f, 0xfc, 0xcf, 0xbe, 0x33, 0xe7, 0xff, 0xf1, 0x3f, 0xf8, 0x07, 0x78, 0x00],
    }.into()
}

#[inline(always)]
pub fn badge_back<'a>() -> ui::Icon<'a> {
    ui::CustomIcon{
        width: 14, height: 14, bits_per_pixel: 1,
        colors: &[0x00000000, 0x00ffffff],
        bitmap: &[0xe0, 0x01, 0xfe, 0xc1, 0xfd, 0x38, 0x7f, 0x06, 0xdf, 0x81, 0xff, 0xc4, 0x7f, 0xf3, 0xff, 0xbc, 0x1f, 0xe7, 0xe7, 0xf1, 0x3f, 0xf8, 0x07, 0x78, 0x00],
    }.into()
}
