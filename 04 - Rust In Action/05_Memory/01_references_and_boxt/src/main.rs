use std::mem::size_of;

static B: [u8; 10] = [99, 97, 114, 114, 121, 116, 111, 119, 101, 108];
static C: [u8; 11] = [116, 104, 97, 110, 107, 115, 102, 105, 115, 104, 0];

fn main() {
    /* usize is the memory address size for the CPU */
    let a: usize = 42;

    let b: &[u8; 10] = &B;

    /* Ownership of C moves to c */
    let c: Box<[u8]> = Box::new(C);

    println!(
        r#"
	a (an unsigned integer):
		location: 	{:p}
		size:		{:?} bytes
		value:		{:?}
    "#,
        &a,
        size_of::<usize>(),
        a
    );

    println!(
        r#"
	b (a reference to B):
		location: 	{:p}
		size:		{:?} bytes
		points to:	{:p}
    "#,
        &b,
        size_of::<&[u8; 10]>(),
        b
    );

    println!(
        r#"
	c (a "box" for C):
		location: 	{:p}
		size:		{:?} bytes
		points to:	{:p}
    "#,
        &c,
        size_of::<Box<[u8]>>(),
        c
    );

    println!(
        r#"
	B (an array of 10 bytes):
		location: 	{:p}
		size:		{:?} bytes
		points to:	{:?}
    "#,
        &B,
        size_of::<&[u8; 10]>(),
        B
    );

    println!(
        r#"
	C (an array of 11 bytes):
		location: 	{:p}
		size:		{:?} bytes
		points to:	{:?}
    "#,
        &C,
        size_of::<&[u8; 11]>(),
        C
    );
}
