use extendr_api::prelude::*;

use heck::{
    ToKebabCase, ToPascalCase, ToShoutyKebabCase, ToShoutySnakeCase, ToSnekCase, ToTitleCase,
    ToTrainCase, ToUpperCamelCase,
};

macro_rules! make_heck_fn {
    ($fn_name:ident) => {
        /// @export
        #[extendr]
        fn $fn_name(x: Strings) -> Strings {
            x.into_iter()
                .map(|xi| match xi.is_na() {
                    true => Rstr::na(),
                    false => Rstr::from(xi.as_ref().$fn_name()),
                })
                .collect::<Strings>()
        }
    };
}

make_heck_fn!(to_snek_case);
make_heck_fn!(to_shouty_snake_case);
make_heck_fn!(to_kebab_case);
make_heck_fn!(to_shouty_kebab_case);
make_heck_fn!(to_pascal_case);
make_heck_fn!(to_upper_camel_case);
make_heck_fn!(to_train_case);
make_heck_fn!(to_title_case);

extendr_module! {
    mod heck;
    fn to_snek_case;
    fn to_shouty_snake_case;
    fn to_kebab_case;
    fn to_shouty_kebab_case;
    fn to_pascal_case;
    fn to_upper_camel_case;
    fn to_train_case;
    fn to_title_case;
}
