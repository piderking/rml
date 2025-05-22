use crate::tensor::traits::dtype::dtype;
use crate::tensor::traits::tensor::TensorBound;
use std::cell::RefCell;
use std::cell::RefMut;

use std::rc::Rc;
pub trait FrameTyped {}

pub trait DataFrame {
    type Typed: FrameTyped;

    fn len(&self) -> usize;
    fn get(&self, s: String) -> Option<&Self::Typed>;
    fn get_mut(&mut self, s: String) -> Option<&mut Self::Typed>;
    fn push<T: dtype>(
        &mut self,
        s: String,
        item: T,
    ) -> Result<&mut Self, crate::tensor::error::DataFrameError>;
    fn extend<T: dtype>(
        &mut self,
        s: String,
        item: Vec<T>,
    ) -> Result<&mut Self, crate::tensor::error::DataFrameError>;
    // fn extend <T: dtype>  (&mut self, )
}

// frame!()
#[allow(unused)]
#[macro_export]
macro_rules! frame {

    (frame $name:ident $ename:ident ($($tl: tt),+)) => {

        pub enum $ename<'a, $($tl: crate::tensor::traits::dtype::dtype),+> {
            // TODO Make into RefCell
            $($tl(Rc<RefCell<crate::tensor::shape::tensor::Tensor<'a, $tl>>>),)+
        }

        impl <'a, $($tl: crate::tensor::traits::dtype::dtype),+> FrameTyped for $ename <'a, $($tl),+>{


        }


        // Invoke Macro Above
        impl <'a, $($tl: crate::tensor::traits::dtype::dtype),+> $ename <'a, $($tl),+>{
            $(
                #[allow(non_snake_case)]
                pub fn $tl (&self) -> Option<crate::tensor::shape::tensor::Tensor<'a, $tl>> {
                    match self {
                        $ename::$tl(tensor) => Option::Some(tensor.clone().borrow().clone()),
                        _ => Option::None,
                    }
                }
            )+

            pub fn to <T: crate::tensor::traits::dtype::dtype> (&self) -> Option<crate::tensor::shape::tensor::Tensor<'a, T>>{
                match self {
                        $($ename::$tl(tensor) => {
                            if stringify!(T) == stringify!($tl){
                                Option::Some(crate::tensor::change::DtypeChange(tensor.clone().borrow().clone()).into())
                            } else {
                                Option::None
                            }
                        },)+
                        #[allow(unreachable_patterns)] // Safety
                        _ => Option::None,
                    }
            }
            pub fn push <T: crate::tensor::traits::dtype::dtype> (&mut self, i: T) -> Option<&mut Self>{
                match self {
                        $($ename::$tl(tensor) => {
                            if stringify!(T) == stringify!($tl){
                            RefMut::map(tensor.borrow_mut(), |f|{
                                    f.push(T::to::<$tl>(i));
                                    f
                                });
                                Option::Some(self)

                            } else {
                                Option::None
                            }
                        },)+
                        #[allow(unreachable_patterns)] // Safety
                        _ => Option::None,
                    }
            }

            pub fn extend <T: crate::tensor::traits::dtype::dtype> (&mut self, i: Vec<T>) -> Option<&mut Self>{
                match self {
                        $($ename::$tl(tensor) => {
                            if stringify!(T) == stringify!($tl){
                            RefMut::map(tensor.borrow_mut(), |f|{
                                for it in i {
                                    f.push(T::to::<$tl>(it.clone()));
                                };
                                f
                                });
                                Option::Some(self)

                            } else {
                                Option::None
                            }
                        },)+
                        #[allow(unreachable_patterns)] // Safety
                        _ => Option::None,
                    }
            }


        }


        pub struct $name<'a, $($tl:crate::tensor::traits::dtype::dtype,)+> {
            header: crate::tensor::stringtensor::StringTensor,
            data: Vec<$ename<'a, $($tl),+>>,
        }

        impl<'a, $($tl:crate::tensor::traits::dtype::dtype,)+>  $name<'a, $($tl,)+> {

            #[allow(non_snake_case)]
            #[allow(unused)]
            fn new($($tl: crate::tensor::shape::tensor::Tensor<'a, $tl>,)+) -> Self {
                Self {
                    data: vec![
                        $( $ename::$tl(Rc::new(RefCell::new($tl))) ,)+
                    ],
                    header: crate::tensor::stringtensor::StringTensor::from(vec![$(stringify!($tl).to_string(),)+])
                }
            }

            #[allow(unused)]
            fn empty() -> Self {
                Self {
                    data: vec![
                        $( $ename::$tl(Rc::new(RefCell::new(crate::tensor::shape::tensor::Tensor::<'a, $tl>::empty()))) ,)+
                    ],
                    header: crate::tensor::stringtensor::StringTensor::from(vec![$(stringify!($tl).to_string(),)+])
                }
            }
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

                match self.get_mut(s.clone()) {
                    Option::Some(ten) => {
                        if let Option::None = ten.push(item) {
                            Err(crate::tensor::error::DataFrameError::Unknown)
                        } else {
                            Ok(self)
                        }

                    },
                    Option::None => Err(crate::tensor::error::DataFrameError::UnknownCol{c1:s})
                }



            }
            fn extend <T: crate::tensor::traits::dtype::dtype> (&mut self, s: String, item: Vec<T>) -> Result<&mut Self, crate::tensor::error::DataFrameError> {

                match self.get_mut(s.clone()) {
                    Option::Some(ten) => {
                        if let Option::None = ten.extend(item) {
                            Err(crate::tensor::error::DataFrameError::Unknown)
                        } else {
                            // Extend the rest.....
                            Ok(self)
                        }

                    },
                    Option::None => Err(crate::tensor::error::DataFrameError::UnknownCol{c1:s})
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{tensor::types::tstring::TString, tstring};

    #[test]
    pub fn macro_rules() {
        frame!(frame Database DataTyped (Name, Age, Size));

        let t: Database<'_, i32, f32, TString> = Database::empty();
    }
}
