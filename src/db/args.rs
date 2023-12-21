#[derive(Debug)]
pub struct CalendarCommand {
    pub command: CalendarSubCommand,
}

/// Used to delete an entity from the database based on the id.
pub struct DeleteEntity {
    pub id: i32,
}

/// Defines all subcommands available for the calendar table.
pub enum CalendarSubCommand {
    Create(CreateCalendar),
    Update(UpdateCalendar),
    Delete(DeleteEntity),
    Show,
}

/// Used to create a calendar entry.
pub struct CreateCalendar {
    pub title: String,
}

/// Used to update a calendar entry.
pub struct UpdateCalendar {
    pub id: i32,
    pub title: String,
}

/// Defines the command to view a calendar.
#[derive(Debug)]
pub struct ViewCommand {
    #[clap(subcommand)]
    pub command: ViewSubcommand,
}

/// Defines the subcommand for viewing a calendar.
#[derive(Debug)]
pub enum ViewSubcommand {
    /// Create a new view.
    Create(CreateView),

    /// Show all views with id numbers for users and calendars.
    Show,

    /// Show all views with names for users and calendars.
    ShowPretty,
}

/// Does the actual view creation.
#[derive(Debug)]
pub struct CreateView {}
