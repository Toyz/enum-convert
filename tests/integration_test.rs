use enum_convert::EnumConvert;

#[derive(EnumConvert, Debug, PartialEq)]
#[EnumType = "i32"]
enum NumericEnumi32 {
    Zero,
    One,
    Two = 100,
    Three,
    Four,
    Five = 1000,
    Six,
}

#[test]
fn numeric_enum_i32_conversion() {
    assert_eq!(NumericEnumi32::Zero as i32, 0);
    assert_eq!(NumericEnumi32::One as i32, 1);
    assert_eq!(NumericEnumi32::Two as i32, 100);
    assert_eq!(NumericEnumi32::Three as i32, 101);
    assert_eq!(NumericEnumi32::Four as i32, 102);
    assert_eq!(NumericEnumi32::Five as i32, 1000);
    assert_eq!(NumericEnumi32::Six as i32, 1001);

    // try from
    assert_eq!(0i32.try_into(), Ok(NumericEnumi32::Zero));
    assert_eq!(1i32.try_into(), Ok(NumericEnumi32::One));
    assert_eq!(100i32.try_into(), Ok(NumericEnumi32::Two));
    assert_eq!(101i32.try_into(), Ok(NumericEnumi32::Three));
    assert_eq!(102i32.try_into(), Ok(NumericEnumi32::Four));
    assert_eq!(1000i32.try_into(), Ok(NumericEnumi32::Five));
    assert_eq!(1001i32.try_into(), Ok(NumericEnumi32::Six));
}

#[derive(EnumConvert, Debug, PartialEq)]
#[EnumType = "u32"]
enum NumericEnumu32 {
    Zero,
    One,
    Two = 100,
    Three,
    Four,
    Five = 1000,
    Six,
}

#[test]
fn numeric_enum_u32_conversion() {
    assert_eq!(NumericEnumi32::Zero as u32, 0);
    assert_eq!(NumericEnumi32::One as u32, 1);
    assert_eq!(NumericEnumi32::Two as u32, 100);
    assert_eq!(NumericEnumi32::Three as u32, 101);
    assert_eq!(NumericEnumi32::Four as u32, 102);
    assert_eq!(NumericEnumi32::Five as u32, 1000);
    assert_eq!(NumericEnumi32::Six as u32, 1001);

// try from
    assert_eq!(0u32.try_into(), Ok(NumericEnumu32::Zero));
    assert_eq!(1u32.try_into(), Ok(NumericEnumu32::One));
    assert_eq!(100u32.try_into(), Ok(NumericEnumu32::Two));
    assert_eq!(101u32.try_into(), Ok(NumericEnumu32::Three));
    assert_eq!(102u32.try_into(), Ok(NumericEnumu32::Four));
    assert_eq!(1000u32.try_into(), Ok(NumericEnumu32::Five));
    assert_eq!(1001u32.try_into(), Ok(NumericEnumu32::Six));
}

#[derive(EnumConvert, Debug, PartialEq)]
#[EnumType = "String"]
enum StringTestEnum {
    Alpha,
    Beta,
}

#[test]
fn string_conversion() {
    assert_eq!("Alpha".parse::<StringTestEnum>().expect("StringTestEnum::Alpha from Alpha"), StringTestEnum::Alpha);
    assert_eq!(StringTestEnum::Beta.to_string(), "Beta");
}

#[derive(EnumConvert, Debug, PartialEq)]
#[EnumType = "String,i32"]
enum MultiTypeTestEnum {
    Alpha,
    Beta = 100,
    Gamma,
}

#[test]
fn multi_type_conversion() {
    assert_eq!("Alpha".parse::<MultiTypeTestEnum>().expect("MultiTypeTestEnum::Alpha from Alpha"), MultiTypeTestEnum::Alpha);
    assert_eq!(MultiTypeTestEnum::Beta.to_string(), "Beta");
    assert_eq!(MultiTypeTestEnum::Beta as i32, 100);
    assert_eq!(100.try_into(), Ok(MultiTypeTestEnum::Beta));

    assert_eq!("Gamma".parse::<MultiTypeTestEnum>().expect("MultiTypeTestEnum::Gamma from Gamma"), MultiTypeTestEnum::Gamma);
    assert_eq!(MultiTypeTestEnum::Gamma.to_string(), "Gamma");
    assert_eq!(MultiTypeTestEnum::Gamma as i32, 101);
    assert_eq!(101.try_into(), Ok(MultiTypeTestEnum::Gamma));
}