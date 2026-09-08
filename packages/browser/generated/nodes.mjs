// Generated from Rust node payload types. Do not edit.
import {fields,string,boolean,nullable,oneOf} from '../fields.mjs'
export const names = Object.freeze({"Blockquote":"markdown_commonmark_contracts::nodes::Blockquote","CodeBlock":"markdown_commonmark_contracts::nodes::CodeBlock","Emphasis":"markdown_commonmark_contracts::nodes::Emphasis","Hardbreak":"markdown_commonmark_contracts::nodes::Hardbreak","Heading":"markdown_commonmark_contracts::nodes::Heading","HtmlBlock":"markdown_commonmark_contracts::nodes::HtmlBlock","HtmlInline":"markdown_commonmark_contracts::nodes::HtmlInline","Image":"markdown_commonmark_contracts::nodes::Image","InlineCode":"markdown_commonmark_contracts::nodes::InlineCode","Link":"markdown_commonmark_contracts::nodes::Link","List":"markdown_commonmark_contracts::nodes::List","ListItem":"markdown_commonmark_contracts::nodes::ListItem","Paragraph":"markdown_commonmark_contracts::nodes::Paragraph","Softbreak":"markdown_commonmark_contracts::nodes::Softbreak","Strong":"markdown_commonmark_contracts::nodes::Strong","Text":"markdown_commonmark_contracts::nodes::Text","ThematicBreak":"markdown_commonmark_contracts::nodes::ThematicBreak","Mark":"markdown_generic_contracts::mark::MarkData","BlockMath":"markdown_generic_contracts::math::BlockMathData","InlineMath":"markdown_generic_contracts::math::InlineMathData","Strikethrough":"markdown_generic_contracts::strikethrough::StrikethroughData","Cell":"markdown_generic_contracts::table::CellData","Row":"markdown_generic_contracts::table::RowData","Table":"markdown_generic_contracts::table::TableData","BlankLine":"markdown_trap_contracts::compat::BlankLineData","Reference":"markdown_trap_contracts::reference::ReferenceData","Spoiler":"markdown_trap_contracts::spoiler::SpoilerData","Stamp":"markdown_trap_contracts::stamp::StampData"})
const validators = new Map([
  ["markdown_commonmark_contracts::nodes::Blockquote",value => fields(value,{},{})],
  ["markdown_commonmark_contracts::nodes::CodeBlock",value => fields(value,{"fenced":boolean,"info":string,"literal":string},{})],
  ["markdown_commonmark_contracts::nodes::Emphasis",value => fields(value,{},{})],
  ["markdown_commonmark_contracts::nodes::Hardbreak",value => fields(value,{},{})],
  ["markdown_commonmark_contracts::nodes::Heading",value => fields(value,{"level":(value) => Number.isInteger(value) && value >= 0 && value <= 255},{})],
  ["markdown_commonmark_contracts::nodes::HtmlBlock",value => fields(value,{"literal":string},{})],
  ["markdown_commonmark_contracts::nodes::HtmlInline",value => fields(value,{"literal":string},{})],
  ["markdown_commonmark_contracts::nodes::Image",value => fields(value,{"destination":string,"label_source":string,"title":nullable(string)},{})],
  ["markdown_commonmark_contracts::nodes::InlineCode",value => fields(value,{"literal":string},{})],
  ["markdown_commonmark_contracts::nodes::Link",value => fields(value,{"destination":string,"form":oneOf("explicit","autolink","linkify"),"title":nullable(string)},{})],
  ["markdown_commonmark_contracts::nodes::List",value => fields(value,{"ordered":boolean,"start":(value) => Number.isInteger(value) && value >= 0 && value <= 4294967295,"tight":boolean},{})],
  ["markdown_commonmark_contracts::nodes::ListItem",value => fields(value,{"marker":string},{})],
  ["markdown_commonmark_contracts::nodes::Paragraph",value => fields(value,{},{})],
  ["markdown_commonmark_contracts::nodes::Softbreak",value => fields(value,{},{})],
  ["markdown_commonmark_contracts::nodes::Strong",value => fields(value,{},{})],
  ["markdown_commonmark_contracts::nodes::Text",value => fields(value,{"value":string},{})],
  ["markdown_commonmark_contracts::nodes::ThematicBreak",value => fields(value,{"marker":string},{})],
  ["markdown_generic_contracts::mark::MarkData",value => fields(value,{},{})],
  ["markdown_generic_contracts::math::BlockMathData",value => fields(value,{"tex":string},{})],
  ["markdown_generic_contracts::math::InlineMathData",value => fields(value,{"tex":string},{})],
  ["markdown_generic_contracts::strikethrough::StrikethroughData",value => fields(value,{},{})],
  ["markdown_generic_contracts::table::CellData",value => fields(value,{"alignment":nullable(oneOf("left","center","right"))},{})],
  ["markdown_generic_contracts::table::RowData",value => fields(value,{"header":boolean},{})],
  ["markdown_generic_contracts::table::TableData",value => fields(value,{},{})],
  ["markdown_trap_contracts::compat::BlankLineData",value => fields(value,{},{})],
  ["markdown_trap_contracts::reference::ReferenceData",value => fields(value,{"id":string,"label":string,"type":oneOf("user","group","channel")},{})],
  ["markdown_trap_contracts::spoiler::SpoilerData",value => fields(value,{},{})],
  ["markdown_trap_contracts::stamp::StampData",value => fields(value,{"literal":string},{})],
])
export const nodes = new Map(validators)
// Check this payload only; children can still contain unknown nodes.
export function isKnownNode(node) {
  return validators.get(node.kind)?.(node.data) ?? false
}
