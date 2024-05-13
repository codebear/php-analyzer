use crate::analysis::state::AnalysisState;
use crate::autonodes::any::AnyNodeRef;
use crate::autonodes::comment::CommentNode;
use crate::autonodes::enum_case::EnumCaseNode;
use crate::autonodes::method_declaration::MethodDeclarationNode;
use crate::autonodes::use_declaration::UseDeclarationNode;
use crate::autotree::NodeAccess;
use crate::autotree::NodeParser;
use crate::autotree::ParseError;
use crate::errornode::ErrorNode;
use crate::extra::ExtraChild;
use crate::issue::IssueEmitter;
use crate::parser::Range;
use crate::types::union::UnionType;
use crate::value::PHPValue;
use tree_sitter::Node;

#[derive(Debug, Clone)]
pub enum EnumDeclarationListChildren {
    EnumCase(Box<EnumCaseNode>),
    MethodDeclaration(Box<MethodDeclarationNode>),
    UseDeclaration(Box<UseDeclarationNode>),
    Extra(ExtraChild),
}

impl NodeParser for EnumDeclarationListChildren {
    fn parse(node: Node, source: &[u8]) -> Result<Self, ParseError> {
        Ok(match node.kind() {
            "comment" => EnumDeclarationListChildren::Extra(ExtraChild::Comment(Box::new(
                CommentNode::parse(node, source)?,
            ))),
            "ERROR" => EnumDeclarationListChildren::Extra(ExtraChild::Error(Box::new(
                ErrorNode::parse(node, source)?,
            ))),
            "enum_case" => {
                EnumDeclarationListChildren::EnumCase(Box::new(EnumCaseNode::parse(node, source)?))
            }
            "method_declaration" => EnumDeclarationListChildren::MethodDeclaration(Box::new(
                MethodDeclarationNode::parse(node, source)?,
            )),
            "use_declaration" => EnumDeclarationListChildren::UseDeclaration(Box::new(
                UseDeclarationNode::parse(node, source)?,
            )),

            _ => {
                return Err(ParseError::new(
                    node.range(),
                    format!("Parse error, unexpected node-type: {}", node.kind()),
                ))
            }
        })
    }
}

impl EnumDeclarationListChildren {
    pub fn parse_opt(node: Node, source: &[u8]) -> Result<Option<Self>, ParseError> {
        Ok(Some(match node.kind() {
            "comment" => EnumDeclarationListChildren::Extra(ExtraChild::Comment(Box::new(
                CommentNode::parse(node, source)?,
            ))),
            "ERROR" => EnumDeclarationListChildren::Extra(ExtraChild::Error(Box::new(
                ErrorNode::parse(node, source)?,
            ))),
            "enum_case" => {
                EnumDeclarationListChildren::EnumCase(Box::new(EnumCaseNode::parse(node, source)?))
            }
            "method_declaration" => EnumDeclarationListChildren::MethodDeclaration(Box::new(
                MethodDeclarationNode::parse(node, source)?,
            )),
            "use_declaration" => EnumDeclarationListChildren::UseDeclaration(Box::new(
                UseDeclarationNode::parse(node, source)?,
            )),

            _ => return Ok(None),
        }))
    }

    pub fn kind(&self) -> &'static str {
        match self {
            EnumDeclarationListChildren::Extra(y) => y.kind(),
            EnumDeclarationListChildren::EnumCase(y) => y.kind(),
            EnumDeclarationListChildren::MethodDeclaration(y) => y.kind(),
            EnumDeclarationListChildren::UseDeclaration(y) => y.kind(),
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
    ) -> Option<UnionType> {
        match self {
            EnumDeclarationListChildren::Extra(x) => x.get_utype(state, emitter),
            EnumDeclarationListChildren::EnumCase(x) => x.get_utype(state, emitter),
            EnumDeclarationListChildren::MethodDeclaration(x) => x.get_utype(state, emitter),
            EnumDeclarationListChildren::UseDeclaration(x) => x.get_utype(state, emitter),
        }
    }

    pub fn get_php_value(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<PHPValue> {
        match self {
            EnumDeclarationListChildren::Extra(x) => x.get_php_value(state, emitter),
            EnumDeclarationListChildren::EnumCase(x) => x.get_php_value(state, emitter),
            EnumDeclarationListChildren::MethodDeclaration(x) => x.get_php_value(state, emitter),
            EnumDeclarationListChildren::UseDeclaration(x) => x.get_php_value(state, emitter),
        }
    }

    pub fn read_from(&self, state: &mut AnalysisState, emitter: &dyn IssueEmitter) {
        match self {
            EnumDeclarationListChildren::Extra(x) => x.read_from(state, emitter),
            EnumDeclarationListChildren::EnumCase(x) => x.read_from(state, emitter),
            EnumDeclarationListChildren::MethodDeclaration(x) => x.read_from(state, emitter),
            EnumDeclarationListChildren::UseDeclaration(x) => x.read_from(state, emitter),
        }
    }
}

impl NodeAccess for EnumDeclarationListChildren {
    fn brief_desc(&self) -> String {
        match self {
            EnumDeclarationListChildren::Extra(x) => {
                format!("EnumDeclarationListChildren::extra({})", x.brief_desc())
            }
            EnumDeclarationListChildren::EnumCase(x) => {
                format!("EnumDeclarationListChildren::enum_case({})", x.brief_desc())
            }
            EnumDeclarationListChildren::MethodDeclaration(x) => format!(
                "EnumDeclarationListChildren::method_declaration({})",
                x.brief_desc()
            ),
            EnumDeclarationListChildren::UseDeclaration(x) => format!(
                "EnumDeclarationListChildren::use_declaration({})",
                x.brief_desc()
            ),
        }
    }

    fn as_any(&self) -> AnyNodeRef<'_> {
        match self {
            EnumDeclarationListChildren::Extra(x) => x.as_any(),
            EnumDeclarationListChildren::EnumCase(x) => x.as_any(),
            EnumDeclarationListChildren::MethodDeclaration(x) => x.as_any(),
            EnumDeclarationListChildren::UseDeclaration(x) => x.as_any(),
        }
    }

    fn children_any(&self) -> Vec<AnyNodeRef<'_>> {
        match self {
            EnumDeclarationListChildren::Extra(x) => x.children_any(),
            EnumDeclarationListChildren::EnumCase(x) => x.children_any(),
            EnumDeclarationListChildren::MethodDeclaration(x) => x.children_any(),
            EnumDeclarationListChildren::UseDeclaration(x) => x.children_any(),
        }
    }

    fn range(&self) -> Range {
        match self {
            EnumDeclarationListChildren::Extra(x) => x.range(),
            EnumDeclarationListChildren::EnumCase(x) => x.range(),
            EnumDeclarationListChildren::MethodDeclaration(x) => x.range(),
            EnumDeclarationListChildren::UseDeclaration(x) => x.range(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct EnumDeclarationListNode {
    pub range: Range,
    pub children: Vec<Box<EnumDeclarationListChildren>>,
    pub extras: Vec<Box<ExtraChild>>,
}

impl NodeParser for EnumDeclarationListNode {
    fn parse(node: Node, source: &[u8]) -> Result<Self, ParseError> {
        let range: Range = node.range().into();
        if node.kind() != "enum_declaration_list" {
            return Err(ParseError::new(range, format!("Node is of the wrong kind [{}] vs expected [enum_declaration_list] on pos {}:{}", node.kind(), range.start_point.row+1, range.start_point.column)));
        }

        Ok(Self {
            range,
            children: EnumDeclarationListChildren::parse_vec(
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

impl EnumDeclarationListNode {
    pub fn kind(&self) -> &'static str {
        "enum_declaration_list"
    }
}

impl NodeAccess for EnumDeclarationListNode {
    fn brief_desc(&self) -> String {
        "EnumDeclarationListNode".into()
    }

    fn as_any(&self) -> AnyNodeRef<'_> {
        AnyNodeRef::EnumDeclarationList(self)
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
