#[allow(non_upper_case_globals)]
pub fn button_container() -> String {
    stylist::style!(
        "
        background-color: green;
        border: none;
        color: white;
        text-align: center;
        display: inline-block;
    "
    )
    .unwrap()
    .get_class_name()
    .to_string()
}

#[allow(non_upper_case_globals)]
pub fn button_text() -> String {
    stylist::style!(
        "
        font-size: 2rem;
    "
    )
    .unwrap()
    .get_class_name()
    .to_string()
}
