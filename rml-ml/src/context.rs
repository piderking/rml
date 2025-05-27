//pub trait Context {}

pub trait Context {
    fn next(&self) -> ();
}

macro_rules! context {
    ($name:ident<$ty:ty> -> [$($t:tt),+] use [$($p:path),*]) => {
        pub mod $name {
            use std::marker::PhantomData;
            use super::Context as TraitContext;
            type Default = $ty;
            $(
                use $p;
            ),*

            enum  Container <'a>{
                $($t($t<'a, Default>)),+
            }


            $(
                impl <'a> Into<Container<'a>> for $t<'a, Default> {
                    fn into(self) -> Container<'a> {
                        Container::$t(self)
                    }

                }
            )+

            

            pub struct Context <'a> {
                layer: Vec<Container<'a>>,
                phn: PhantomData<&'a f32>
            }
            impl <'a> TraitContext for Context<'a> {
                fn next(&self) -> () {
                    todo!()
                }
            }
            impl <'a> Context <'a> {
                pub fn new(v: Vec<Container<'a>>) -> Self {
                    Context { layer: v, phn: PhantomData}
                }
            }

        }
    };
}

context!(fun<f32> -> [Softmax] use [crate::layers::create::Softmax]);
/*

Context {

}

*/
