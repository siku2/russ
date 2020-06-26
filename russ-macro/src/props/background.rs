use russ_internal_macro::vds;

vds! { <"background-color"> = <color>; }

vds! {
    <"background-image"> = <bg-image>#;
    <bg-image> = <image> | none;
}

vds! {
    <"background-repeat"> = <repeat-style>#;
    <repeat-style> = repeat-x | repeat-y | [repeat | space | round | no-repeat]{1,5};
}

vds! {
    <"background-attachment"> = <attachment>#;
    <attachment> = scroll | fixed | local;
}

vds! {
    <"background-position"> = <bg-position>#;
    <bg-position> = [ left | center | right | top | bottom | <length-percentage> ] |
                    [ left | center | right | <length-percentage> ] [ top | center | bottom | <length-percentage> ] |
                    [ center | [ left | right ] <length-percentage>? ] && [ center | [ top | bottom ] <length-percentage>? ];
}

vds! {
    <"background-clip"> = <css-box>#;
}

vds! {
    <"background-origin"> = <css-box>#;
}

vds! {
    <"background-size"> = <bg-size>#;
    <bg-size> = [ <length-percentage [0,inf]> | auto ]{1,2} | cover | contain;
}

vds! {
    <"background"> = <bg-layer>#, <final-bg-layer>;
    <bg-layer> = <bg-image> || <bg-position> [ "/" <bg-size> ]? || <repeat-style> || <attachment> || <css-box> || <css-box>;
    <final-bg-layer> =  <"background-color"> || <bg-image> || <bg-position> [ "/" <bg-size> ]? || <repeat-style> || <attachment> || <css-box> || <css-box>;
}
