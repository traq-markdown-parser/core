// Code generated from Rust extension payload types. DO NOT EDIT.
package trap

import "github.com/traPtitech/traq-markdown-parser/go/ast"

func Registry() ast.Registry {
	return ast.Registry{
		BlankLineName: decodeBlankLine,
		ReferenceName: decodeReference,
		SpoilerName:   decodeSpoiler,
		StampName:     decodeStamp,
	}
}
