#[allow(unused_imports)] use super::*;

#[test]
fn get_ugen_plugins_path_as_argument_str() {
    let mut options = Options::default();
    options.ugen_plugins_path = Some(vec!("/foo/bar".to_string(), "/home/ugens".to_string(), "/some/more".to_string()));
    assert_eq!(Some(String::from("/foo/bar:/home/ugens:/some/more")), options.get_ugen_plugins_path_as_argument_str());

    options.ugen_plugins_path = Some(vec!("/foo/bar".to_string()));
    assert_eq!(Some(String::from("/foo/bar")), options.get_ugen_plugins_path_as_argument_str());

    options.ugen_plugins_path = Some(vec!());
    assert_eq!(None, options.get_ugen_plugins_path_as_argument_str());

    options.ugen_plugins_path = None;
    assert_eq!(None, options.get_ugen_plugins_path_as_argument_str());
}
