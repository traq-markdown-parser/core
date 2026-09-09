package binding

import (
	"context"
	"errors"
	"testing"
)

func TestCanceledWaitDoesNotTakeInstance(t *testing.T) {
	instance := &Instance{gate: make(chan struct{}, 1)}
	instance.gate <- struct{}{}
	ctx, cancel := context.WithCancel(context.Background())
	done := make(chan error, 1)
	go func() { _, err := instance.Call(ctx, "parse", "waiting"); done <- err }()
	cancel()
	if err := <-done; !errors.Is(err, context.Canceled) {
		t.Fatal(err)
	}
	if len(instance.gate) != 1 {
		t.Fatal("canceled waiter changed the instance gate")
	}
}
