#[macro_use] extern crate lenses;

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

#[derive(Copy, Clone, Debug)]
struct PersonInfo<'a> {
    first_name: &'a str,
    last_name: &'a str,
    personal_address: ExternalAddress<'a>,
    office_address: ExternalAddress<'a>,
}

fn mystr<'a> (v: &'a str) -> &'a str { v }

#[test]
fn main() {
    use lenses::lens::Lens;
    use lenses::lens::compose;

    let aop = struct_lens!(clone PersonInfo, personal_address);
    let coa = struct_lens!(clone ExternalAddress, country);
    let noc = struct_lens!(clone Country, name);
    let aop_coa = compose(&aop, &coa);
    let aop_coa_noc = compose(&aop_coa, &noc);
    let aop_coa_noc2 = gen_lens!(copy PersonInfo, personal_address.country.name);

    let person = PersonInfo {
        first_name: mystr("Foo"),
        last_name: mystr("Bar"),
        personal_address: ExternalAddress {
            address: mystr("123, Fake St."),
            zip: 12345,
            country: Country { name: mystr("USA"), flag: 0 }},
        office_address: ExternalAddress {
            address: mystr(""), zip: 0,
            country: Country { name: mystr("USA"), flag: 0 },
        },
    };

    let p2 = aop_coa_noc.set(&person, &("United States"));
    let p3 = aop_coa_noc2.set(&person, &("United States of America"));

    println!("Result of lense changes:\n{:?}\n{:?}\n{:?}\n\n", person, p2, p3);

    // Example of explicit Getter syntax
    println!("Hello!: {:?}\n", lenses::lens::Getter::view(&aop_coa, &person));

    {   use lenses::lens::Getter; // Example of implicit syntax
        println!("Hello!: {:?}\n", aop_coa.view(&person));
    }
}
