import { Model, ShadowType } from "./Model";

export class User implements Model {
	private readonly username: string;

	private readonly email: string;

	readonly shadowType: ShadowType = ShadowType.User;

	constructor(username: string, email: string) {
		this.username = username;
		this.email = email;
	}

	getUsername(): string {
		return this.username;
	}

	getEmail(): string {
		return this.email;
	}

	getShadowType(): ShadowType {
		return this.shadowType;
	}
}
