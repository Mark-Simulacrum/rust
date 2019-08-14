// edition:2018

#![feature(async_await)]

#![feature(arbitrary_self_types)]
#![allow(non_snake_case)]

use std::pin::Pin;

struct Struct { }

type Alias = Struct;

impl Struct {
    // Test using an alias for `Struct`:

    async fn ref_Alias(self: &mut Alias, f: &u32) -> &u32 {
        //~^ ERROR missing lifetime specifier
        //~| ERROR cannot infer an appropriate lifetime
        f
    }

    async fn box_ref_Alias(self: Box<&mut Alias>, f: &u32) -> &u32 {
        //~^ ERROR missing lifetime specifier
        //~| ERROR cannot infer an appropriate lifetime
        f
    }

    async fn pin_ref_Alias(self: Pin<&mut Alias>, f: &u32) -> &u32 {
        //~^ ERROR missing lifetime specifier
        //~| ERROR cannot infer an appropriate lifetime
        f
    }

    async fn box_box_ref_Alias(self: Box<Box<&mut Alias>>, f: &u32) -> &u32 {
        //~^ ERROR missing lifetime specifier
        //~| ERROR cannot infer an appropriate lifetime
        f
    }

    async fn box_pin_ref_Alias(self: Box<Pin<&mut Alias>>, f: &u32) -> &u32 {
        //~^ ERROR missing lifetime specifier
        //~| ERROR cannot infer an appropriate lifetime
        f
    }
}

fn main() { }
