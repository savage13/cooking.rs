use cooking::Cook;

#[test]
fn test_dye() {
    let cook = Cook::new();
    cook.cook(&["Navy", "Orange", "Brown", "Gray"]);
}

#[test]
fn test_picture() {
    let cook = Cook::new();
    cook.cook(&[
        "Fauna Picture", 
        "Enemy Picture",
        "Material Picture",
        "Other Picture"
    ]);
    cook.cook(&[
        "Weapon Picture", 
        "Elite Enemy Picture",
    ]);
}
