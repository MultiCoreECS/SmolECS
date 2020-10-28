use proc_macro::{TokenStream, TokenTree};

trait Concat{
    fn concat(&mut self, other: String);
}

impl Concat for String{
    fn concat(&mut self, other: String){
        *self = format!("{}{}", self, other);
    }
}

#[proc_macro]
pub fn impl_joinable(input: TokenStream) -> TokenStream {
    let tokens: Vec<String> = input
        .to_string()
        .split(',')
        .map(|token|{
            token.to_uppercase()
        })
        .collect();

    let mut out_stream = String::new();

    out_stream.concat(format!("impl<'w"));
    for token in tokens.iter(){
        out_stream.concat(format!(",{} ", token));
    }

    out_stream.concat(format!("> Joinable<'w> for ("));
    for (n, token) in tokens.iter().enumerate(){
        if n == 0{
            out_stream.concat(format!("{}", token));
        }
        else{
            out_stream.concat(format!(",{}", token));
        }
    }
    out_stream.concat(format!(")"));

    for (n, token) in tokens.iter().enumerate(){
        if n == 0{
            out_stream.concat(format!("where {}: Joinable<'w> + 'w", token));
        }
        else{
            out_stream.concat(format!(",{}: Joinable<'w> + 'w", token));
        }
    }
    
    // Open impl block
    out_stream.concat(format!("{{\ntype Target = ("));

    for (n, token) in tokens.iter().enumerate(){
        if n == 0{
            out_stream.concat(format!("{}::Target", token));
        }
        else{
            out_stream.concat(format!(",{}::Target", token));
        }
    }
    
    // Open join block
    // Open JoinIter block
    // Open box block
    out_stream.concat(format!(");
        fn join(self) -> JoinIter<'w, Self::Target>{{
            JoinIter{{
                items: Box::new("));

    for (n, token) in tokens.iter().enumerate(){
        if n == 0{
            out_stream.concat(format!("self.{}.join().items", n));
        }
        else{
            out_stream.concat(format!(".zip(self.{}.join().items)", n));
        }
    }

    out_stream.concat(format!(".map(|"));
    for (n, token) in tokens.iter().enumerate(){
        if n == 0{
            for par in 0..(tokens.len() - 1){
                out_stream.concat(format!("("));
            }
            out_stream.concat(format!("{}", token.to_lowercase()));
        }
        else{
            out_stream.concat(format!(",{})", token.to_lowercase()));
        }
    }

    out_stream.concat(format!("|{{ if "));
    for (n, token) in tokens.iter().enumerate(){
        if n == 0{
            out_stream.concat(format!("{}.0", token.to_lowercase()));
        }
        else{
            out_stream.concat(format!("&& {}.0", token.to_lowercase()));
        }
    }

    out_stream.concat(format!("{{ return (true, Some(("));
    for (n, token) in tokens.iter().enumerate(){
        if n == 0{
            out_stream.concat(format!("{}.1.unwrap()", token.to_lowercase()));
        }
        else{
            out_stream.concat(format!(", {}.1.unwrap()", token.to_lowercase()));
        }
    }
    out_stream.concat(format!(")));}}else{{return (false, None);}} }})"));

    // Close box block
    // Close JoinIter block
    // Close join block
    out_stream.concat(format!(")\n }}\n }}\n }}"));
    out_stream.parse().unwrap()
}

#[proc_macro]
pub fn impl_joinable_multi(input: TokenStream) -> TokenStream {
    let arg = input.to_string().parse::<usize>().unwrap();

    let mut out_stream = String::new();

    for i in (0..arg).skip(1){
        out_stream.concat(format!("impl_joinable!("));
        for j in 0..=i{
            if j == 0{
                out_stream.concat(format!("T{}", j));
            }
            else{
                out_stream.concat(format!(", T{}", j));
            }
        }
        out_stream.concat(format!(");"));
    }

    out_stream.parse().unwrap()
}

#[proc_macro]
pub fn impl_system_data(input: TokenStream) -> TokenStream{
    let tokens: Vec<String> = input
        .to_string()
        .split(',')
        .map(|token|{
            token.to_uppercase()
        })
        .collect();

    let mut out_stream = String::new();

    out_stream.concat(format!("impl<'d"));
    for token in tokens.iter(){
        out_stream.concat(format!(", {}: SystemData<'d>", token));
    }
    out_stream.concat(format!("> SystemData<'d> for ({}", tokens[0]));
    
    for token in tokens.iter().skip(1){
        out_stream.concat(format!(", {}", token));
    }
    out_stream.concat(format!("){{ fn get_data<'w: 'd, W: WorldCommon>(world: &'w W) -> Self{{({}::get_data(world)", tokens[0]));
    for token in tokens.iter().skip(1){
        out_stream.concat(format!(", {}::get_data(world)", token));
    }
    out_stream.concat(format!(")}}"));

    out_stream.concat(format!("fn get_dep_vec<'w: 'd, W: WorldCommon>(world: &W) -> DepVec{{ {}::get_dep_vec(world)", tokens[0]));
    for token in tokens.iter().skip(1){
        out_stream.concat(format!(".and(&{}::get_dep_vec(world))", token));
    }
    out_stream.concat(format!("}} }}"));
    

    out_stream.parse().unwrap()
}

#[proc_macro]
pub fn impl_system_data_multi(input: TokenStream) -> TokenStream {
    let arg = input.to_string().parse::<usize>().unwrap();

    let mut out_stream = String::new();

    for i in (0..arg).skip(1){
        out_stream.concat(format!("impl_system_data!("));
        for j in 0..=i{
            if j == 0{
                out_stream.concat(format!("T{}", j));
            }
            else{
                out_stream.concat(format!(", T{}", j));
            }
        }
        out_stream.concat(format!(");"));
    }

    out_stream.parse().unwrap()
}