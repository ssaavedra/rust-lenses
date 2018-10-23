extern crate lenses;
use lenses::mutable::*;

#[derive(Copy, Clone, Debug)]
struct Country<'a> {
    name: &'a str,
    flag: i64,
}

#[derive(Copy, Clone, Debug)]
struct ExternalAddress<'a> {
    address: &'a str,
    zip: i64,
    country: Country<'a>,
}

#[test]
pub fn test1() {
    let mut a = ExternalAddress {
        address: "test",
        zip: 0,
        country: Country { name: "USA", flag: 0 }
    };

    let mut _c = Country { name: "USA", flag: 0 };
    let lens_cn = mod_lens(&|x: &mut Country| &mut x.name);
    let lens_ac = mod_lens(&|x: &mut ExternalAddress| &mut x.country);
    let lens_acn = mod_lens(&|x: &mut ExternalAddress| &mut x.country.name);
    let lens_acn2 = mod_compose(&lens_ac, &lens_cn); // mod_lens(&closure);
    *lens_acn.view(&mut a) = "UNI2";
    // println!("Lens {:?}\n", a);
    *lens_acn2.view(&mut a) = "UNI3";
    // *lens_acn2.view(&mut a) = "UNI4"; // Cannot call a second time!
    println!("Lens {:?}\n", a);
    // */
}

