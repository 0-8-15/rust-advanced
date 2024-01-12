   let mut x = XX {a:2, b:4};
   doit(x)();

struct XX {
    a: usize,
    b: usize
}


fn doit (mut x: XX) -> impl FnMut() -> usize {

    return move || {  x.b=13 ; x.a + 25}
}
