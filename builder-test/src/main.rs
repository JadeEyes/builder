use builder::Builder;

#[derive(Debug)]
struct X {}

#[derive(Debug, Builder)]
struct MyStruct<T, U>
where
    T: Default,
{
    a: u32,
    b: Option<&'static str>,
    c: String,
    #[builder(required)]
    d: X,
    e: T,
    #[builder(required)]
    f: U,
}

fn main() {
    let algo: MyStruct<i32, &str> = MyStruct::builder()
        .a(42u32)
        .b("hola")
        .c("boom".to_owned())
        .d(X {})
        .e(0i32)
        .f("bonjour")
        .build();
    println!("{:#?}", algo);
}
