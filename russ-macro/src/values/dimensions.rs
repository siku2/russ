use russ_internal_macro::vds;

// Ways to solve "5em" issue:
//  - 5 em
//  - 5'em

vds! {
    <length-percentage> = <length> | <percentage>;
    <frequency-percentage> = <frequency> | <percentage>;
    <angle-percentage> = <angle> | <percentage>;
    <time-percentage> = <time> | <percentage>;
}
