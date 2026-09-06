package core

import "slices"

// Names are optional diagnostics. Symbols, rather than names, identify definitions.
type symbol struct{ reserved byte }
type PluginGroup struct {
	named    bool
	identity *symbol
	parent   *PluginGroup
	name     string
}

func NewPluginGroup() *PluginGroup { return &PluginGroup{identity: new(symbol)} }
func (g *PluginGroup) Named(name string) *PluginGroup {
	copy := *g
	copy.name = name
	copy.named = true
	return &copy
}
func (g *PluginGroup) Name() string         { return g.name }
func (g *PluginGroup) Parent() *PluginGroup { return g.parent }
func (g *PluginGroup) Group() *PluginGroup  { child := NewPluginGroup(); child.parent = g; return child }
func (g *PluginGroup) New() *Plugin         { p := NewPlugin(); p.group = g; return p }

type Rule struct {
	runtime    *Runtime
	index      int
	name       string
	phase      string
	extensions []string
}

func (r *Rule) Name() string  { return r.name }
func (r *Rule) Phase() string { return r.phase }

type Plugin struct {
	named    bool
	identity *symbol
	name     string
	group    *PluginGroup
	rules    []*Rule
	frozen   bool
}

func NewPlugin() *Plugin { return &Plugin{identity: new(symbol)} }
func (p *Plugin) Named(name string) *Plugin {
	copy := *p
	copy.name = name
	copy.named = true
	copy.frozen = false
	return &copy
}
func (p *Plugin) Name() string            { return p.name }
func (p *Plugin) Namespace() *PluginGroup { return p.group }
func (p *Plugin) Add(rule *Rule) error {
	if p.frozen {
		return &BuildError{Code: "invalid_definition", Reason: "bundled plugins are immutable"}
	}
	if rule == nil {
		return &BuildError{Code: "invalid_definition", Reason: "nil rule"}
	}
	p.rules = append(slices.Clone(p.rules), rule)
	p.identity = new(symbol)
	return nil
}
func (p *Plugin) phaseRules(phase string) []*Rule {
	var result []*Rule
	for _, rule := range p.rules {
		if rule.phase == phase {
			result = append(result, rule)
		}
	}
	return result
}
func (p *Plugin) InlineRules() []*Rule { return p.phaseRules("inline") }
func (p *Plugin) BlockRules() []*Rule  { return p.phaseRules("block") }
func (p *Plugin) TextRules() []*Rule   { return p.phaseRules("text") }
