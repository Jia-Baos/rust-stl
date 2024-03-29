use std::cmp::min;

pub fn edit_distance1(source: &str, target: &str) -> usize {
    // 极端情况：空字符串到字符串的转换
    if source.is_empty() {
        return target.len();
    } else if target.is_empty() {
        return source.len();
    }

    // 建立矩阵存储过程值
    let source_c = source.chars().count();
    let target_c = target.chars().count();
    let mut distance = vec![vec![0; target_c + 1]; source_c + 1];

    for i in 1..=source_c {
        distance[i][0] = i;
    }
    for j in 1..=target_c {
        distance[0][j] = j;
    }

    // 存储过程值，取增、删、改中的最小步骤数
    for (i, cs) in source.chars().enumerate() {
        for (j, ct) in target.chars().enumerate() {
            let ins = distance[i + 1][j] + 1;
            let del = distance[i][j + 1] + 1;
            let sub = distance[i][j] + (cs != ct) as usize;
            distance[i + 1][j + 1] = min(min(ins, del), sub);
        }
    }

    // 返回最后一行最后一列的值
    *distance.last().and_then(|d| d.last()).unwrap()
}

pub fn edit_distance2(source: &str, target: &str) -> usize {
    if source.is_empty() {
        return target.len();
    } else if target.is_empty() {
        return source.len();
    }

    // distances 存储了到各种字符串的编辑距离
    let target_c = target.chars().count();
    let mut distances = (0..=target_c).collect::<Vec<_>>();

    for (i, cs) in source.chars().enumerate() {
        let mut substt = i;
        distances[0] = substt + 1;
        for (j, ct) in target.chars().enumerate() {
            let dist = min(min(distances[j], distances[j + 1]) + 1,
                           substt + (cs != ct) as usize);
            substt = distances[j + 1];
            distances[j + 1] = dist;
        }
    }


    distances.pop().unwrap()
}

#[cfg(test)]
mod tests {
    use crate::demo::edit_distance;

    #[test]
    fn test_edit_distance1() {
        let source = "abce";
        let target = "adcf";
        let distance = edit_distance::edit_distance1(source, target);
        assert_eq!(distance, 2);

        let source = "bdfc";
        let target = "adcf";
        let distance = edit_distance::edit_distance1(source, target);
        assert_eq!(distance, 3);
    }

    #[test]
    fn test_edit_distance2() {
        let source = "abce";
        let target = "adcf";
        let distance = edit_distance::edit_distance2(source, target);
        assert_eq!(distance, 2);

        let source = "bdfc";
        let target = "adcf";
        let distance = edit_distance::edit_distance2(source, target);
        assert_eq!(distance, 3);
    }
}