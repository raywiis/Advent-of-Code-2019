
fn main() {
    let start = 145_852;
    let end = 616_942;

    let mut count = 0;
    for i in start..end {
        if verify_password(i) {
            count += 1;
        }
    }

    println!("[Day 4] total passwords: {}", count);
}

fn verify_password(mut password: i32) -> bool {
    let mut digits: [i32; 6] = [0; 6];
    for i in 0..6 {
        let idx = 5 - i;
        digits[idx] = password % 10;
        password /= 10;
    }

    for i in 1..6 {
        if digits[i - 1] > digits[i] {
            return false
        }
    }

    let mut group = 1;
    for i in 1..6 {
        if digits[i - 1] == digits[i] {
            group += 1;
        } else if group == 2 {
            return true;
        } else {
            group = 1;
        }
    }
    if group == 2 {
        return true;
    }

    false
}
