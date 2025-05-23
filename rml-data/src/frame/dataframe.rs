use crate::tensor::traits::dtype::dtype;

pub trait FrameTyped {}

pub trait DataFrame {
    type Typed: FrameTyped;

    fn len(&self) -> usize;
    fn get(&self, s: &String) -> Option<&Self::Typed>;
    fn get_mut(&mut self, s: &String) -> Option<&mut Self::Typed>;
    fn push<T: dtype>(
        &mut self,
        s: String,
        item: T,
    ) -> Result<&mut Self, crate::tensor::error::DataFrameError>;
    fn extend<T: dtype>(
        &mut self,
        s: &String,
        item: Vec<T>,
    ) -> Result<&mut Self, crate::tensor::error::DataFrameError>;
    // fn extend <T: dtype>  (&mut self, )
}

// frame!()
#[allow(unused)]
#[macro_export]
macro_rules! frame {


    (frame $name:ident ($($tl: tt),+)) => {

        pub mod $name {

        use super::{DataFrame, FrameTyped};
        use crate::tensor::traits::tensor::TensorBound;
        use std::cell::RefCell;
        use std::cell::RefMut;
        use crate::tensor::traits::dtype::dtype;


        use std::rc::Rc;

        pub enum Typed<'a, $($tl: crate::tensor::traits::dtype::dtype),+> {
            // TODO Make into RefCell
            $($tl(Rc<RefCell<crate::tensor::shape::tensor::Tensor<'a, $tl>>>),)+
        }

        impl <'a, $($tl: crate::tensor::traits::dtype::dtype),+> FrameTyped for Typed <'a, $($tl),+>{

        }


        // Invoke Macro Above
        impl <'a, $($tl: crate::tensor::traits::dtype::dtype),+> Typed <'a, $($tl),+>{
            pub fn to_string(&self) -> String {
                match self {
                        $(Typed::$tl(tensor) => {
                            format!("{}", tensor.borrow())
                        },)+
                        #[allow(unreachable_patterns)] // Safety
                        _ => panic!("Critical Error!"),
                    }
            }
            $(
                #[allow(non_snake_case)]
                pub fn $tl (&self) -> Option<crate::tensor::shape::tensor::Tensor<'a, $tl>> {
                    match self {
                        Typed::$tl(tensor) => Option::Some(tensor.clone().borrow().clone()),
                        _ => Option::None,
                    }
                }
            )+

            pub fn to <T: crate::tensor::traits::dtype::dtype> (&self) -> Option<crate::tensor::shape::tensor::Tensor<'a, T>>{
                match self {
                        $(Typed::$tl(tensor) => {
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
                        $(Typed::$tl(tensor) => {


                            RefMut::map(tensor.borrow_mut(), |f|{
                                    // satisfy type checker
                                    f.push(T::to::<$tl>(i));
                                    f
                                });
                                Option::Some(self)
                        },)+
                        #[allow(unreachable_patterns)] // Safety
                        _ => Option::None,
                    }
            }

            pub fn extend <T: crate::tensor::traits::dtype::dtype> (&mut self, i: Vec<T>) -> Option<&mut Self>{
                match self {
                        $(Typed::$tl(tensor) => {
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

        pub struct Item {}


        /// THIS IS EPIC
        pub struct Frame<'a, $($tl:crate::tensor::traits::dtype::dtype,)+> {
            header: crate::tensor::stringtensor::StringTensor,
            data: Vec<Typed<'a, $($tl),+>>,
        }

        impl<'a, $($tl:crate::tensor::traits::dtype::dtype,)+> Frame<'a, $($tl,)+> {

            #[allow(non_snake_case)]
            #[allow(unused)]
            fn new($($tl: crate::tensor::shape::tensor::Tensor<'a, $tl>,)+) -> Self {
                Self {
                    data: vec![
                        $( Typed::$tl(Rc::new(RefCell::new($tl))) ,)+
                    ],
                    header: crate::tensor::stringtensor::StringTensor::from(vec![$(stringify!($tl).to_string(),)+])
                }
            }

            #[allow(unused)]
            pub fn empty() -> Self {
                Self {
                    data: vec![
                        $( Typed::$tl(Rc::new(RefCell::new(crate::tensor::shape::tensor::Tensor::<'a, $tl>::empty()))) ,)+
                    ],
                    header: crate::tensor::stringtensor::StringTensor::from(vec![$(stringify!($tl).to_string(),)+])
                }
            }
        }

        impl<'a, $($tl:crate::tensor::traits::dtype::dtype,)+> DataFrame for Frame<'a, $($tl,)+> {
            type Typed = Typed <'a, $($tl),+>;
            fn len(&self) -> usize {
                self.header.len() // header dictates size
            }
            fn get(&self, s: &String) -> Option<&Self::Typed> {
                self.header.find(s.clone()).and_then(|i| self.data.get(i))
            }
            fn get_mut(&mut self, s: &String) -> Option<&mut Self::Typed> {
                self.header.find(s.clone()).and_then(|i| self.data.get_mut(i))
            }

            // TODO make collumn container for
            fn push <T: crate::tensor::traits::dtype::dtype> (&mut self, s: String, item: T) -> Result<&mut Self, crate::tensor::error::DataFrameError> {

                match self.get_mut(&s) {
                    Option::Some(ten) => {
                        if let Option::None = ten.push(item) {
                            // push having error here
                            Err(crate::tensor::error::DataFrameError::Unknown)
                        } else {
                            let t = self.header.into_iter().filter(|f|*f!=&s).map(|f|f.clone()).collect::<Vec<_>>();
                            t.iter().for_each(|l|
                                match self.get_mut(&l).unwrap() {
                                    $(Typed::$tl(tensor) => {

                                        RefMut::map(tensor.borrow_mut(), |tp| {
                                            tp.push($tl::default());
                                            tp
                                        });
                                    }),+
                                    #[allow(unreachable_patterns)] // Safety
                                    _ => panic!("Critical Error!"),
                                }
                            );
                            Ok(self)
                        }

                    },
                    Option::None => Err(crate::tensor::error::DataFrameError::UnknownCol{c1:s})
                }



            }
            fn extend <T: crate::tensor::traits::dtype::dtype> (&mut self, s: &String, item: Vec<T>) -> Result<&mut Self, crate::tensor::error::DataFrameError> {

                match self.get_mut(s) {
                    Option::Some(ten) => {
                        if let Option::None = ten.extend(item) {
                            Err(crate::tensor::error::DataFrameError::Unknown)
                        } else {
                            // Extend the rest.....
                            Ok(self)
                        }

                    },
                    Option::None => Err(crate::tensor::error::DataFrameError::UnknownCol{c1:s.clone()})
                }



            }




        }

        impl <'a, $($tl:crate::tensor::traits::dtype::dtype,)+> std::ops::Index<String> for Frame<'a, $($tl,)+>{
            type Output = Typed<'a, $($tl,)+>;
            fn index(&self, s: String) -> &Self::Output {
                self.get(&s).unwrap()
            }
        }

        impl <'a, $($tl:crate::tensor::traits::dtype::dtype,)+> std::fmt::Display for Frame<'a, $($tl,)+>{
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.data.iter().enumerate().map(|(i, f)|format!("{}:{}", self.header.get(i).unwrap(), f.to_string())).collect::<Vec<_>>().join("\n"))
            }
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
        frame!(frame database (Name, Age, Size));

        let mut t: database::Frame<'_, TString, f32, i32> = database::Frame::empty();

        let _ = t.push("Name".to_string(), tstring!("Hi"));
        let _ = t.push("Age".to_string(), 0.111);
        let _ = t.push("Age".to_string(), 0.11121);

        print!("{}", t);
    }
}
