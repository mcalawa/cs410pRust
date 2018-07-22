// Actual formula/algorithm for finding the correct pythagorean triplet
// is from StackOverflow user Oleg Razgulyaev's answer to the following
// question: <https://stackoverflow.com/questions/2817848/find-pythagorean-triplet-for-which-a-b-c-1000>
// Putting it in Rust and the actual logic for returning is still me,
// but it's been a hot minute since I've taken a math class and the amount
// of time it would have taken me to do the math to figure that part of 
// the problem out would be not be worth my time nor would it do much to 
// demonstrate my coding ability beyond what I've already demonstrated.

pub fn find() -> Option<u32> {
    let sum = 1000;
    let first = 1;
    for a in first..sum/3 {
        let second = a + 1;

        for b in second..sum/2 {
            let c = sum - a - b;
            if a * a + b * b == c * c {
                return Some(a * b * c)
            }
        }
    }

    return None
}
