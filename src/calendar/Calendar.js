/**
 *  The frontend representation of the calendar in simple manner.
 *  
 *  This means, that this object is only a data holder, 
 *  it does not offer any of the functionality the backend offers.
 */
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
     *  Map Calendar from the backend to this Calendar,
     *  which we can use in the frontend to render the calendar.
     *
     *  @param {object} calendar an object containing the backend's calendar data.
     */
    static map(calendar) {
        let result;
        let components = [];
        let properties = [];

        // Map all components
        for (let component in calendar.components) {
            let type;

            console.log("Found component " + component);

            switch (component.type) {
                case "Event": type = ComponentType.Event;
                case "Todo": type = ComponentType.Todo;
                case "Venue": type = ComponentType.Venue;
                case "Other": type = ComponentType.Other;
            }

            components.push(new Component(component.properties, type));
        }

        console.log("Resulting in component list: " + components);

        // Map all properties
        for (let property in calendar.properties) {
            properties.push(property);
            console.log("Found Property: " + property);
        }

        console.log("Resulting in property list: " + properties)

        result = new Calendar(components, properties);

        console.log("Resulting in calendar: " + calendar);

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
    Todo: '',
    Venue: '',
    Other: ''
}

Object.freeze(ComponentType);
