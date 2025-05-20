use crate::tensor::traits::tensor::TensorBound;

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
             pub fn add(&mut self, col: String, s: $ename<'a, $($tl),+>) -> &Self {
                self.data.iter()
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

pub enum DBT<'a,Id:crate::tensor::traits::dtype::dtype,Cash:crate::tensor::traits::dtype::dtype,Credit:crate::tensor::traits::dtype::dtype>{
    Id(crate::tensor::shape::tensor::Tensor<'a,Id>),Cash(crate::tensor::shape::tensor::Tensor<'a,Cash>),Credit(crate::tensor::shape::tensor::Tensor<'a,Credit>),
}
impl <'a,Id:crate::tensor::traits::dtype::dtype,Cash:crate::tensor::traits::dtype::dtype,Credit:crate::tensor::traits::dtype::dtype>DBT<'a,Id,Cash,Credit>{
    #[allow(non_snake_case)]
    pub fn Id(&self) -> Option<&crate::tensor::shape::tensor::Tensor<'a,Id>>{
        match self {
            DBT::Id(tensor) => Option::Some(tensor),
            _ => Option::None,
        
            }
    }
    #[allow(non_snake_case)]
    pub fn Cash(&self) -> Option<&crate::tensor::shape::tensor::Tensor<'a,Cash>>{
        match self {
            DBT::Cash(tensor) => Option::Some(tensor),
            _ => Option::None,
        
            }
    }
    #[allow(non_snake_case)]
    pub fn Credit(&self) -> Option<&crate::tensor::shape::tensor::Tensor<'a,Credit>>{
        match self {
            DBT::Credit(tensor) => Option::Some(tensor),
            _ => Option::None,
        
            }
    }
    pub fn into<T:crate::tensor::traits::dtype::dtype>(&self) -> Option<crate::tensor::shape::tensor::Tensor<'a,T>>{
        match self {
            DBT::Id(tensor) => {
                if stringify!(T)==stringify!(Id){
                    Option::Some(crate::tensor::change::DtypeChange(tensor.clone()).into())
                }else {
                    Option::None
                }
            },
            DBT::Cash(tensor) => {
                if stringify!(T)==stringify!(Cash){
                    Option::Some(crate::tensor::change::DtypeChange(tensor.clone()).into())
                }else {
                    Option::None
                }
            },
            DBT::Credit(tensor) => {
                if stringify!(T)==stringify!(Credit){
                    Option::Some(crate::tensor::change::DtypeChange(tensor.clone()).into())
                }else {
                    Option::None
                }
            }, 
            #[allow(unreachable_patterns)]
            _ => Option::None,
        
            }
    }

    }
pub struct DB<'a,Id:crate::tensor::traits::dtype::dtype,Cash:crate::tensor::traits::dtype::dtype,Credit:crate::tensor::traits::dtype::dtype, >{
    header:crate::tensor::stringtensor::StringTensor,data:Vec<DBT<'a,Id,Cash,Credit>>,
}
impl <'a,Id:crate::tensor::traits::dtype::dtype,Cash:crate::tensor::traits::dtype::dtype,Credit:crate::tensor::traits::dtype::dtype, >DB<'a,Id,Cash,Credit, >{
    pub fn len(&self) -> usize {
        self.header.len()
    }
    pub fn get(&self,s:String) -> Option<&DBT<'a,Id,Cash,Credit, >>{
        self.header.find(s).and_then(|i|self.data.get(i))
    }
    pub fn add(&mut self,col:String,s:DBT<'a,Id,Cash,Credit>) ->  &Self {
        if let Option::None = self.header.find(col) {
            self.header.get_mut(i)
        }
        self
    }

    }
impl <'a,Id:crate::tensor::traits::dtype::dtype,Cash:crate::tensor::traits::dtype::dtype,Credit:crate::tensor::traits::dtype::dtype, >std::ops::Index<String>for DB<'a,Id,Cash,Credit, >{
    type Output = DBT<'a,Id,Cash,Credit, >;
    fn index(&self,s:String) ->  &Self::Output {
        self.get(s).unwrap()
    }

    }
