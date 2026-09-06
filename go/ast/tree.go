package ast

import (
	"encoding/json"
	"fmt"
	"slices"
	"strconv"
)

// The wire JSON is decoded once. Tree validation never reparses a parent subtree.
func objectFields(value any, required, optional, nullable []string) (map[string]any, error) {
	fields, ok := value.(map[string]any)
	if !ok {
		return nil, fmt.Errorf("expected object")
	}
	for _, name := range required {
		if _, ok := fields[name]; !ok {
			return nil, fmt.Errorf("missing field %s", name)
		}
	}
	for name, value := range fields {
		if !slices.Contains(required, name) && !slices.Contains(optional, name) {
			return nil, fmt.Errorf("unknown field %s", name)
		}
		if value == nil && !slices.Contains(nullable, name) {
			return nil, fmt.Errorf("null field %s", name)
		}
	}
	return fields, nil
}

func number(value any) (uint32, error) {
	n, ok := value.(json.Number)
	if !ok {
		return 0, fmt.Errorf("expected unsigned integer")
	}
	u, err := strconv.ParseUint(string(n), 10, 32)
	return uint32(u), err
}

func readNode(value any) (Node, []any, error) {
	var node Node
	fields, ok := value.(map[string]any)
	if !ok {
		return node, nil, fmt.Errorf("expected node object")
	}
	kind, ok := fields["kind"].(string)
	if !ok {
		return node, nil, fmt.Errorf("missing node kind")
	}
	shape, ok := nodeFields[kind]
	if !ok {
		return node, nil, fmt.Errorf("unknown node kind %s", kind)
	}
	if _, err := objectFields(fields, append([]string{"kind", "span"}, shape...), []string{"children"}, []string{"title"}); err != nil {
		return node, nil, err
	}
	node.Kind = kind
	span, err := objectFields(fields["span"], []string{"start", "end"}, nil, nil)
	if err != nil {
		return node, nil, err
	}
	if node.Span.Start, err = number(span["start"]); err != nil {
		return node, nil, err
	}
	if node.Span.End, err = number(span["end"]); err != nil {
		return node, nil, err
	}
	for _, field := range []struct {
		name   string
		target *string
	}{
		{"value", &node.Value}, {"literal", &node.Literal}, {"destination", &node.Destination},
		{"form", &node.Form}, {"label_source", &node.LabelSource}, {"marker", &node.Marker},
		{"info", &node.Info}, {"name", &node.Name},
	} {
		if value, present := fields[field.name]; present {
			text, ok := value.(string)
			if !ok {
				return node, nil, fmt.Errorf("invalid string %s", field.name)
			}
			*field.target = text
		}
	}
	for _, field := range []struct {
		name   string
		target *bool
	}{{"ordered", &node.Ordered}, {"tight", &node.Tight}, {"fenced", &node.Fenced}} {
		if value, present := fields[field.name]; present {
			flag, ok := value.(bool)
			if !ok {
				return node, nil, fmt.Errorf("invalid boolean %s", field.name)
			}
			*field.target = flag
		}
	}
	if value, present := fields["level"]; present {
		level, err := number(value)
		if err != nil || level < 1 || level > 6 {
			return node, nil, fmt.Errorf("invalid heading level")
		}
		node.Level = uint8(level)
	}
	if value, present := fields["start"]; present {
		if node.Start, err = number(value); err != nil {
			return node, nil, err
		}
	}
	if value := fields["title"]; value != nil {
		title, ok := value.(string)
		if !ok {
			return node, nil, fmt.Errorf("invalid title")
		}
		node.Title = &title
	}
	if node.Kind == "link" && !slices.Contains([]string{"explicit", "autolink", "linkify"}, node.Form) {
		return node, nil, fmt.Errorf("invalid link form")
	}
	if node.Kind == "extension" {
		if _, ok := fields["data"].(map[string]any); !ok {
			return node, nil, fmt.Errorf("expected extension object")
		}
		node.Data, err = json.Marshal(fields["data"])
		if err != nil {
			return node, nil, err
		}
	}
	var children []any
	if value, present := fields["children"]; present {
		children, ok = value.([]any)
		if !ok {
			return node, nil, fmt.Errorf("invalid children")
		}
	}
	if len(children) > 0 && !slices.Contains([]string{"paragraph", "heading", "blockquote", "list", "list_item", "emphasis", "strong", "link", "image", "extension"}, node.Kind) {
		return node, nil, fmt.Errorf("unexpected children")
	}
	return node, children, nil
}
