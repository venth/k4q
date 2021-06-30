package commands

import (
	"github.com/urfave/cli/v2"
)

func newQuery() *cli.Command {
	return &cli.Command{
		Name:  "query",
		Usage: "Performs a query against given topic using a filter specified by a user",

		Subcommands: registeredQuery(),
	}
}

func registeredQuery() []*cli.Command {
	return []*cli.Command{
		newQueryKey(),
	}
}

func newQueryKey() *cli.Command {
	return &cli.Command{
		Name:     "key",
		Usage:    "queries for the specific key",
		Category: "Query",
		Subcommands: []*cli.Command{
			{
				Category:  "Query operation",
				Name:      "eq",
				ArgsUsage: "<value> <topic1> .. [topicN]",
			},
		},
	}
}
