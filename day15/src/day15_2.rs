const FILE_DATA: &str = include_str!("input.txt");

pub fn hash(data: &str) -> usize {
    let mut h = 0;

    for c in data.chars() {
        h += c as usize;
        h *= 17;
        h %= 256;
    }

    h
}

pub fn answer() -> usize {
    let mut boxes: Vec<Vec<(&str, usize)>> = vec![Vec::new(); 256];
    
    for code in FILE_DATA.split(',') {
        let letters = code.split_once(['=', '-']).unwrap().0;
        let idx = hash(letters);

        if code.ends_with('-') {
            let mut delete_idx = None;

            for (i, (string, _)) in boxes[idx].iter().enumerate() {
                if *string == letters {
                    delete_idx = Some(i);
                    break;
                }
            }

            if let Some(i) = delete_idx {
                boxes[idx].remove(i);
            }
        } else {
            let lens = code.chars().last().unwrap() as usize  - '0' as usize;

            if let Some((_, existing_lens)) = boxes[idx].iter_mut().find(|(string, _)| *string == letters) {
                *existing_lens = lens;
            } else {
                boxes[idx].push((letters, lens));
            }
        }
    }

    let mut sum = 0;
    for (idx_box, tgt_box) in boxes.iter().enumerate() {
        for (lens_idx, &(_, lens_focal)) in tgt_box.iter().enumerate() {
            sum += (1 + idx_box) * (lens_idx + 1) * lens_focal;
        }
    }

    sum
}