#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlanResourcesInput {
    #[prost(string, tag = "1")]
    pub request_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub action: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub principal: ::core::option::Option<Principal>,
    #[prost(message, optional, tag = "4")]
    pub resource: ::core::option::Option<plan_resources_input::Resource>,
    #[prost(message, optional, tag = "5")]
    pub aux_data: ::core::option::Option<AuxData>,
    #[prost(bool, tag = "6")]
    pub include_meta: bool,
}
/// Nested message and enum types in `PlanResourcesInput`.
pub mod plan_resources_input {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Resource {
        #[prost(string, tag = "1")]
        pub kind: ::prost::alloc::string::String,
        #[prost(map = "string, message", tag = "2")]
        pub attr: ::std::collections::HashMap<
            ::prost::alloc::string::String,
            ::prost_types::Value,
        >,
        #[prost(string, tag = "3")]
        pub policy_version: ::prost::alloc::string::String,
        #[prost(string, tag = "4")]
        pub scope: ::prost::alloc::string::String,
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlanResourcesAst {
    #[prost(message, optional, tag = "1")]
    pub filter_ast: ::core::option::Option<plan_resources_ast::Node>,
}
/// Nested message and enum types in `PlanResourcesAst`.
pub mod plan_resources_ast {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Node {
        #[prost(oneof = "node::Node", tags = "1, 2")]
        pub node: ::core::option::Option<node::Node>,
    }
    /// Nested message and enum types in `Node`.
    pub mod node {
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Node {
            #[prost(message, tag = "1")]
            LogicalOperation(super::LogicalOperation),
            #[prost(message, tag = "2")]
            Expression(
                super::super::super::super::super::google::api::expr::v1alpha1::CheckedExpr,
            ),
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct LogicalOperation {
        #[prost(enumeration = "logical_operation::Operator", tag = "1")]
        pub operator: i32,
        #[prost(message, repeated, tag = "2")]
        pub nodes: ::prost::alloc::vec::Vec<Node>,
    }
    /// Nested message and enum types in `LogicalOperation`.
    pub mod logical_operation {
        #[derive(
            Clone,
            Copy,
            Debug,
            PartialEq,
            Eq,
            Hash,
            PartialOrd,
            Ord,
            ::prost::Enumeration
        )]
        #[repr(i32)]
        pub enum Operator {
            Unspecified = 0,
            And = 1,
            Or = 2,
            Not = 3,
        }
        impl Operator {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    Operator::Unspecified => "OPERATOR_UNSPECIFIED",
                    Operator::And => "OPERATOR_AND",
                    Operator::Or => "OPERATOR_OR",
                    Operator::Not => "OPERATOR_NOT",
                }
            }
            /// Creates an enum from field names used in the ProtoBuf definition.
            pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                match value {
                    "OPERATOR_UNSPECIFIED" => Some(Self::Unspecified),
                    "OPERATOR_AND" => Some(Self::And),
                    "OPERATOR_OR" => Some(Self::Or),
                    "OPERATOR_NOT" => Some(Self::Not),
                    _ => None,
                }
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlanResourcesFilter {
    #[prost(enumeration = "plan_resources_filter::Kind", tag = "1")]
    pub kind: i32,
    #[prost(message, optional, tag = "2")]
    pub condition: ::core::option::Option<plan_resources_filter::expression::Operand>,
}
/// Nested message and enum types in `PlanResourcesFilter`.
pub mod plan_resources_filter {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Expression {
        #[prost(string, tag = "1")]
        pub operator: ::prost::alloc::string::String,
        #[prost(message, repeated, tag = "2")]
        pub operands: ::prost::alloc::vec::Vec<expression::Operand>,
    }
    /// Nested message and enum types in `Expression`.
    pub mod expression {
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Operand {
            #[prost(oneof = "operand::Node", tags = "1, 2, 3")]
            pub node: ::core::option::Option<operand::Node>,
        }
        /// Nested message and enum types in `Operand`.
        pub mod operand {
            #[allow(clippy::derive_partial_eq_without_eq)]
            #[derive(Clone, PartialEq, ::prost::Oneof)]
            pub enum Node {
                #[prost(message, tag = "1")]
                Value(::prost_types::Value),
                #[prost(message, tag = "2")]
                Expression(super::super::Expression),
                #[prost(string, tag = "3")]
                Variable(::prost::alloc::string::String),
            }
        }
    }
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum Kind {
        Unspecified = 0,
        AlwaysAllowed = 1,
        AlwaysDenied = 2,
        Conditional = 3,
    }
    impl Kind {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Kind::Unspecified => "KIND_UNSPECIFIED",
                Kind::AlwaysAllowed => "KIND_ALWAYS_ALLOWED",
                Kind::AlwaysDenied => "KIND_ALWAYS_DENIED",
                Kind::Conditional => "KIND_CONDITIONAL",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "KIND_UNSPECIFIED" => Some(Self::Unspecified),
                "KIND_ALWAYS_ALLOWED" => Some(Self::AlwaysAllowed),
                "KIND_ALWAYS_DENIED" => Some(Self::AlwaysDenied),
                "KIND_CONDITIONAL" => Some(Self::Conditional),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlanResourcesOutput {
    #[prost(string, tag = "1")]
    pub request_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub action: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub kind: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub policy_version: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub scope: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "6")]
    pub filter: ::core::option::Option<PlanResourcesFilter>,
    #[prost(string, tag = "7")]
    pub filter_debug: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "8")]
    pub validation_errors: ::prost::alloc::vec::Vec<
        super::super::schema::v1::ValidationError,
    >,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CheckInput {
    #[prost(string, tag = "1")]
    pub request_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub resource: ::core::option::Option<Resource>,
    #[prost(message, optional, tag = "3")]
    pub principal: ::core::option::Option<Principal>,
    #[prost(string, repeated, tag = "4")]
    pub actions: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "5")]
    pub aux_data: ::core::option::Option<AuxData>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CheckOutput {
    #[prost(string, tag = "1")]
    pub request_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub resource_id: ::prost::alloc::string::String,
    #[prost(map = "string, message", tag = "3")]
    pub actions: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        check_output::ActionEffect,
    >,
    #[prost(string, repeated, tag = "4")]
    pub effective_derived_roles: ::prost::alloc::vec::Vec<
        ::prost::alloc::string::String,
    >,
    #[prost(message, repeated, tag = "5")]
    pub validation_errors: ::prost::alloc::vec::Vec<
        super::super::schema::v1::ValidationError,
    >,
    #[prost(message, repeated, tag = "6")]
    pub outputs: ::prost::alloc::vec::Vec<OutputEntry>,
}
/// Nested message and enum types in `CheckOutput`.
pub mod check_output {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ActionEffect {
        #[prost(enumeration = "super::super::super::effect::v1::Effect", tag = "1")]
        pub effect: i32,
        #[prost(string, tag = "2")]
        pub policy: ::prost::alloc::string::String,
        #[prost(string, tag = "3")]
        pub scope: ::prost::alloc::string::String,
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OutputEntry {
    #[prost(string, tag = "1")]
    pub src: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub val: ::core::option::Option<::prost_types::Value>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Resource {
    #[prost(string, tag = "1")]
    pub kind: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub policy_version: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub id: ::prost::alloc::string::String,
    #[prost(map = "string, message", tag = "4")]
    pub attr: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost_types::Value,
    >,
    #[prost(string, tag = "5")]
    pub scope: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Principal {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub policy_version: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "3")]
    pub roles: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(map = "string, message", tag = "4")]
    pub attr: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost_types::Value,
    >,
    #[prost(string, tag = "5")]
    pub scope: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuxData {
    #[prost(map = "string, message", tag = "1")]
    pub jwt: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost_types::Value,
    >,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Trace {
    #[prost(message, repeated, tag = "1")]
    pub components: ::prost::alloc::vec::Vec<trace::Component>,
    #[prost(message, optional, tag = "2")]
    pub event: ::core::option::Option<trace::Event>,
}
/// Nested message and enum types in `Trace`.
pub mod trace {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Component {
        #[prost(enumeration = "component::Kind", tag = "1")]
        pub kind: i32,
        #[prost(oneof = "component::Details", tags = "2, 3, 4, 5, 6, 7, 8, 9, 10, 11")]
        pub details: ::core::option::Option<component::Details>,
    }
    /// Nested message and enum types in `Component`.
    pub mod component {
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Variable {
            #[prost(string, tag = "1")]
            pub name: ::prost::alloc::string::String,
            #[prost(string, tag = "2")]
            pub expr: ::prost::alloc::string::String,
        }
        #[derive(
            Clone,
            Copy,
            Debug,
            PartialEq,
            Eq,
            Hash,
            PartialOrd,
            Ord,
            ::prost::Enumeration
        )]
        #[repr(i32)]
        pub enum Kind {
            Unspecified = 0,
            Action = 1,
            ConditionAll = 2,
            ConditionAny = 3,
            ConditionNone = 4,
            Condition = 5,
            DerivedRole = 6,
            Expr = 7,
            Policy = 8,
            Resource = 9,
            Rule = 10,
            Scope = 11,
            Variable = 12,
            Variables = 13,
            Output = 14,
        }
        impl Kind {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    Kind::Unspecified => "KIND_UNSPECIFIED",
                    Kind::Action => "KIND_ACTION",
                    Kind::ConditionAll => "KIND_CONDITION_ALL",
                    Kind::ConditionAny => "KIND_CONDITION_ANY",
                    Kind::ConditionNone => "KIND_CONDITION_NONE",
                    Kind::Condition => "KIND_CONDITION",
                    Kind::DerivedRole => "KIND_DERIVED_ROLE",
                    Kind::Expr => "KIND_EXPR",
                    Kind::Policy => "KIND_POLICY",
                    Kind::Resource => "KIND_RESOURCE",
                    Kind::Rule => "KIND_RULE",
                    Kind::Scope => "KIND_SCOPE",
                    Kind::Variable => "KIND_VARIABLE",
                    Kind::Variables => "KIND_VARIABLES",
                    Kind::Output => "KIND_OUTPUT",
                }
            }
            /// Creates an enum from field names used in the ProtoBuf definition.
            pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                match value {
                    "KIND_UNSPECIFIED" => Some(Self::Unspecified),
                    "KIND_ACTION" => Some(Self::Action),
                    "KIND_CONDITION_ALL" => Some(Self::ConditionAll),
                    "KIND_CONDITION_ANY" => Some(Self::ConditionAny),
                    "KIND_CONDITION_NONE" => Some(Self::ConditionNone),
                    "KIND_CONDITION" => Some(Self::Condition),
                    "KIND_DERIVED_ROLE" => Some(Self::DerivedRole),
                    "KIND_EXPR" => Some(Self::Expr),
                    "KIND_POLICY" => Some(Self::Policy),
                    "KIND_RESOURCE" => Some(Self::Resource),
                    "KIND_RULE" => Some(Self::Rule),
                    "KIND_SCOPE" => Some(Self::Scope),
                    "KIND_VARIABLE" => Some(Self::Variable),
                    "KIND_VARIABLES" => Some(Self::Variables),
                    "KIND_OUTPUT" => Some(Self::Output),
                    _ => None,
                }
            }
        }
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Details {
            #[prost(string, tag = "2")]
            Action(::prost::alloc::string::String),
            #[prost(string, tag = "3")]
            DerivedRole(::prost::alloc::string::String),
            #[prost(string, tag = "4")]
            Expr(::prost::alloc::string::String),
            #[prost(uint32, tag = "5")]
            Index(u32),
            #[prost(string, tag = "6")]
            Policy(::prost::alloc::string::String),
            #[prost(string, tag = "7")]
            Resource(::prost::alloc::string::String),
            #[prost(string, tag = "8")]
            Rule(::prost::alloc::string::String),
            #[prost(string, tag = "9")]
            Scope(::prost::alloc::string::String),
            #[prost(message, tag = "10")]
            Variable(Variable),
            #[prost(string, tag = "11")]
            Output(::prost::alloc::string::String),
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Event {
        #[prost(enumeration = "event::Status", tag = "1")]
        pub status: i32,
        #[prost(enumeration = "super::super::super::effect::v1::Effect", tag = "2")]
        pub effect: i32,
        #[prost(string, tag = "3")]
        pub error: ::prost::alloc::string::String,
        #[prost(string, tag = "4")]
        pub message: ::prost::alloc::string::String,
        #[prost(message, optional, tag = "5")]
        pub result: ::core::option::Option<::prost_types::Value>,
    }
    /// Nested message and enum types in `Event`.
    pub mod event {
        #[derive(
            Clone,
            Copy,
            Debug,
            PartialEq,
            Eq,
            Hash,
            PartialOrd,
            Ord,
            ::prost::Enumeration
        )]
        #[repr(i32)]
        pub enum Status {
            Unspecified = 0,
            Activated = 1,
            Skipped = 2,
        }
        impl Status {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    Status::Unspecified => "STATUS_UNSPECIFIED",
                    Status::Activated => "STATUS_ACTIVATED",
                    Status::Skipped => "STATUS_SKIPPED",
                }
            }
            /// Creates an enum from field names used in the ProtoBuf definition.
            pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                match value {
                    "STATUS_UNSPECIFIED" => Some(Self::Unspecified),
                    "STATUS_ACTIVATED" => Some(Self::Activated),
                    "STATUS_SKIPPED" => Some(Self::Skipped),
                    _ => None,
                }
            }
        }
    }
}
