export class User {
	private readonly username: string;

	private readonly email: string;

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
}
