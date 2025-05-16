use super::tensor::TensorWrapper;

// frame!()
macro_rules! frame {
    (frame $name:ident $ename:ident ($($tl: tt),+)) => {
        use crate::tensor::{
            shape::tensor::Tensor, stringtensor::StringTensor, traits::dtype::dtype,
        };

        pub enum $ename<'a, $($tl: dtype),+> {
            $($tl(Tensor<'a, $tl>),)+
        }
        pub struct $name<'a, $($tl:dtype,)+> {
            header: crate::tensor::stringtensor::StringTensor,
            data: Vec<$ename<'a, $($tl),+>>,
        }

        impl<'a, $($tl:dtype,)+> Df<'a, $($tl,)+> {
            pub fn len(&self) -> usize {
                self.header.len() // header dictates size
            }
            pub fn get(&self, s: String) -> Option<&$ename<'a, $($tl,)+>> {
                self.header.find(s).and_then(|i| self.data.get(i))
            }
        }

        impl <'a, $($tl:dtype,)+> std::ops::Index<String> for $name<'a, $($tl,)+>{
            type Output = $ename<'a, $($tl,)+>;
            fn index(&self, s: String) -> &Self::Output {
                self.get(s).unwrap()
            }
        }



    };


}

//frame!(frame Df DfEnum (T));

frame!(frame Df DfEnum (T, A));
