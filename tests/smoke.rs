use wnewtype::NewType;

struct Bar<T>(T);

#[derive(NewType)]
struct Foo<T>(#[inner] Bar<T>, ());

#[test]
fn smoke() {
    let bar = Bar(5usize);
    let foo: Foo<usize> = bar.into();
    let _unwrapped = foo.into_inner();
}

#[derive(NewType)]
struct Bar2<T> {
    #[inner]
    foo: Foo<T>,
    _other: (),
}

#[test]
fn smoke2() {
    let bar = Bar(5usize);
    let foo: Foo<usize> = bar.into();
    let foo: Bar2<usize> = foo.into();
    let _unwrapped = foo.into_inner();
}
