pub fn raindrops(n: usize) -> String {
    let mut return_obj = String::with_capacity(60);
    if n % 3 == 0 { return_obj.push_str("Pling");}
    if n % 5 == 0 { return_obj.push_str("Plang");}
    if n % 7 == 0 { return_obj.push_str("Plong");}

    if return_obj.len() == 0 { return_obj.push_str(&n.to_string()); }

    return_obj
}
