// Generated from Rust extension payload types. Do not edit.
import {fields,string,boolean,nullable,oneOf} from '../fields.mjs'
export const names = Object.freeze({"Mark":"generic/mark@1","BlockMath":"generic/math_block@1","InlineMath":"generic/math_inline@1","Strikethrough":"generic/strikethrough@1","Table":"generic/table@1","Cell":"generic/table_cell@1","Row":"generic/table_row@1","BlankLine":"trap/blank_line@1","Reference":"trap/reference@1","Spoiler":"trap/spoiler@1","Stamp":"trap/stamp@1"})
export const extensions = new Map([
  ["generic/mark@1",value => fields(value,{},{})],
  ["generic/math_block@1",value => fields(value,{"tex":string},{})],
  ["generic/math_inline@1",value => fields(value,{"tex":string},{})],
  ["generic/strikethrough@1",value => fields(value,{},{})],
  ["generic/table@1",value => fields(value,{},{})],
  ["generic/table_cell@1",value => fields(value,{"alignment":nullable(oneOf("left","center","right"))},{})],
  ["generic/table_row@1",value => fields(value,{"header":boolean},{})],
  ["trap/blank_line@1",value => fields(value,{},{})],
  ["trap/reference@1",value => fields(value,{"id":string,"label":string,"type":oneOf("user","group","channel")},{})],
  ["trap/spoiler@1",value => fields(value,{},{})],
  ["trap/stamp@1",value => fields(value,{"literal":string},{})],
])
