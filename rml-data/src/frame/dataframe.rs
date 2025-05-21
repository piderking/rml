use crate::tensor::traits::dtype::dtype;

use super::tensor::TensorWrapper;


pub trait FrameTyped {
}

pub trait DataFrame {
    type Typed: FrameTyped;

    fn len(&self) -> usize;
    fn get(&self, s: String) -> Option<&Self::Typed>;
    fn get_mut(&mut self, s: String) -> Option<&mut Self::Typed>;
    fn push <T: dtype> (&mut self, s: String, item: T) ->Result<&mut Self, crate::tensor::error::DataFrameError>;
    // fn extend <T: dtype>  (&mut self, )


}
// frame!()
macro_rules! frame {


    (frame $name:ident $ename:ident ($($tl: tt),+)) => {

        pub enum $ename<'a, $($tl: crate::tensor::traits::dtype::dtype),+> {
            // TODO Make into RefCell
            $($tl(crate::tensor::shape::tensor::Tensor<'a, $tl>),)+
        }

        impl <'a, $($tl: crate::tensor::traits::dtype::dtype),+> FrameTyped for $ename <'a, $($tl),+>{
           

        }


        // Invoke Macro Above
        impl <'a, $($tl: crate::tensor::traits::dtype::dtype),+> $ename <'a, $($tl),+>{
            $(
                #[allow(non_snake_case)]
                pub fn $tl (&self) -> Option<&crate::tensor::shape::tensor::Tensor<'a, $tl>> {
                    match self {
                        $ename::$tl(tensor) => Option::Some(tensor),
                        _ => Option::None,
                    }
                }
            )+

            pub fn to <T: crate::tensor::traits::dtype::dtype> (&self) -> Option<crate::tensor::shape::tensor::Tensor<'a, T>>{
                match self {
                        $($ename::$tl(tensor) => {
                            if stringify!(T) == stringify!($tl){
                                Option::Some(crate::tensor::change::DtypeChange(tensor.clone()).into())
                            } else {
                                Option::None
                            }
                        },)+
                        #[allow(unreachable_patterns)] // Safety
                        _ => Option::None,
                    }
            }
            pub fn push <T: crate::tensor::traits::dtype::dtype> (&mut self) -> (){
                match self {
                        $($ename::$tl(tensor) => {
                            if stringify!(T) == stringify!($tl){
                                Option::Some(crate::tensor::change::DtypeChange(tensor.clone()).into());
                            } else {
                                
                            }
                        },)+
                        #[allow(unreachable_patterns)] // Safety
                        _ => (),
                    }
            }

            
        }


        pub struct $name<'a, $($tl:crate::tensor::traits::dtype::dtype,)+> {
            header: crate::tensor::stringtensor::StringTensor,
            data: Vec<$ename<'a, $($tl),+>>,
        }


        impl<'a, $($tl:crate::tensor::traits::dtype::dtype,)+> DataFrame for $name<'a, $($tl,)+> {
            type Typed = $ename <'a, $($tl),+>;

            fn len(&self) -> usize {
                self.header.len() // header dictates size
            }
            fn get(&self, s: String) -> Option<&Self::Typed> {
                self.header.find(s).and_then(|i| self.data.get(i))
            }
            fn get_mut(&mut self, s: String) -> Option<&mut Self::Typed> {
                self.header.find(s).and_then(|i| self.data.get_mut(i))
            }
            fn push <T: crate::tensor::traits::dtype::dtype> (&mut self, s: String, item: T) -> Result<&mut Self, crate::tensor::error::DataFrameError> {
                
                match self.header.find(s.clone()) {
                    Option::Some(n) => {
                        // Extend All
                        //n.to::<T>();

                        Ok(self)
                    },
                    Option::None => {
                        Err(crate::tensor::error::DataFrameError::UnknownCol{c1: s})
                    }
                }
                
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

frame!(frame Df DfEnum (Cash, A));
