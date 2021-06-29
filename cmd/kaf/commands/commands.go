package commands

import "github.com/urfave/cli/v2"

func Registered() []*cli.Command {
	return []*cli.Command{
		newQuery(),
	}
}
