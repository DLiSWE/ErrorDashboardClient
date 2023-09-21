#[allow(non_upper_case_globals)]
pub fn navbar_logo_container() -> String {
    stylist::style!(
        "
        display: flex;
        flex-direction: row;
        justify-content: space-between;
        min-width:300px;
        align-items: center;
        align-text: center;
        padding: 0 10px 0 10px;
        :hover {
            cursor: pointer;
        }
        @media screen and (max-width: 640px) {
            font-size: 14px;
        }
    "
    )
    .unwrap()
    .get_class_name()
    .to_string()
}

#[allow(non_upper_case_globals)]
pub fn navbar_container() -> String {
    stylist::style!(
        "
        display: flex;
        flex-direction: row;
        justify-content: space-between;
        align-items: center;
        align-text: center;
        background-color: #DFE6DA;
    "
    )
    .unwrap()
    .get_class_name()
    .to_string()
}
