/*use super::tensor::TensorWrapper;

// frame!()
macro_rules! frame {
    /*
     // Implement Into<Option<Tensor<>>> for each enum variant
        $(
            impl<'a> Into<Option<crate::tensor::shape::tensor::Tensor<'a, $tl>>> for $ename<'a, $($tl),+>
            where
                $tl: crate::tensor::traits::dtype::dtype,
            {
                fn into(self) -> Option<crate::tensor::shape::tensor::Tensor<'a, $tl>> {
                    match self {
                        $ename::$tl(tensor) => Some(tensor),
                        _ => None,
                    }
                }
            }
        )+


      (into, $name:ident, $tp:tt, ($($tl:tt,)+)) => {
        impl <'a, $($tl: crate::tensor::traits::dtype::dtype),+> Into<Option<crate::tensor::shape::tensor::Tensor<'a, $tp>>> for $name<'a, $($tl,)+> {
            fn into(self) -> Option<crate::tensor::shape::tensor::Tensor<'a, $tp>>{
                 match self {
                        $name::$tp(tensor) => Some(tensor),
                        _ => None,
                    }
            }
        }
    };
     */


    (frame $name:ident $ename:ident ($($tl: tt),+)) => {


        pub enum $ename<'a, $($tl: crate::tensor::traits::dtype::dtype),+> {
            $($tl(crate::tensor::shape::tensor::Tensor<'a, $tl>),)+
        }

        // Invoke Macro Above




        pub struct $name<'a, $($tl:crate::tensor::traits::dtype::dtype,)+> {
            header: crate::tensor::stringtensor::StringTensor,
            data: Vec<$ename<'a, $($tl),+>>,
        }


        impl<'a, $($tl:crate::tensor::traits::dtype::dtype,)+> $name<'a, $($tl,)+> {
            pub fn len(&self) -> usize {
                self.header.len() // header dictates size
            }
            pub fn get(&self, s: String) -> Option<&$ename<'a, $($tl,)+>> {
                self.header.find(s).and_then(|i| self.data.get(i))
            }

        }

        impl <'a, $($tl:crate::tensor::traits::dtype::dtype,)+> std::ops::Index<String> for $name<'a, $($tl,)+>{
            type Output = $ename<'a, $($tl,)+>;
            fn index(&self, s: String) -> &Self::Output {
                self.get(s).unwrap()
            }
        }





    };



}

//frame!(frame Df DfEnum (T, A));

frame!(frame Df DfEnum (T, A));
 */

use proc_macro::TokenStream;
use proc_macro2::TokenStream as Tokens;
use quote::{quote, ToTokens, TokenStreamExt};
use syn::{Data, DeriveInput, Fields, Ident, Type};

pub(crate) fn impl_df_creator(input: &DeriveInput) -> Tokens {
    // Extract the struct name
    let name = &input.ident;


    // Generate the method to count fields

    // Convert the generated code into a TokenStream
    let df = DF::new(
        name.clone(),
        match &input.data {
        Data::Struct(data_struct) => data_struct.fields.iter().filter_map(|f| match &f.ident {
            Some(i) => Option::Some(DFFeilds {
                name: i.clone(),
                ty: f.ty.clone(),
            }),
            None => todo!(),
        }).collect(),
        _ => panic!("Cannot use Enum!"),
    });
    //TokenStream::from(input)
    quote![t]
}

struct DF {
    // Names
    name: Ident,
    ename: Ident,
    // Feilds
    feilds: Vec<DFFeilds>,
}
impl DF {
    pub fn new(name: Ident, feilds: Vec<DFFeilds>) -> Self {
        Self {
            ename: Ident::new(&format! {"{}Typed", &name}, name.span()),
            name: name,
            feilds: feilds,
        }
    }
}

struct DFFeilds {
    name: Ident,
    ty: Type,
}

