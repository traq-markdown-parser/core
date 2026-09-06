// Code generated from Rust extension payload types. DO NOT EDIT.
package generic

import "github.com/traPtitech/traq-markdown-parser/go/ast"

func Registry() ast.Registry {
	return ast.Registry{
		MarkName:          decodeMark,
		BlockMathName:     decodeBlockMath,
		InlineMathName:    decodeInlineMath,
		StrikethroughName: decodeStrikethrough,
		TableName:         decodeTable,
		CellName:          decodeCell,
		RowName:           decodeRow,
	}
}
