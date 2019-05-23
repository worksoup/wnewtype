use newtype::NewType;

struct Bar<T>(T);

#[derive(NewType)]
struct Foo<T>(Bar<T>);

#[test]
fn smoke() {
    let bar = Bar(5usize);
    let foo: Foo<usize> = bar.into();
    let _unwrapped = foo.into_inner();
}