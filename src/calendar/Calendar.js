class Calendar {

    /**
    *   A list of components that belong to this calendar.
    *   Each component is essentially just a list of properties too.
    *
    *   @type {Component[][]}
    */
    components;

    /**
    *   A list of properties describing the calendar's characteristics.
    *
    *   @type {string[]}
    */
    properties;

    /**
    *   Initializes a new Calendar object.
    *
    *   @param {Component[][]} components belong to this calendar.
    *   @param {string[]} properties describe the calendar's characteristics.
    */
    constructor(components, properties) {
        this.components = components;
        this.properties = properties;
    }

    /**
     *  Map icalendar::Calendar from the backend to this Calendar,
     *  which we can use in the frontend to render the calendar.
     *
     *  @param {object} iCalendar an object containing the icalendar's data.
     */
    map(iCalendar) {
        let result;
        let components = [];
        let properties = [];

        // Map all components
        for (let iComponent in iCalendar.components) {
            let type;

            switch (iComponent.type) {
                case "Event": type = ComponentType.Event;
                // TODO: Handle all types of components.
            }

            let component = new Component(iComponent.properties, type);
            components.push(component);
        }

        // Map all properties
        for (let iProperty in iCalendar.properties) {
            properties.push(iProperty);
        }

        result = new Calendar(components, properties);

        return result;
    }
}

class Component {

    /**
    *   A list of properties describing the component's characteristics.
    *
    *   @type {string[]}
    */
    properties;

    /**
    *   The type of Component
    *
    *   @type {ComponentType}
    */
    type;

    /**
    *   Initializes a new Component.
    *
    *   @param {string[]} properties describe the characteristics of the component.
    *   @param {ComponentType} type defines the type of Component.
    */
    constructor(properties, type) {
        this.properties = properties;
        this.type = type;
    }
}

// A Javascript enum for component types, each name is a color in hex format assigned.
const ComponentType = {
    Event: '#111111',
}

Object.freeze(ComponentType);
