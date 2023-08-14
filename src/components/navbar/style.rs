#[allow(non_upper_case_globals)]
pub fn navbar_container() -> String {
    stylist::style!("
        display: flex;
        flex-direction: row;
        justify-content: space-between;
        align-items: center;
        align-text: center;
        background-color: #DFE6DA;
        ul {
            list-style-type: none;
            padding: 0;
            display: flex;
            flex-direction: row;
        }
    ").unwrap().get_class_name().to_string()
}

#[allow(non_upper_case_globals)]
pub fn navlinks() -> String {
    stylist::style!("
        min-width: 300px;
        justify-content: space-around;
    ").unwrap().get_class_name().to_string()
}

#[allow(non_upper_case_globals)]
pub fn navbutton() -> String {
    stylist::style!("
        background-color: #d9dcd7;
        justify-content: space-around;
        padding: 4px 8px 4px 8px;
        font-size: 1rem;
        border: none;
        border-radius:5px;
        :hover {
            background-color: #babfb6;
        }
    ").unwrap().get_class_name().to_string()
}
