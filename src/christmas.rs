
fn main() {
    let triangles = 5; // Задайте кількість трикутників

    draw_tree(triangles);
}

fn draw_tree(triangles: usize) {
    let mut width = 1;

    // Ітеруємо по кількості трикутників
    for t in 1..=triangles {
        // Ітеруємо по рівнях кожного трикутника
        for level in 0..t {
            let spaces = " ".repeat(triangles + triangles - level - 1);
            let stars = "*".repeat(width);
            println!("{}{}", spaces, stars);
            width += 2;
        }
    }
}

pub(crate) fn celebrate() {
    todo!()
}