// Code generated from Rust extension payload types. DO NOT EDIT.
package generic

import (
	"encoding/json"
	"github.com/traPtitech/traq-markdown-parser/go/ast"
)

const RowName = "generic/table_row@1"

type Row struct {
	Header bool `json:"header"`
}

func decodeRow(raw json.RawMessage) (any, error) {
	var value Row
	if err := ast.DecodeFields(raw, &value, []string{"header"}, nil, nil); err != nil {
		return nil, err
	}

	return value, nil
}
