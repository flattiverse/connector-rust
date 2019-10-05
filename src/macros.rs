macro_rules! vec_of_none {
    ($x:expr) => {{
        let mut vec = Vec::with_capacity($x);
        (0..$x).for_each(|_| vec.push(None));
        vec
    }};
}

macro_rules! expand_vec_of_none_if_necessary {
    ($vec:expr, $index:expr) => {{
        if $index + 1 > $vec.len() {
            let diff = $index - $vec.len() + 1;
            $vec.reserve(diff);
            (0..diff).for_each(|_| $vec.push(None));
        }
    }};
}
macro_rules! map_payload_with_try_from {
    ($packet:expr, $t:ident) => {{
        if $packet.payload.is_some() {
            use std::convert::TryFrom;
            Some($t::try_from($packet)?)
        } else {
            None
        }
    }};
}
