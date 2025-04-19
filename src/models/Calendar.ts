import { User } from "./User";

/**
 *  The frontend representation of the calendar in simple manner.
 *
 *  This means, that this object is only a data holder,
 *  it does not offer any of the functionality the backend offers.
 */
export class Calendar {
	/**
	 * The title or alias assigned to a calendar to identify it in the gui.
	 *
	 *  @type {string}
	 */
	private readonly name: string;

	/**
	 *   A list of components that belong to this calendar.
	 *   Each component is essentially just a list of properties too.
	 *
	 *   @type {Component[]}
	 */
	private readonly components: Component[];

	/**
	 *   A list of properties describing the calendar's characteristics.
	 *
	 *   @type {Map<string, string>}
	 */
	private readonly properties: Map<string, string>;

	/**
	 * A list of users that have access to this calendar.
	 * @type {User[]}
	 */
	private readonly users: User[];

	/**
	 *   Initializes a new Calendar object.
	 *
	 *   @param name the name of the calendar.
	 *   @param {Component[]} components belong to this calendar.
	 *   @param {Map<string, string>} properties describe the calendar's characteristics.
	 *   @param {User[]} users - a list of users with access to this calendar.
	 */
	constructor(name: string, components: Component[], properties: Map<string, string>, users: User[]) {
		this.name = name;
		this.components = components;
		this.properties = properties;
		this.users = users;
	}

	/**
	 * Returns the name of this calendar.
	 *
	 * @returns {string} The name if this calendar.
	 */
	getName(): string {
		return this.name;
	}

	/**
	 * Returns a list of components that this calendar includes.
	 *
	 * @returns {Component[]} all of this calendars components.
	 */
	getComponents(): Component[] {
		return this.components;
	}

	/**
	 * Returns a map of all properties defining the calendar.
	 *
	 * @returns {Map<string, string>} all properties this calendar has assigned.
	 */
	getProperties(): Map<string, string> {
		return this.properties;
	}

	/**
	 * Returns a list of users that have access to this calendar.
	 *
	 * @returns {User[]} A list of users that have access to this calendar.
	 */
	getUsers(): User[] {
		return this.users;
	}

	/**
	 *  Map Calendar from the backend to this Calendar,
	 *  which we can use in the frontend to render the calendar.
	 *
	 *  @param {Object[]} calendars an object containing the backend's calendar data.
	 */
	static map(calendars: any[]) {
		let result = [];
		let components = [];
		let i = 0;
		const amount_of_calendars = calendars.length;

		// Loop calendars
		while (i < amount_of_calendars) {
			const calendar = calendars[i];
			const amountOfComponents = calendar.components.length;
			let j = 0;

			// Loop components
			while (j < amountOfComponents) {
				const component = calendar.components[j];

				let type;
				switch (component.c_type) {
					case "EVENT":
						type = ComponentType.Event;
						break;
					case "TODO":
						type = ComponentType.Todo;
						break;
					case "VENUE":
						type = ComponentType.Venue;
						break;
					default:
						type = ComponentType.Other;
						break;
				}

				// Loop all properties of the component
				const amountOfProperties = component.properties.length;
				const propertyMap = new Map();
				let k = 0;

				// Loop properties of component to map them to Map<string, string>
				while (k < amountOfProperties) {
					const property = component.properties[k];
					propertyMap.set(property.key, property.val);
					k++;
				}
				components.push(new Component(propertyMap, type));
				j++;
			}

			const calendarPropertyMap = new Map();
			let l = 0;
			while (l < calendar.properties.length) {
				const property = calendar.properties[l];
				calendarPropertyMap.set(property.key, property.val);
				l++;
			}

			let m = 0;
			let users: User[] = [];
			while (m < calendar.users.length) {
				let user = calendar.users[m];
				users.push(new User(user.username, user.email))
				m++;
			}

			result.push(
				new Calendar(calendar.name, components, calendarPropertyMap, users),
			);

			i++;
		}

		return result;
	}
}

class Component {
	/**
	 *   A list of properties describing the component's characteristics.
	 *
	 *   @type {Map<string, string>}
	 */
	private readonly properties: Map<string, string>;

	/**
	 *   The type of Component
	 *
	 *   @type {ComponentType}
	 */
	private readonly type: ComponentType;

	/**
	 *   Initializes a new Component.
	 *
	 *   @param {Map<string, string>} properties describe the characteristics of the component.
	 *   @param {ComponentType} type defines the type of Component.
	 */
	constructor(properties: Map<string, string>, type: ComponentType) {
		this.properties = properties;
		this.type = type;
	}

	/**
	 * Returns a map of the properties of this Comoponent.
	 *
	 * @returns {Map<string, string>} all properties of this component.
	 */
	getProperties(): Map<string, string> {
		return this.properties;
	}

	/**
	 * Returns the type of component this object is.
	 *
	 * @returns {ComponentType} The type of comoponent this object is.
	 */
	getType(): ComponentType {
		return this.type;
	}
}

export enum ComponentType {
	Event = "#111111",
	Todo = "",
	Venue = "",
	Other = "",
};
