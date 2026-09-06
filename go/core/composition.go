package core

type groupDefinition struct {
	Parent *int    `json:"parent"`
	Name   *string `json:"name"`
}
type pluginDefinition struct {
	Group *int    `json:"group"`
	Name  *string `json:"name"`
	Rules []int   `json:"rules"`
}
type composition struct {
	Groups  []groupDefinition  `json:"groups"`
	Plugins []pluginDefinition `json:"plugins"`
	Order   []int              `json:"order"`
}

func optionalName(name string, named bool) *string {
	if !named {
		return nil
	}
	return &name
}
func (d definition) composition() composition {
	result := composition{Groups: []groupDefinition{}, Plugins: []pluginDefinition{}, Order: append([]int{}, d.order...)}
	indices := map[*symbol]int{}
	var visit func(*PluginGroup) *int
	visit = func(group *PluginGroup) *int {
		if group == nil {
			return nil
		}
		if index, exists := indices[group.identity]; exists {
			return &index
		}
		parent := visit(group.parent)
		index := len(result.Groups)
		result.Groups = append(result.Groups, groupDefinition{parent, optionalName(group.name, group.named)})
		indices[group.identity] = index
		return &index
	}
	for _, plugin := range d.plugins {
		rules := []int{}
		for _, rule := range plugin.rules {
			rules = append(rules, rule.index)
		}
		result.Plugins = append(result.Plugins, pluginDefinition{visit(plugin.group), optionalName(plugin.name, plugin.named), rules})
	}
	return result
}
