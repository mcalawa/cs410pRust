pub fn raindrops(n: usize) -> String {
    let mut raindrop_speak = "".to_string();

    if n % 3 == 0 {
        raindrop_speak.push_str("Pling");
    }
    
    if n % 5 == 0 {
        raindrop_speak.push_str("Plang");
    }
        
    if n % 7 == 0 {
        raindrop_speak.push_str("Plong");
    }
    
    if raindrop_speak.len() == 0 {
        raindrop_speak.push_str(&n.to_string());
    }
    
    raindrop_speak
}
