use super::tensor::TensorWrapper;


// frame!()
macro_rules! frame {


    
    
    (impl $name:ident ($($tl: tt),+) {$($it:item)*}) => {
        impl <'a, $($tl: crate::tensor::traits::dtype::dtype),+> $name <'a, $($tl),+> {
            $(it)*
        }
    };
    (frame $name:ident $ename:ident ($($tl: tt),+) ) => { //


        pub enum $ename<'a, $($tl: crate::tensor::traits::dtype::dtype),+> {
            $($tl(crate::tensor::shape::tensor::Tensor<'a, $tl>),)+
        }
        
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

            pub fn into <T: crate::tensor::traits::dtype::dtype> (&self) -> Option<crate::tensor::shape::tensor::Tensor<'a, T>>{
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
             pub fn add(&self, s: $ename<'a, $($tl),+>) -> &Self {
                
                self
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

frame!(frame DB DBT (Id, Cash, Credit));
