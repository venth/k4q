package main

import (
	"log"
	"os"

	"github.com/urfave/cli/v2"
	"github.com/venth/kaf/cmd/kaf/commands"
)

func main() {
	app := &cli.App{
		Name: "kaf",
		Action: cli.ShowAppHelp,
		Commands: commands.Registered(),
	}

	err := app.Run(os.Args)
	if err != nil {
		log.Fatal(err)
	}
}
