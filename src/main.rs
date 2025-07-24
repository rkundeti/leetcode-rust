fn third_max(nums: Vec<i32>) -> i32 {
    let mut first_max = i64::MIN;
    let mut second_max = i64::MIN;
    let mut third_max = i64::MIN;

    for &num in &nums {
        let num = num as i64;

        if num == first_max || num == second_max || num == third_max {
            continue;
        }

        if num > first_max {
            third_max = second_max;
            second_max = first_max;
            first_max = num;
        }
        else if num > second_max {
            third_max = second_max;
            second_max = num;
        } else if num > third_max {
            third_max = num;
        }
    }

    if third_max == i64::MIN {
        first_max as i32
    } else {
        third_max as i32
    }
}

fn main() {
    let input = vec![1,2,5,67,4];
    let result = third_max(input);
    println!("{}", result);
    }
