//! custom_serialize

#[test]
fn test_serializable() {
    // def test_serializable(obj):
    //     dumped = json.dumps(obj, cls=Serializer)
    //     loaded = json.loads(dumped, cls=Serializer)
    //
    //     assert loaded == obj
    assert!(false);
}

#[test]
fn test_default_save_load() {
    // def test_default_save_load(obj):
    //     # test that have 'save' and 'load' methods added by default
    //     assert hasattr(obj, 'save')
    //     assert hasattr(obj, 'load')
    //     assert obj.__class__.load(obj.save()) == obj
    assert!(false);
}

#[test]
fn test_serializable_with_class_methods() {
    // def test_serializable_with_class_methods(obj):
    //     dumped_srz = json.dumps(obj, cls=Serializer)
    //     dumped_self = obj.to_json()
    //
    //     assert isinstance(dumped_srz, str)
    //     assert isinstance(dumped_self, dict)
    //
    //     decoded_self = obj.from_json(dumped_self)
    //     decoded_srz = json.loads(dumped_srz, cls=Serializer)
    //
    //     assert decoded_self == decoded_srz == obj
    assert!(false);
}

#[test]
fn test_serializable_custom() {
    // def test_serializable_custom(obj):
    //     dumped_srz = json.dumps(obj, cls=Serializer)
    //     dumped_self = encode_custom(obj)
    //
    //     assert isinstance(dumped_srz, str)
    //     assert isinstance(dumped_self, dict)
    //
    //     decoded_self = decode_custom(obj.__class__, dumped_self)
    //     decoded_srz = json.loads(dumped_srz, cls=Serializer)
    //
    //     assert decoded_self == decoded_srz == obj
    assert!(false);
}
