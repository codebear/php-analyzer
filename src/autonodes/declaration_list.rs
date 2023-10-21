use crate::analysis::state::AnalysisState;
use crate::autonodes::any::AnyNodeRef;
use crate::autonodes::comment::CommentNode;
use crate::autonodes::const_declaration::ConstDeclarationNode;
use crate::autonodes::method_declaration::MethodDeclarationNode;
use crate::autonodes::property_declaration::PropertyDeclarationNode;
use crate::autonodes::text_interpolation::TextInterpolationNode;
use crate::autonodes::use_declaration::UseDeclarationNode;
use crate::autotree::NodeAccess;
use crate::autotree::ParseError;
use crate::errornode::ErrorNode;
use crate::extra::ExtraChild;
use crate::issue::IssueEmitter;
use crate::types::union::UnionType;
use crate::value::PHPValue;
use tree_sitter::Node;
use tree_sitter::Range;

#[derive(Debug, Clone)]
pub enum DeclarationListChildren {
    ConstDeclaration(Box<ConstDeclarationNode>),
    MethodDeclaration(Box<MethodDeclarationNode>),
    PropertyDeclaration(Box<PropertyDeclarationNode>),
    UseDeclaration(Box<UseDeclarationNode>),
    Extra(ExtraChild),
}

impl DeclarationListChildren {
    pub fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        Ok(match node.kind() {
            "comment" => DeclarationListChildren::Extra(ExtraChild::Comment(Box::new(
                CommentNode::parse(node, source)?,
            ))),
            "text_interpolation" => DeclarationListChildren::Extra(ExtraChild::TextInterpolation(
                Box::new(TextInterpolationNode::parse(node, source)?),
            )),
            "ERROR" => DeclarationListChildren::Extra(ExtraChild::Error(Box::new(
                ErrorNode::parse(node, source)?,
            ))),
            "const_declaration" => DeclarationListChildren::ConstDeclaration(Box::new(
                ConstDeclarationNode::parse(node, source)?,
            )),
            "method_declaration" => DeclarationListChildren::MethodDeclaration(Box::new(
                MethodDeclarationNode::parse(node, source)?,
            )),
            "property_declaration" => DeclarationListChildren::PropertyDeclaration(Box::new(
                PropertyDeclarationNode::parse(node, source)?,
            )),
            "use_declaration" => DeclarationListChildren::UseDeclaration(Box::new(
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

    pub fn parse_opt(node: Node, source: &Vec<u8>) -> Result<Option<Self>, ParseError> {
        Ok(Some(match node.kind() {
            "comment" => DeclarationListChildren::Extra(ExtraChild::Comment(Box::new(
                CommentNode::parse(node, source)?,
            ))),
            "text_interpolation" => DeclarationListChildren::Extra(ExtraChild::TextInterpolation(
                Box::new(TextInterpolationNode::parse(node, source)?),
            )),
            "ERROR" => DeclarationListChildren::Extra(ExtraChild::Error(Box::new(
                ErrorNode::parse(node, source)?,
            ))),
            "const_declaration" => DeclarationListChildren::ConstDeclaration(Box::new(
                ConstDeclarationNode::parse(node, source)?,
            )),
            "method_declaration" => DeclarationListChildren::MethodDeclaration(Box::new(
                MethodDeclarationNode::parse(node, source)?,
            )),
            "property_declaration" => DeclarationListChildren::PropertyDeclaration(Box::new(
                PropertyDeclarationNode::parse(node, source)?,
            )),
            "use_declaration" => DeclarationListChildren::UseDeclaration(Box::new(
                UseDeclarationNode::parse(node, source)?,
            )),

            _ => return Ok(None),
        }))
    }

    pub fn kind(&self) -> &'static str {
        match self {
            DeclarationListChildren::Extra(y) => y.kind(),
            DeclarationListChildren::ConstDeclaration(y) => y.kind(),
            DeclarationListChildren::MethodDeclaration(y) => y.kind(),
            DeclarationListChildren::PropertyDeclaration(y) => y.kind(),
            DeclarationListChildren::UseDeclaration(y) => y.kind(),
        }
    }

    pub fn parse_vec<'a, I>(children: I, source: &Vec<u8>) -> Result<Vec<Box<Self>>, ParseError>
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
            DeclarationListChildren::Extra(x) => x.get_utype(state, emitter),
            DeclarationListChildren::ConstDeclaration(x) => x.get_utype(state, emitter),
            DeclarationListChildren::MethodDeclaration(x) => x.get_utype(state, emitter),
            DeclarationListChildren::PropertyDeclaration(x) => x.get_utype(state, emitter),
            DeclarationListChildren::UseDeclaration(x) => x.get_utype(state, emitter),
        }
    }

    pub fn get_php_value(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<PHPValue> {
        match self {
            DeclarationListChildren::Extra(x) => x.get_php_value(state, emitter),
            DeclarationListChildren::ConstDeclaration(x) => x.get_php_value(state, emitter),
            DeclarationListChildren::MethodDeclaration(x) => x.get_php_value(state, emitter),
            DeclarationListChildren::PropertyDeclaration(x) => x.get_php_value(state, emitter),
            DeclarationListChildren::UseDeclaration(x) => x.get_php_value(state, emitter),
        }
    }

    pub fn read_from(&self, state: &mut AnalysisState, emitter: &dyn IssueEmitter) {
        match self {
            DeclarationListChildren::Extra(x) => x.read_from(state, emitter),
            DeclarationListChildren::ConstDeclaration(x) => x.read_from(state, emitter),
            DeclarationListChildren::MethodDeclaration(x) => x.read_from(state, emitter),
            DeclarationListChildren::PropertyDeclaration(x) => x.read_from(state, emitter),
            DeclarationListChildren::UseDeclaration(x) => x.read_from(state, emitter),
        }
    }
}

impl NodeAccess for DeclarationListChildren {
    fn brief_desc(&self) -> String {
        match self {
            DeclarationListChildren::Extra(x) => {
                format!("DeclarationListChildren::extra({})", x.brief_desc())
            }
            DeclarationListChildren::ConstDeclaration(x) => format!(
                "DeclarationListChildren::const_declaration({})",
                x.brief_desc()
            ),
            DeclarationListChildren::MethodDeclaration(x) => format!(
                "DeclarationListChildren::method_declaration({})",
                x.brief_desc()
            ),
            DeclarationListChildren::PropertyDeclaration(x) => format!(
                "DeclarationListChildren::property_declaration({})",
                x.brief_desc()
            ),
            DeclarationListChildren::UseDeclaration(x) => format!(
                "DeclarationListChildren::use_declaration({})",
                x.brief_desc()
            ),
        }
    }

    fn as_any<'a>(&'a self) -> AnyNodeRef<'a> {
        match self {
            DeclarationListChildren::Extra(x) => x.as_any(),
            DeclarationListChildren::ConstDeclaration(x) => x.as_any(),
            DeclarationListChildren::MethodDeclaration(x) => x.as_any(),
            DeclarationListChildren::PropertyDeclaration(x) => x.as_any(),
            DeclarationListChildren::UseDeclaration(x) => x.as_any(),
        }
    }

    fn children_any<'a>(&'a self) -> Vec<AnyNodeRef<'a>> {
        match self {
            DeclarationListChildren::Extra(x) => x.children_any(),
            DeclarationListChildren::ConstDeclaration(x) => x.children_any(),
            DeclarationListChildren::MethodDeclaration(x) => x.children_any(),
            DeclarationListChildren::PropertyDeclaration(x) => x.children_any(),
            DeclarationListChildren::UseDeclaration(x) => x.children_any(),
        }
    }

    fn range(&self) -> Range {
        match self {
            DeclarationListChildren::Extra(x) => x.range(),
            DeclarationListChildren::ConstDeclaration(x) => x.range(),
            DeclarationListChildren::MethodDeclaration(x) => x.range(),
            DeclarationListChildren::PropertyDeclaration(x) => x.range(),
            DeclarationListChildren::UseDeclaration(x) => x.range(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct DeclarationListNode {
    pub range: Range,
    pub children: Vec<Box<DeclarationListChildren>>,
    pub extras: Vec<Box<ExtraChild>>,
}

impl DeclarationListNode {
    pub fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        let range = node.range();
        if node.kind() != "declaration_list" {
            return Err(ParseError::new(
                range,
                format!(
                    "Node is of the wrong kind [{}] vs expected [declaration_list] on pos {}:{}",
                    node.kind(),
                    range.start_point.row + 1,
                    range.start_point.column
                ),
            ));
        }

        Ok(Self {
            range,
            children: DeclarationListChildren::parse_vec(
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

    pub fn parse_vec<'a, I>(children: I, source: &Vec<u8>) -> Result<Vec<Box<Self>>, ParseError>
    where
        I: Iterator<Item = Node<'a>>,
    {
        let mut res: Vec<Box<Self>> = vec![];
        for child in children {
            if child.kind() == "comment" {
                continue;
            }
            res.push(Box::new(Self::parse(child, source)?));
        }
        Ok(res)
    }

    pub fn kind(&self) -> &'static str {
        "declaration_list"
    }
}

impl NodeAccess for DeclarationListNode {
    fn brief_desc(&self) -> String {
        "DeclarationListNode".into()
    }

    fn as_any<'a>(&'a self) -> AnyNodeRef<'a> {
        AnyNodeRef::DeclarationList(self)
    }

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
