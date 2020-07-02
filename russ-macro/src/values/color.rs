use super::numeric::{Number, Percentage};
use russ_internal_macro::vds;

vds! {
    <"color"> = <hex-color> | <named-color> | currentcolor | transparent
                <rgb-color> | <rgba-color> | <hsl-color> | <hsla-color> | <hwb-color> |
                <lab-color> | <lch-color> | <gray-color> |
                <color-color> | <device-cmyk-color> |
                <system-color>;
}

vds! {
    <"rgb-color"> = "rgb(" <percentage>{3} [ "/" <alpha-value> ]? ")" |
                    "rgb(" <number>{3} [ "/" <alpha-value> ]? ")";
    <alpha-value> = <number> | <percentage>;
}
