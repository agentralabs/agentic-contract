//! MCP resource definitions for AgenticContract.

/// Number of resource definitions.
pub const RESOURCE_COUNT: usize = 12;

/// A resource definition.
pub struct ResourceDefinition {
    /// Resource URI.
    pub uri: &'static str,
    /// Resource name.
    pub name: &'static str,
    /// Resource description.
    pub description: &'static str,
    /// MIME type.
    pub mime_type: &'static str,
}

/// All AgenticContract MCP resources.
pub const RESOURCES: &[ResourceDefinition] = &[
    ResourceDefinition {
        uri: "acon://policy/",
        name: "All Policies",
        description: "List of all policies",
        mime_type: "application/json",
    },
    ResourceDefinition {
        uri: "acon://policy/active",
        name: "Active Policies",
        description: "Currently active policies",
        mime_type: "application/json",
    },
    ResourceDefinition {
        uri: "acon://risk-limit/",
        name: "All Risk Limits",
        description: "List of all risk limits",
        mime_type: "application/json",
    },
    ResourceDefinition {
        uri: "acon://risk-limit/exceeded",
        name: "Exceeded Risk Limits",
        description: "Risk limits at or above threshold",
        mime_type: "application/json",
    },
    ResourceDefinition {
        uri: "acon://approval/",
        name: "All Approval Requests",
        description: "List of all approval requests",
        mime_type: "application/json",
    },
    ResourceDefinition {
        uri: "acon://approval/pending",
        name: "Pending Approvals",
        description: "Approval requests awaiting decision",
        mime_type: "application/json",
    },
    ResourceDefinition {
        uri: "acon://obligation/",
        name: "All Obligations",
        description: "List of all obligations",
        mime_type: "application/json",
    },
    ResourceDefinition {
        uri: "acon://obligation/pending",
        name: "Pending Obligations",
        description: "Obligations not yet fulfilled",
        mime_type: "application/json",
    },
    ResourceDefinition {
        uri: "acon://violation/",
        name: "All Violations",
        description: "List of all recorded violations",
        mime_type: "application/json",
    },
    ResourceDefinition {
        uri: "acon://violation/critical",
        name: "Critical Violations",
        description: "Violations with critical or fatal severity",
        mime_type: "application/json",
    },
    ResourceDefinition {
        uri: "acon://condition/",
        name: "All Conditions",
        description: "List of all conditions",
        mime_type: "application/json",
    },
    ResourceDefinition {
        uri: "acon://stats",
        name: "Contract Statistics",
        description: "Overall contract statistics",
        mime_type: "application/json",
    },
];

/// List all resources.
pub fn list_resources() -> &'static [ResourceDefinition] {
    RESOURCES
}
