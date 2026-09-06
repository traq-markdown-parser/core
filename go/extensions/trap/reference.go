// Code generated from Rust extension payload types. DO NOT EDIT.
package trap

import (
	"encoding/json"
	"fmt"
	"github.com/traPtitech/traq-markdown-parser/go/ast"
)

const ReferenceName = "trap/reference@1"

type Reference struct {
	ID    string `json:"id"`
	Label string `json:"label"`
	Type  string `json:"type"`
}

func decodeReference(raw json.RawMessage) (any, error) {
	var value Reference
	if err := ast.DecodeFields(raw, &value, []string{"id", "label", "type"}, nil, nil); err != nil {
		return nil, err
	}
	if value.Type != "user" && value.Type != "group" && value.Type != "channel" {
		return nil, fmt.Errorf("invalid type")
	}
	return value, nil
}
