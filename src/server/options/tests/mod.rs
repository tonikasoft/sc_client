#[allow(unused_imports)] use super::*;

#[test]
fn parse_ugen_plugins_path() {
    let mut options = Options::default();
    options.ugen_plugins_path = Some(vec!("/foo/bar".to_string(), "/home/ugens".to_string(), "/some/more".to_string()));
    assert_eq!(Some(String::from("/foo/bar:/home/ugens:/some/more")), options.parse_ugen_plugins_path());

    options.ugen_plugins_path = Some(vec!("/foo/bar".to_string()));
    assert_eq!(Some(String::from("/foo/bar")), options.parse_ugen_plugins_path());

    options.ugen_plugins_path = Some(vec!());
    assert_eq!(None, options.parse_ugen_plugins_path());

    options.ugen_plugins_path = None;
    assert_eq!(None, options.parse_ugen_plugins_path());
}
