// Code generated from Rust extension payload types. DO NOT EDIT.
package generic

import (
	"encoding/json"
	"github.com/traPtitech/traq-markdown-parser/go/ast"
)

const MarkName = "generic/mark@1"

type Mark struct {
}

func decodeMark(raw json.RawMessage) (any, error) {
	var value Mark
	if err := ast.DecodeFields(raw, &value, nil, nil, nil); err != nil {
		return nil, err
	}

	return value, nil
}
