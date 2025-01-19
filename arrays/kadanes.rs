// PROB: Maximum of sub array
fn kadanes(a: &[i32]) -> i32 {
    let mut sum = 0;
    let mut maxi = i32::MIN;

    let mut j = 0;
    while j < a.len() - 1 {
        sum += a[j];

        if sum < 0 {
            sum = 0;
        }

        maxi = sum.max(maxi);

        // This will fail when all in array are negative
        // if a[j] <= sum + a[j] {
        //     sum += a[j];
        //     msum = msum.max(sum);
        // } else {
        //     sum = a[j];
        //     i = j;
        // }

        j += 1;
    }

    return maxi;
}

fn main() {
    let res = kadanes(&[-2, 6, -3, 3, 4, -1, -2, 1, 5, -3]);
    println!("{res}");

    let res = kadanes(&[-2, -6, -3, -1, -5, -3]);
    println!("{res}");
}
