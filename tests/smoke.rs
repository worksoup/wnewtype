use newtype::NewType;

struct Bar<T>(T);

#[derive(NewType)]
struct Foo<T>(Bar<T>);