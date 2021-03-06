use std::cmp::Ordering::*;

#[allow(dead_code)]
pub fn selection_sort_naive<T: PartialOrd>(list: &mut [T]) {
  for i in 0..list.len() {
    let mut small = i;
    for j in (i + 1)..list.len() {
      if list[j] < list[small] {
        small = j;
      }
    }
    list.swap(small, i);
  }
}
#[allow(dead_code)]
pub fn selection_sort<T: Ord>(list: &mut [T]) {
  for i in 0..list.len() {
    if let Some((j, _)) = list.iter()
      .enumerate()
      .skip(i)
      .min_by_key(|x| x.1){
        list.swap(i, j);
      }
  }
}
#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn test_selection_sort_naive() {
    let mut unsorted = vec![
      691040, 83642, 993016, 707058, 383979, 710110, 173194, 338622, 305874, 33962, 237030,
      632260, 481490, 413149, 741138, 762856, 345386, 478834, 20818, 624760, 458200, 237089,
      887982, 994813, 667671, 365350, 375510, 420190, 884819, 928030, 542702, 950454, 717027,
      721050, 318440, 334246, 302284, 721610, 851794, 583184, 822652, 667434, 864259, 869115,
      340056, 618758, 53453, 565501, 793777, 466741,
    ];

    let sorted = vec![
      20818, 33962, 53453, 83642, 173194, 237030, 237089, 302284, 305874, 318440, 334246, 338622,
      340056, 345386, 365350, 375510, 383979, 413149, 420190, 458200, 466741, 478834, 481490,
      542702, 565501, 583184, 618758, 624760, 632260, 667434, 667671, 691040, 707058, 710110,
      717027, 721050, 721610, 741138, 762856, 793777, 822652, 851794, 864259, 869115, 884819,
      887982, 928030, 950454, 993016, 994813,
    ];

    selection_sort_naive(&mut unsorted);
    assert_eq!(
      sorted,
      unsorted
    );
  }

  #[test]
  fn test_selection_sort() {
    let mut unsorted = vec![
      691040, 83642, 993016, 707058, 383979, 710110, 173194, 338622, 305874, 33962, 237030,
      632260, 481490, 413149, 741138, 762856, 345386, 478834, 20818, 624760, 458200, 237089,
      887982, 994813, 667671, 365350, 375510, 420190, 884819, 928030, 542702, 950454, 717027,
      721050, 318440, 334246, 302284, 721610, 851794, 583184, 822652, 667434, 864259, 869115,
      340056, 618758, 53453, 565501, 793777, 466741,
    ];

    let sorted = vec![
      20818, 33962, 53453, 83642, 173194, 237030, 237089, 302284, 305874, 318440, 334246, 338622,
      340056, 345386, 365350, 375510, 383979, 413149, 420190, 458200, 466741, 478834, 481490,
      542702, 565501, 583184, 618758, 624760, 632260, 667434, 667671, 691040, 707058, 710110,
      717027, 721050, 721610, 741138, 762856, 793777, 822652, 851794, 864259, 869115, 884819,
      887982, 928030, 950454, 993016, 994813,
    ];

    selection_sort(&mut unsorted);
    assert_eq!(
      sorted,
      unsorted
    );
  }
}
