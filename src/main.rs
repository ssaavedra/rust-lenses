
#[macro_use] mod lens;
use crate::lens::Lens;
use crate::lens::compose;


#[derive(Clone, Debug)]
struct Country {
    name: String,
    flag: i64,
}

#[derive(Clone, Debug)]
struct ExternalAddress {
    address: String,
    zip: i64,
    country: Country,
}

#[derive(Clone, Debug)]
struct PersonInfo {
    first_name: String,
    last_name: String,
    personal_address: ExternalAddress,
    office_address: ExternalAddress,
}


fn main() {
    let aop = struct_lens!(PersonInfo, personal_address);
    let coa = struct_lens!(ExternalAddress, country);
    let noc = struct_lens!(Country, name);
    let aop_coa = compose(&aop, &coa);
    let aop_coa_noc = compose(&aop_coa, &noc);
    let aop_coa_noc2 = gen_lens!(PersonInfo, personal_address.country.name);

    let person = PersonInfo {
        first_name: String::from("Foo"),
        last_name: String::from("Bar"),
        personal_address: ExternalAddress {
            address: String::from("123, Fake St."),
            zip: 12345,
            country: Country { name: String::from("USA"), flag: 0 }},
        office_address: ExternalAddress {
            address: String::new(), zip: 0,
            country: Country { name: String::from("USA"), flag: 0 },
        },
    };

    let p2 = aop_coa_noc.set(&person, &String::from("United States"));
    let p3 = aop_coa_noc2.set(&person, &String::from("United States of America"));

    println!("Hello, world! {:?} {:?} {:?}", person, p2, p3);
}
