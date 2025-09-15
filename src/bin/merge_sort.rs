fn merge_sort(arr: &mut [i32]) {
    let len = arr.len();
    if len <= 1 {
        return;
    }

    let mid = len / 2;

    // Cria vetores temporários para cada metade
    let mut left = arr[..mid].to_vec();
    let mut right = arr[mid..].to_vec();

    // Ordena recursivamente cada metade
    merge_sort(&mut left);
    merge_sort(&mut right);

    // Mescla as duas metades ordenadas
    let mut i = 0;
    let mut j = 0;
    let mut k = 0;

    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            arr[k] = left[i];
            i += 1;
        } else {
            arr[k] = right[j];
            j += 1;
        }
        k += 1;
    }

    // Copia o restante da esquerda
    while i < left.len() {
        arr[k] = left[i];
        i += 1;
        k += 1;
    }

    // Copia o restante da direita
    while j < right.len() {
        arr[k] = right[j];
        j += 1;
        k += 1;
    }
}

fn main() {
    let mut array = [12, 11, 13, 5, 6, 7];
    println!("Array Original: {:?}", array);

    merge_sort(&mut array);
    println!("Merge Sort: {:?}", array);
}
