use amber_msgs_metaio::CommandMetadata;

fn main() {
    gear_wasm_builder::build_with_metadata::<CommandMetadata>();
}
