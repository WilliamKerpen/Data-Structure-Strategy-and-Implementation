fn partition(arr: &mut [i32]) -> usize {
    let len = arr.len();
    let pivot = arr[len - 1]; // pivô é o último elemento
    let mut i = 0;

    for j in 0..(len - 1) {
        if arr[j] < pivot {
            arr.swap(i, j);
            i += 1;
        }
    }
    arr.swap(i, len - 1);
    i
}

fn quick_sort(arr: &mut [i32]) {
    if arr.len() <= 1 {
        return;
    }

    let pivot_index = partition(arr);

    // divide o slice em duas partes: antes e depois do pivô
    let (left, right) = arr.split_at_mut(pivot_index);
    
    quick_sort(left); // ordena lado esquerdo
    
    // o pivô está em `right[0]`, então ordenamos do 1 em diante
    if right.len() > 1 {
        quick_sort(&mut right[1..]);
    }
}

fn main() {
    let mut array = [10, 7, 8, 9, 1, 5];
    println!("Array original: {:?}", array);

    quick_sort(&mut array);
    println!("Quick Sort: {:?}", array);
}
