#![feature(test)]

extern crate test;

#[cfg(test)]
mod tests {
    use test::{black_box, Bencher};

    macro_rules! with_vec_push {
        ( $( $e:expr ),* ) => {{
            let mut tmp = Vec::new();
            $(
                tmp.push($e);
            )*
            tmp.into_iter().collect::<Vec<_>>()
        }};
    }

    #[bench]
    fn vec_push(b: &mut Bencher) {
        b.iter(|| {
            black_box(with_vec_push![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
        })
    }

    macro_rules! with_vec {
        ( $( $e:expr ),* ) => { (vec![ $( $e ),* ]).into_iter().collect::<Vec<_>>() };
    }

    #[bench]
    fn vec(b: &mut Bencher) {
        b.iter(|| {
            black_box(with_vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
        })
    }

    macro_rules! with_slice {
        ( $( $e:expr ),* ) => { (&[ $( $e ),* ][..]).into_iter().collect::<Vec<_>>() };
    }

    #[bench]
    fn slice(b: &mut Bencher) {
        b.iter(|| {
            black_box(with_slice![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
        })
    }
}
