use super::tensor::TensorWrapper;



// frame!()
macro_rules! frame {


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


    };



}

//frame!(frame Df DfEnum (T, A));


frame!(frame Df DfEnum (T, A));
