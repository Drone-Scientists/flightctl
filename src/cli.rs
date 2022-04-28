fn test() {
    Command::new(env!("CARGO_CRATE_NAME"))
        .about("Multi Vehicle flight controller")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .author("Daniel Lee")

        .subcommand(
            Command::new("generate")
                .subcommand_required(true)
                .arg_required_else_help(true)
                .short_flag('g')
                .long_flag("generate")
                .about()
                .subcommand(
                    Command::new("circle")
                        .about("Create circular shape")
                        .arg_required_else_help(true)
                        .arg(
                            Arg::new("count")
                                .short('c')
                                .takes_value(true)
                                .required(true)
                                .help("Number of vehicles used to create the shape"),
                        ),
                )
                .subcommand(
                    Command::new("square")
                        .about("Create square shape")
                        .arg_required_else_help(true)
                        .arg(
                            Arg::new("count")
                                .short('c')
                                .takes_value(true)
                                .required(true)
                                .help("Number of vehicles used to create the shape"),
                        ),
                )
                .subcommand(
                    Command::new("line")
                        .about("Create line shape")
                        .arg_required_else_help(true)
                        .arg(
                            Arg::new("count")
                                .short('c')
                                .takes_value(true)
                                .required(true)
                                .help("Number of vehicles used to create the shape"),
                        ),
                ),
        )
        .subcommand(
            Command::new("echo")
                .short_flag('e')
                .long_flag("echo")
                .about("Sanity check cli parser"),
        );
}
