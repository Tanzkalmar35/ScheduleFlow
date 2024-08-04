// The representation of rust's icalendar::Calendar on the frontend.
class Calendar {

    // A list of key-value pairs - properties that describe the calendar.
    properties;

    // A list of components (like Events, Todos, ...),
    // that are essentially just lists of Properties too
    components;

    constructor(properties, components) {
        this.properties = properties;
        this.components = components;
    }

    getProperties() {
        return this.properties;
    }

    getComponents() {
        return this.components;
    }
}
