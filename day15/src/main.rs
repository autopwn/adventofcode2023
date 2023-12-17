struct Lens {
    label: String,
    focal_length: u32,
}

struct LensBox {
    number: u32,
    lenses: Vec<Lens>,
}

impl LensBox {
    fn new(number: u32) -> Self {
        LensBox {
            number,
            lenses: Vec::default(),
        }
    }
}

fn hash(string: &str) -> u32 {
    string
        .chars()
        .fold(0, |acc, c| ((acc + c as u32) * 17) % 256)
}

fn part1(steps: &Vec<&str>) -> u32 {
    steps.iter().map(|s| hash(s)).sum()
}

fn part2(steps: &Vec<&str>) -> u32 {
    // build boxes
    let mut boxes: Vec<_> = (0..=255).map(|n| LensBox::new(n)).collect();
    for step in steps {
        if let Some((label, focal)) = step.split_once("=") {
            let focal = focal.parse().unwrap();
            let number = hash(label) as usize;
            let mut lens_replaced = false;
            // try to replace existing lense in box
            for lens in &mut boxes[number].lenses.iter_mut() {
                if lens.label == label {
                    *lens = Lens {
                        label: label.to_string(),
                        focal_length: focal,
                    };
                    lens_replaced = true;
                }
            }
            if !lens_replaced {
                // insert lens
                boxes[number].lenses.push(Lens {
                    label: label.to_string(),
                    focal_length: focal,
                });
            }
        } else {
            let label = &step[..step.len() - 1];
            let number = hash(&label) as usize;
            // try to remove lens
            if let Some(position) = boxes[number].lenses.iter().position(|l| l.label == label) {
                boxes[number].lenses.remove(position);
            }
        }
    }

    // calculate focussing power
    boxes
        .iter()
        .filter(|b| b.lenses.len() > 0)
        .map(|b| {
            b.lenses
                .iter()
                .enumerate()
                .map(|(idx, lens)| (b.number + 1) * (idx as u32 + 1) * lens.focal_length)
                .sum::<u32>()
        })
        .sum()
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let steps: Vec<_> = input.trim().split(",").collect();
    println!("Solution of part 1 {}", part1(&steps));
    println!("Solution of part 2 {}", part2(&steps));
}
