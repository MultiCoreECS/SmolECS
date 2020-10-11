use SmolCommonMacros::{impl_joinable, impl_joinable_multi};
pub trait Joinable<'w> {
    type Target;    
    fn join(self) -> JoinIter<'w, Self::Target>;
}

pub struct JoinIter<'w, T>{
    pub items: Box<Iterator<Item = (bool, Option<T>)> + 'w>
}

impl<'w, T> Iterator for JoinIter<'w, T>{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item>{
        // If v exists we can step in items
        while let Some(i) = self.items.next(){
            if i.0{
                return Some(i.1.unwrap());
            }
        }
        None
    }
}

// This should be unneeded now but I'm going to keep it around so
// if I ever need to change the implementation of the macro
// I'll know what the output should look like
/*
impl<'w, A, B> Joinable<'w> for (A, B)
    where A: Joinable<'w> + 'w,
          B: Joinable<'w> + 'w{
    
    type Target = (A::Target, B::Target);

    fn join(self) -> JoinIter<'w, Self::Target>{
        JoinIter{
            items: Box::new(
                self.0.join().items
                    .zip(self.1.join().items)
                    .map(|(a, b)|{
                        if a.0 && b.0{
                            return (true, Some((a.1.unwrap(), b.1.unwrap())));
                        }
                        else{
                            return (false, None);
                        }
                    })
            )       
        }
    }
}
*/

impl_joinable_multi!(16);
