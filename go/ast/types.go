// Package ast decodes the common tree independently of product extensions.
package ast

import "encoding/json"

type Document struct {
	Source   string `json:"source"`
	Children []Node `json:"children"`
}

type Span struct {
	Start uint32 `json:"start"`
	End   uint32 `json:"end"`
}

// Extension payloads are decoded by the registry, never by a product switch here.
type Node struct {
	Kind        string          `json:"kind"`
	Span        Span            `json:"span"`
	Children    []Node          `json:"children,omitempty"`
	Name        string          `json:"name,omitempty"`
	Data        json.RawMessage `json:"data,omitempty"`
	Payload     any             `json:"-"`
	Value       string          `json:"value,omitempty"`
	Literal     string          `json:"literal,omitempty"`
	Destination string          `json:"destination,omitempty"`
	Title       *string         `json:"title,omitempty"`
	Form        string          `json:"form,omitempty"`
	LabelSource string          `json:"label_source,omitempty"`
	Level       uint8           `json:"level,omitempty"`
	Ordered     bool            `json:"ordered,omitempty"`
	Start       uint32          `json:"start,omitempty"`
	Tight       bool            `json:"tight,omitempty"`
	Marker      string          `json:"marker,omitempty"`
	Fenced      bool            `json:"fenced,omitempty"`
	Info        string          `json:"info,omitempty"`
}

type Decoder func(json.RawMessage) (any, error)
type Registry map[string]Decoder
