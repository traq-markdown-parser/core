// Code generated from Rust extension payload types. DO NOT EDIT.
package generic

import (
	"encoding/json"
	"github.com/traPtitech/traq-markdown-parser/go/ast"
)

const InlineMathName = "generic/math_inline@1"

type InlineMath struct {
	Tex string `json:"tex"`
}

func decodeInlineMath(raw json.RawMessage) (any, error) {
	var value InlineMath
	if err := ast.DecodeFields(raw, &value, []string{"tex"}, nil, nil); err != nil {
		return nil, err
	}

	return value, nil
}
