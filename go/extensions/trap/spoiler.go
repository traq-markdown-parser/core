// Code generated from Rust extension payload types. DO NOT EDIT.
package trap

import (
	"encoding/json"
	"github.com/traPtitech/traq-markdown-parser/go/ast"
)

const SpoilerName = "trap/spoiler@1"

type Spoiler struct {
}

func decodeSpoiler(raw json.RawMessage) (any, error) {
	var value Spoiler
	if err := ast.DecodeFields(raw, &value, nil, nil, nil); err != nil {
		return nil, err
	}

	return value, nil
}
