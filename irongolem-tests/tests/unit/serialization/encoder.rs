//! encoder

#[test]
fn test_encoder() {
    // def test_encoder(case: EncoderTestCase, mock_classes_fixture):
    //     serializer = Serializer()
    //     if getattr(case.test_input, '__dict__', None) is not None:
    //         keys_before = vars(case.test_input).keys()
    //         encoded = {k: v for k, v in serializer.default(case.test_input).items() if k != CLASS_PATH_KEY}
    //         keys_after = vars(case.test_input).keys()
    //     else:
    //         keys_before = keys_after = {}
    //         encoded = {k: v for k, v in serializer.default(case.test_input).items() if k != CLASS_PATH_KEY}
    //     assert encoded == case.test_answer, 'Encoded json objects are not the same'
    //     assert keys_before == keys_after, 'Object instance was changed'
    //     if isinstance(case.test_input, MockGraph):
    //         assert MOCK_NODE_1.uid == MOCK_NODE_1_COPY.uid
    //         for node in case.test_input.nodes:
    //             assert getattr(node, 'uid', None) is not None
    assert!(false);
}
