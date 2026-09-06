// Code generated from Rust extension payload types. DO NOT EDIT.
package generic

import (
	"encoding/json"
	"github.com/traPtitech/traq-markdown-parser/go/ast"
)

const StrikethroughName = "generic/strikethrough@1"

type Strikethrough struct {
}

func decodeStrikethrough(raw json.RawMessage) (any, error) {
	var value Strikethrough
	if err := ast.DecodeFields(raw, &value, nil, nil, nil); err != nil {
		return nil, err
	}

	return value, nil
}
