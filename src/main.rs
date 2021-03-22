#![allow(dead_code)]

/*
 *
 * Check This Post
 * https://deterministic.space/impl-a-trait-for-str-slices-and-slices-of-strs.html
 *
 */

trait ToFoo {
    fn to_foo(&self) -> Vec<String>;
}

impl ToFoo for str {
    fn to_foo(&self) -> Vec<String> {
        // First ALternative
        //let mut args = Vec::new();
        //self.split(' ').for_each(|ch| args.push(ch.to_owned()));
        //args

        // Second Alternative
        self.split(' ').map(|ch| ch.to_owned()).collect()
    }
}

impl<'a, T> ToFoo for T
where
    T: AsRef<[&'a str]>,
{
    fn to_foo(&self) -> Vec<String> {
        self.as_ref().into_iter().map(|x| x.to_string()).collect()
    }
}

fn foo<'a, T: ToFoo + ?Sized>(data: &'a T) {
    println!("{:?}", data.to_foo());
}

fn main() {
    foo("la la pa");
    foo(&["la", "la", "pa"]);
}
