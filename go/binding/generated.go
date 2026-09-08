// Code generated from the Rust catalog. DO NOT EDIT.
package binding

import (
	_ "embed"
	"github.com/traPtitech/traq-markdown-parser/go/ast"
	"github.com/traPtitech/traq-markdown-parser/go/extensions/commonmark"
	"github.com/traPtitech/traq-markdown-parser/go/extensions/generic"
	"github.com/traPtitech/traq-markdown-parser/go/extensions/trap"
)

//go:embed catalog.json
var CatalogJSON []byte

func Registry() ast.Registry {
	all := ast.Registry{}
	for name, decoder := range commonmark.Registry() {
		all[name] = decoder
	}
	for name, decoder := range generic.Registry() {
		all[name] = decoder
	}
	for name, decoder := range trap.Registry() {
		all[name] = decoder
	}
	return all
}
