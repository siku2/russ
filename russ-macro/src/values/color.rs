use super::numeric::{Number, Percentage};
use russ_internal_macro::vds;

// TODO vds needs to support grouping using () natively.

vds! {
    <"color"> = <hex-color> | <named-color> | currentcolor | transparent |
                <rgb-fn> | <hsl-fn> | <hwb-fn> |
                <lab-fn> | <lch-fn> | <gray-fn> |
                <color-fn> | <device-cmyk-fn> |
                <system-color>;

    // common values
    <alpha-value> = <number> | <percentage>;
    <hue> = <number> | <angle>;

    <rgb-fn> = "rgb"( <percentage>{3} [ "/" <alpha-value> ]? ) |
               "rgb"( <number>{3} [ "/" <alpha-value> ]? );
    <hsl-fn> = "hsl"( <hue> <percentage> <percentage> [ "/" <alpha-value> ]? );
    <hwb-fn> = "hwb"( <hue> <percentage> <percentage> [ "/" <alpha-value> ]? );

    <lab-fn> = "lab"( <percentage> <number> <number> [ "/" <alpha-value> ]? );
    <lch-fn> = "lch"( <percentage> <number> <hue> [ "/" <alpha-value> ]? );
    <gray-fn> = "gray"( <number>  [ "/" <alpha-value> ]? );

    <color-fn> = "color"( [ <css-ident>? [ <number>+ | <css-string> ] [ "/" <alpha-value> ]? ]# , <color>? );
    <device-cmyk-fn> = "device-cmyk"( <cmyk-component>{4} [ "/" <alpha-value> ]? , <color>? );
    <cmyk-component> = <number> | <percentage>;
}
