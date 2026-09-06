// Code generated from Rust extension payload types. DO NOT EDIT.
package trap

import (
	"encoding/json"
	"github.com/traPtitech/traq-markdown-parser/go/ast"
)

const StampName = "trap/stamp@1"

type Stamp struct {
	Literal string `json:"literal"`
}

func decodeStamp(raw json.RawMessage) (any, error) {
	var value Stamp
	if err := ast.DecodeFields(raw, &value, []string{"literal"}, nil, nil); err != nil {
		return nil, err
	}

	return value, nil
}
