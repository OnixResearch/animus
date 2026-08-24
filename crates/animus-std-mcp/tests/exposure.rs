//! Positive and negative MCP exposure policy fixtures.

use animus_std_mcp::{
    BuiltInToolSource, ChildToolAuthority, ExactMutationAuthority, ExposureDenial,
    McpExposureDescriptor, McpExposureRequest, ReadOnlyHint, project_child_mcp_exposure,
    reduce_mcp_exposure,
};

const SESSION: &str = "session.root";
const GENERATION: u64 = 7;
const SERVER: &str = "server.alpha";
const SCHEMA: &str = "b3:schema";
const OPERATION: &str = "tool.call";

fn descriptor(name: &'static str, hint: ReadOnlyHint) -> McpExposureDescriptor<'static> {
    McpExposureDescriptor {
        server_identity: SERVER,
        tool_name: name,
        schema_identity: SCHEMA,
        operation_class: OPERATION,
        session_identity: SESSION,
        generation: GENERATION,
        read_only_hint: hint,
    }
}

fn exact_authority(name: &'static str) -> ExactMutationAuthority<'static> {
    ExactMutationAuthority {
        server_identity: SERVER,
        tool_name: name,
        schema_identity: SCHEMA,
        operation_class: OPERATION,
        session_identity: SESSION,
        host_admitted: true,
        basalt_admitted: true,
    }
}

#[test]
fn declared_read_only_is_the_only_default_exposure() {
    // r[verify animus.mcp.exposure.readonly]
    // r[verify animus.mcp.exposure.mutating]
    const READ: &str = "read";
    let descriptors = [
        descriptor(READ, ReadOnlyHint::DeclaredTrue),
        descriptor("false", ReadOnlyHint::DeclaredFalse),
        descriptor("missing", ReadOnlyHint::Missing),
        descriptor("malformed", ReadOnlyHint::Malformed),
        descriptor("unknown", ReadOnlyHint::Unknown),
    ];
    let catalog = reduce_mcp_exposure(McpExposureRequest {
        session_identity: SESSION,
        generation: GENERATION,
        descriptors: &descriptors,
        mutation_authorities: &[],
        built_in_sources: &[],
    });
    assert_eq!(catalog.exposed.len(), 1);
    assert_eq!(catalog.exposed[0].tool_name, READ);
    assert_eq!(
        catalog.rejected.len(),
        descriptors.len() - catalog.exposed.len()
    );
    assert!(
        catalog
            .rejected
            .iter()
            .all(|item| item.denial == ExposureDenial::AuthorityDenied)
    );
}

#[test]
fn exact_host_and_basalt_authority_admits_only_one_mutating_tool() {
    // r[verify animus.mcp.authority.exact]
    // r[verify animus.mcp.authority.non_claim]
    const ADMITTED: &str = "write.admitted";
    let descriptors = [
        descriptor(ADMITTED, ReadOnlyHint::DeclaredFalse),
        descriptor("write.denied", ReadOnlyHint::DeclaredFalse),
    ];
    let authority = [exact_authority(ADMITTED)];
    let catalog = reduce_mcp_exposure(McpExposureRequest {
        session_identity: SESSION,
        generation: GENERATION,
        descriptors: &descriptors,
        mutation_authorities: &authority,
        built_in_sources: &[],
    });
    assert_eq!(catalog.exposed.len(), 1);
    assert_eq!(catalog.exposed[0].tool_name, ADMITTED);
    assert!(catalog.exposed[0].mutation_authorized);
    assert!(catalog.receipt.hint_is_untrusted_metadata);
    assert!(!catalog.receipt.claims_side_effect_freedom);
}

#[test]
fn collisions_reject_every_entry_and_name_every_source() {
    // r[verify animus.mcp.collision.duplicate]
    const NAME: &str = "search";
    let mut other = descriptor(NAME, ReadOnlyHint::DeclaredTrue);
    other.server_identity = "server.beta";
    let descriptors = [descriptor(NAME, ReadOnlyHint::DeclaredTrue), other];
    let built_ins = [BuiltInToolSource {
        tool_name: NAME,
        source_label: "builtin.catalog",
    }];
    let catalog = reduce_mcp_exposure(McpExposureRequest {
        session_identity: SESSION,
        generation: GENERATION,
        descriptors: &descriptors,
        mutation_authorities: &[],
        built_in_sources: &built_ins,
    });
    assert!(catalog.exposed.is_empty());
    assert_eq!(catalog.collisions.len(), 1);
    assert_eq!(
        catalog.collisions[0].source_labels,
        ["builtin.catalog", SERVER, "server.beta"]
    );
    assert!(
        catalog
            .rejected
            .iter()
            .all(|item| item.denial == ExposureDenial::NameCollision)
    );
}

#[test]
fn child_projection_is_an_exact_current_generation_intersection() {
    // r[verify animus.mcp.child.intersection]
    const ALLOWED: &str = "read.allowed";
    const DENIED: &str = "read.denied";
    let descriptors = [
        descriptor(ALLOWED, ReadOnlyHint::DeclaredTrue),
        descriptor(DENIED, ReadOnlyHint::DeclaredTrue),
    ];
    let root = reduce_mcp_exposure(McpExposureRequest {
        session_identity: SESSION,
        generation: GENERATION,
        descriptors: &descriptors,
        mutation_authorities: &[],
        built_in_sources: &[],
    });
    let child = project_child_mcp_exposure(
        &root,
        GENERATION,
        &[ChildToolAuthority {
            tool_name: ALLOWED,
            schema_identity: SCHEMA,
            admitted: true,
        }],
    );
    assert_eq!(child.len(), 1);
    assert_eq!(child[0].tool_name, ALLOWED);
    assert!(project_child_mcp_exposure(&root, GENERATION + 1, &[]).is_empty());
}
