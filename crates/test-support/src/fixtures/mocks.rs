//! Legacy mock placeholders — domain mocks live in `mock_adapter.rs`.

#[derive(Debug, Default)]
pub struct MockAdapterStub;

pub fn mock_graph_with_params() -> irongolem::golem::dag::GraphDelegate {
    irongolem::golem::dag::GraphDelegate::empty()
}
