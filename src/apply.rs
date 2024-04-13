#[macro_export]
macro_rules! apply {
    ($this:ident, { $($prop:ident = $val:expr);+ ; }) => {
        {
            $(
               $this.$prop = $val;
            )*
            $this
        }
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn apply_simple() {
        #[derive(Debug, Default, Eq, PartialEq)]
        struct TestStruct {
            pub a: usize,
            pub b: usize,
            pub c: usize,
        }

        let mut res = TestStruct::default();

        assert_eq!(
            apply!(res, {
                a = 1;
                b = 2;
                c = 3;
            }),
            TestStruct { a: 1, b: 2, c: 3 }
        );
    }
}
