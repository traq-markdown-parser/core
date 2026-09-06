package ast_test

import (
	"github.com/traPtitech/traq-markdown-parser/go/ast"
	"github.com/traPtitech/traq-markdown-parser/go/binding"
	"strings"
	"testing"
)

func TestStrictCommonAndExtensionContracts(t *testing.T) {
	valid := `{"source":"日本","children":[{"kind":"extension","span":{"start":0,"end":6},"name":"trap/reference@1","data":{"type":"user","id":"u","label":"@u"}}]}`
	if _, err := ast.Decode([]byte(valid), binding.Registry()); err != nil {
		t.Fatal(err)
	}
	for _, raw := range []string{
		strings.Replace(valid, `"id":"u"`, `"id":null`, 1),
		strings.Replace(valid, `"id":"u"`, `"id":3`, 1),
		strings.Replace(valid, `"label":"@u"`, `"unexpected":"@u"`, 1),
		strings.Replace(valid, `"start":0`, `"start":1`, 1),
		strings.Replace(valid, `"end":6`, `"end":7`, 1),
		strings.Replace(valid, `"kind":"extension"`, `"kind":"extension","value":"extra"`, 1),
		strings.Replace(valid, `"kind":"extension"`, `"kind":"extension","children":null`, 1),
		strings.Replace(valid, `trap/reference@1`, `unknown@1`, 1),
		valid + `{}`,
	} {
		if _, err := ast.Decode([]byte(raw), binding.Registry()); err == nil {
			t.Fatal("invalid contract was accepted")
		}
	}
}
