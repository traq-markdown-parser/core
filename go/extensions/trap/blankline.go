// Code generated from Rust extension payload types. DO NOT EDIT.
package trap

import (
	"encoding/json"
	"github.com/traPtitech/traq-markdown-parser/go/ast"
)

const BlankLineName = "trap/blank_line@1"

type BlankLine struct {
}

func decodeBlankLine(raw json.RawMessage) (any, error) {
	var value BlankLine
	if err := ast.DecodeFields(raw, &value, nil, nil, nil); err != nil {
		return nil, err
	}

	return value, nil
}
