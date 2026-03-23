use unicode_general_category::{GeneralCategory, get_general_category};

pub(super) fn is_accented(c: char) -> bool {
    let category = get_general_category(c);
    category == GeneralCategory::ModifierSymbol
}