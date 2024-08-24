use crate::analysis::state::AnalysisState;
use crate::autonodes::any::AnyNodeRef;
use crate::autonodes::comment::CommentNode;
use crate::autonodes::dynamic_variable_name::DynamicVariableNameNode;
use crate::autonodes::text_interpolation::TextInterpolationNode;
use crate::autonodes::variable_name::VariableNameNode;
use crate::autotree::NodeAccess;
use crate::autotree::NodeParser;
use crate::autotree::ParseError;
use crate::errornode::ErrorNode;
use crate::extra::ExtraChild;
use crate::issue::IssueEmitter;
use crate::parser::Range;
use crate::types::union::PHPType;
use crate::value::PHPValue;
use tree_sitter::Node;

#[derive(Debug, Clone)]
pub enum GlobalDeclarationChildren {
    DynamicVariableName(Box<DynamicVariableNameNode>),
    VariableName(Box<VariableNameNode>),
    Extra(ExtraChild),
}

impl NodeParser for GlobalDeclarationChildren {
    fn parse(node: Node, source: &[u8]) -> Result<Self, ParseError> {
        Ok(match node.kind() {
            "comment" => GlobalDeclarationChildren::Extra(ExtraChild::Comment(Box::new(
                CommentNode::parse(node, source)?,
            ))),
            "text_interpolation" => {
                GlobalDeclarationChildren::Extra(ExtraChild::TextInterpolation(Box::new(
                    TextInterpolationNode::parse(node, source)?,
                )))
            }
            "ERROR" => GlobalDeclarationChildren::Extra(ExtraChild::Error(Box::new(
                ErrorNode::parse(node, source)?,
            ))),
            "dynamic_variable_name" => GlobalDeclarationChildren::DynamicVariableName(Box::new(
                DynamicVariableNameNode::parse(node, source)?,
            )),
            "variable_name" => GlobalDeclarationChildren::VariableName(Box::new(
                VariableNameNode::parse(node, source)?,
            )),

            _ => {
                return Err(ParseError::new(
                    node.range(),
                    format!(
                        "GlobalDeclarationChildren: Parse error, unexpected node-type: {}",
                        node.kind()
                    ),
                ))
            }
        })
    }
}

impl GlobalDeclarationChildren {
    pub fn parse_opt(node: Node, source: &[u8]) -> Result<Option<Self>, ParseError> {
        Ok(Some(match node.kind() {
            "comment" => GlobalDeclarationChildren::Extra(ExtraChild::Comment(Box::new(
                CommentNode::parse(node, source)?,
            ))),
            "text_interpolation" => {
                GlobalDeclarationChildren::Extra(ExtraChild::TextInterpolation(Box::new(
                    TextInterpolationNode::parse(node, source)?,
                )))
            }
            "ERROR" => GlobalDeclarationChildren::Extra(ExtraChild::Error(Box::new(
                ErrorNode::parse(node, source)?,
            ))),
            "dynamic_variable_name" => GlobalDeclarationChildren::DynamicVariableName(Box::new(
                DynamicVariableNameNode::parse(node, source)?,
            )),
            "variable_name" => GlobalDeclarationChildren::VariableName(Box::new(
                VariableNameNode::parse(node, source)?,
            )),

            _ => return Ok(None),
        }))
    }

    pub fn kind(&self) -> &'static str {
        match self {
            GlobalDeclarationChildren::Extra(y) => y.kind(),
            GlobalDeclarationChildren::DynamicVariableName(y) => y.kind(),
            GlobalDeclarationChildren::VariableName(y) => y.kind(),
        }
    }

    pub fn parse_vec<'a, I>(children: I, source: &[u8]) -> Result<Vec<Box<Self>>, ParseError>
    where
        I: Iterator<Item = Node<'a>>,
    {
        let mut res: Vec<Box<Self>> = vec![];
        for child in children {
            res.push(Box::new(Self::parse(child, source)?));
        }
        Ok(res)
    }

    pub fn get_utype(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<PHPType> {
        match self {
            GlobalDeclarationChildren::Extra(x) => x.get_utype(state, emitter),
            GlobalDeclarationChildren::DynamicVariableName(x) => x.get_utype(state, emitter),
            GlobalDeclarationChildren::VariableName(x) => x.get_utype(state, emitter),
        }
    }

    pub fn get_php_value(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<PHPValue> {
        match self {
            GlobalDeclarationChildren::Extra(x) => x.get_php_value(state, emitter),
            GlobalDeclarationChildren::DynamicVariableName(x) => x.get_php_value(state, emitter),
            GlobalDeclarationChildren::VariableName(x) => x.get_php_value(state, emitter),
        }
    }

    pub fn read_from(&self, state: &mut AnalysisState, emitter: &dyn IssueEmitter) {
        match self {
            GlobalDeclarationChildren::Extra(x) => x.read_from(state, emitter),
            GlobalDeclarationChildren::DynamicVariableName(x) => x.read_from(state, emitter),
            GlobalDeclarationChildren::VariableName(x) => x.read_from(state, emitter),
        }
    }
}

impl NodeAccess for GlobalDeclarationChildren {
    fn brief_desc(&self) -> String {
        match self {
            GlobalDeclarationChildren::Extra(x) => {
                format!("GlobalDeclarationChildren::extra({})", x.brief_desc())
            }
            GlobalDeclarationChildren::DynamicVariableName(x) => format!(
                "GlobalDeclarationChildren::dynamic_variable_name({})",
                x.brief_desc()
            ),
            GlobalDeclarationChildren::VariableName(x) => format!(
                "GlobalDeclarationChildren::variable_name({})",
                x.brief_desc()
            ),
        }
    }

    fn as_any(&self) -> AnyNodeRef<'_> {
        match self {
            GlobalDeclarationChildren::Extra(x) => x.as_any(),
            GlobalDeclarationChildren::DynamicVariableName(x) => x.as_any(),
            GlobalDeclarationChildren::VariableName(x) => x.as_any(),
        }
    }

    fn children_any(&self) -> Vec<AnyNodeRef<'_>> {
        match self {
            GlobalDeclarationChildren::Extra(x) => x.children_any(),
            GlobalDeclarationChildren::DynamicVariableName(x) => x.children_any(),
            GlobalDeclarationChildren::VariableName(x) => x.children_any(),
        }
    }

    fn range(&self) -> Range {
        match self {
            GlobalDeclarationChildren::Extra(x) => x.range(),
            GlobalDeclarationChildren::DynamicVariableName(x) => x.range(),
            GlobalDeclarationChildren::VariableName(x) => x.range(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct GlobalDeclarationNode {
    pub range: Range,
    pub children: Vec<Box<GlobalDeclarationChildren>>,
    pub extras: Vec<Box<ExtraChild>>,
}

impl NodeParser for GlobalDeclarationNode {
    fn parse(node: Node, source: &[u8]) -> Result<Self, ParseError> {
        let range: Range = node.range().into();
        if node.kind() != "global_declaration" {
            return Err(ParseError::new(range, format!("GlobalDeclarationNode: Node is of the wrong kind [{}] vs expected [global_declaration] on pos {}:{}", node.kind(), range.start_point.row+1, range.start_point.column)));
        }

        Ok(Self {
            range,
            children: GlobalDeclarationChildren::parse_vec(
                node.named_children(&mut node.walk())
                    .filter(|node| node.kind() != "comment"),
                source,
            )?,
            extras: ExtraChild::parse_vec(
                node.named_children(&mut node.walk())
                    .filter(|node| node.kind() == "comment"),
                source,
            )?,
        })
    }
}

impl GlobalDeclarationNode {
    pub fn kind(&self) -> &'static str {
        "global_declaration"
    }
}

impl NodeAccess for GlobalDeclarationNode {
    fn brief_desc(&self) -> String {
        "GlobalDeclarationNode".into()
    }

    fn as_any(&self) -> AnyNodeRef<'_> {
        AnyNodeRef::GlobalDeclaration(self)
    }

    #[allow(clippy::vec_init_then_push)]
    fn children_any<'a>(&'a self) -> Vec<AnyNodeRef<'a>> {
        let mut child_vec: Vec<AnyNodeRef<'a>> = vec![];

        // let any_children: Vec<AnyNodeRef<'a>> = self.children.iter().map(|x| x.as_any()).collect();
        child_vec.extend(self.children.iter().map(|n| n.as_any()));
        child_vec.extend(self.extras.iter().map(|n| n.as_any()));

        child_vec.sort_by(|a, b| a.range().start_byte.cmp(&b.range().start_byte));
        child_vec
    }

    fn range(&self) -> Range {
        self.range
    }
}
