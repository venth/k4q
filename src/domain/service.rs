use shaku::Interface;

pub trait App: Interface {
    fn run<'a>(&self, args: &'a Vec<&'a str>);
}
