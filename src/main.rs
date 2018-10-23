
#[macro_use] mod lens;
use crate::lens::Lens;
use crate::lens::compose;


#[derive(Clone, Copy, Debug)]
struct Country<'a> {
    name: &'a str,
    flag: i64,
}

#[derive(Clone, Copy, Debug)]
struct ExternalAddress<'a> {
    address: &'a str,
    zip: i64,
    country: Country<'a>,
}

#[derive(Clone, Copy, Debug)]
struct PersonInfo<'a> {
    first_name: &'a str,
    last_name: &'a str,
    personal_address: ExternalAddress<'a>,
    office_address: ExternalAddress<'a>,
}


fn main() {
    let aop = struct_lens!(PersonInfo, personal_address);
    let coa = struct_lens!(ExternalAddress, country);
    let noc = struct_lens!(Country, name);
    let aop_coa = compose(&aop, &coa);
    let aop_coa_noc = compose(&aop_coa, &noc);
    let aop_coa_noc2 = gen_lens!(PersonInfo, personal_address.country.name);

    let person = PersonInfo {
        first_name: "Foo",
        last_name: "Bar",
        personal_address: ExternalAddress {
            address: "123, Fake St.",
            zip: 12345,
            country: Country { name: "USA", flag: 0 }},
        office_address: ExternalAddress {
            address: "", zip: 0,
            country: Country { name: "USA", flag: 0 },
        },
    };

    let p2 = aop_coa_noc.set(&person, &"United States");
    let p3 = aop_coa_noc2.set(&person, &"United States of America");

    println!("Hello, world! {:?} {:?} {:?}", person, p2, p3);
}
