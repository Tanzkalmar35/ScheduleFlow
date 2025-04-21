import { invoke } from "@tauri-apps/api/core";

export class ChangeTracker<T implements Model> {
	shadow: T;
	changes: Change[];

	constructor(shadow: T) {
		this.shadow = shadow;
		this.changes = [];
	}

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

	add(change: Change) {
		this.changes.push(change);
	}

	undo(change: Change) {
		const changeIdx = this.changes.indexOf(change);
		if (changeIdx == -1) {
			throw new Error("Error in change state, item to undo not found in changes.");
		}
		this.changes.splice(changeIdx, 1);
	}

	undo_last() {
		this.changes.splice(this.changes.length - 1, 1);
	}

	undo_all() {
		this.changes = [];
	}
}

export class Change {
	changePath: string;
	oldValue?: string;
	newValue?: string;

	constructor(changePath: string) {
		this.changePath = changePath;
	}

	change(oldValue: string, newValue: string) {
		this.oldValue = oldValue;
		this.newValue = newValue;
	}
}
