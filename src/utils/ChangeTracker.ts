import { invoke } from "@tauri-apps/api/core";
import { Model } from "../models/Model";

/**
 * Tracks changes in edit modals and handles all (add, undo, commit).
 * @template T - The model type to change.
 */
export class ChangeTracker<T extends Model> {

	/**
	 * The shadow of the edited object. Stores the original version of the edited model.
	 * @type {T}
	 */
	shadow: T;

	/**
	 * The changes to be made to the model.
	 * @type {Change[]}
	 */
	changes: Change[];

	constructor(shadow: T) {
		this.shadow = shadow;
		this.changes = [];
	}

	/**
	 * Commits the changes to the Rust backend to be saved.
	 *
	 * @async
	 * @returns {Promise<Boolean>} true, if the changes were applied successfully, otherwise false.
	 */
	async commit(): Promise<Boolean> {
		// Commit changes to backend;
		if (this.changes.length == 0) return false;

		await invoke("commit_edit_changes", {
			changes: this.changes,
			shadow: this.shadow,
		})
			.then(() => { return true })

		return false;
	}

	/**
	 * Adds a new change to the list.
	 *
	 * @param {Change} change - The change to add.
	 */
	add(change: Change) {
		this.changes.push(change);
	}

	/**
	 * Undoes a specific change given the Change object itself.
	 *
	 * @param {Change} change - The change to undo.
	 * @throws {Error} - If the change to undo is not found in the list of changes.
	 */
	undo(change: Change) {
		const changeIdx = this.changes.indexOf(change);
		if (changeIdx == -1) {
			throw new Error("Error in change state, item to undo not found in changes.");
		}
		this.changes.splice(changeIdx, 1);
	}

	/**
	 * Undoes the latest change.
	 */
	undo_last() {
		this.changes.splice(this.changes.length - 1, 1);
	}

	/**
	 * Clears all changes, therefore undoing everything.
	 */
	undo_all() {
		this.changes = [];
	}
}

/**
 * Defines a change in a edit modal.
 */
export class Change {

	/**
	 * The path for the change to take in backend in order to be processed.
	 * @type {string}
	 */
	changePath: string;

	/**
	 * The field to be changed, if it is a field change.
	 * @type {string}
	 */
	field?: string;

	/**
	 * The old value, which is going to be changed, if it's a value change.
	 * @type {string}
	 */
	oldValue?: string;

	/**
	 * The new value, which the old value is going to be changed to, if it's a value change.
	 * @type {string}
	 */
	newValue?: string;

	constructor(changePath: string) {
		this.changePath = changePath;
	}

	/**
	 * Stores the values for a value change.
	 *
	 * @param {string} field - The field to be changed.
	 * @param {string} oldValue - The old value of that field.
	 * @param {string} newValue - The new value the field is changed to.
	 */
	valueChange(field: string, oldValue: string, newValue: string) {
		this.field = field;
		this.oldValue = oldValue;
		this.newValue = newValue;
	}
}
