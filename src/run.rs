use anyhow::{bail, Result};
use ratatui::crossterm::style::{style, Stylize};
use std::io::{self, Write};

use crate::{
    app_state::{AppState, ExercisesProgress},
    exercise::{RunnableExercise, OUTPUT_CAPACITY},
    terminal_link::TerminalFileLink,
};

pub fn run(app_state: &mut AppState) -> Result<()> {
    let exercise = app_state.current_exercise();
    let mut output = Vec::with_capacity(OUTPUT_CAPACITY);
    let success = exercise.run_exercise(Some(&mut output), app_state.cmd_runner())?;

    let mut stdout = io::stdout().lock();
    stdout.write_all(&output)?;

    if !success {
        app_state.set_pending(app_state.current_exercise_ind())?;

        bail!(
            "Ran {} with errors",
            app_state.current_exercise().terminal_link(),
        );
    }

    writeln!(
        stdout,
        "{}{}",
        "✓ Successfully ran ".green(),
        exercise.path.green(),
    )?;

    if let Some(solution_path) = app_state.current_solution_path()? {
        writeln!(
            stdout,
            "\n{} for comparison: {}\n",
            "Solution".bold(),
            style(TerminalFileLink(&solution_path)).underlined().cyan(),
        )?;
    }

    match app_state.done_current_exercise(&mut stdout)? {
        ExercisesProgress::AllDone => (),
        ExercisesProgress::CurrentPending | ExercisesProgress::NewPending => writeln!(
            stdout,
            "Next exercise: {}",
            app_state.current_exercise().terminal_link(),
        )?,
    }

    Ok(())
}
